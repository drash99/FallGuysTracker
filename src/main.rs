#![windows_subsystem = "windows"]
pub mod game;
pub mod parsing;
pub mod ui;

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use game::Reconstruct;
use parsing::*;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::time;
use ui::Overlay;

use nwd::NwgUi;
use nwg::NativeUi;

pub struct AppData {
    parser: Parser,
    pub game: Reconstruct,
}
impl AppData {
    pub fn new() -> AppData {
        let userpath = std::env::var("userprofile").expect("No APP_DATA directory");
        let path = format!(
            "{}\\AppData\\LocalLow\\Mediatonic\\FallGuys_client\\Player.log",
            userpath
        );
        let file = File::open(path).unwrap_or_else(|_| {
            nwg::simple_message(
                "Error",
                "log file not found! execute the game at least once before running this program",
            );
            panic!("log file not found! execute the game at least once before running this program")
        });
        let parser = Parser::new(file);
        AppData {
            parser,
            game: Reconstruct::new(),
        }
    }

    pub fn update(&mut self) {
        self.parser.parse();
        for parsed in &self.parser.parsed {
            self.game.push(parsed.clone());
        }
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (500, 1000), position: (300, 300), title: "Fallguys Tracker", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text:"", size: (280, 1000), position: (10, 0))]
    text_box: nwg::Label,
    #[nwg_control(text:"", size: (200, 700), position: (300, 0))]
    team_box: nwg::Label,

    #[nwg_control(text:"Overlay", size: (100, 30), position: (300, 700))]
    #[nwg_events(OnButtonClick: [BasicApp::overlay])]
    overlay_button: nwg::Button,

    #[nwg_control(interval: time::Duration::from_millis(500), active: true)]
    #[nwg_events(OnTimerTick: [BasicApp::update])]
    timer: nwg::AnimationTimer,

    data: Arc<Mutex<AppData>>,
}

impl BasicApp {
    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }

    fn update(&self) {
        let mut data = self.data.lock().expect("failed to get lock");
        data.update();
        self.text_box.set_text(&format!("{}", data.game));
        self.team_box.set_text(&data.game.print_team());
    }
    fn overlay(&self) {
        Overlay::popup(self.data.clone());
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let mut font = nwg::Font::default();
    let _ = nwg::Font::builder().size(14).build(&mut font);
    nwg::Font::set_global_default(Some(font)).expect("Failed to set default font");
    let data = Arc::new(Mutex::new(AppData::new()));
    let _app = BasicApp::build_ui(BasicApp {
        data,
        ..Default::default()
    })
    .expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
