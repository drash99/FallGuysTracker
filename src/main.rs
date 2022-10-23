pub mod parsing;
pub mod game;

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use parsing::*;
use game::Reconstruct;
use std::fs::{File};
use std::cell::RefCell;
use std::time;

use nwd::NwgUi;
use nwg::NativeUi;

pub struct AppData {
    parser : Parser,
    pub game : Reconstruct,
}
impl AppData {
    pub fn new() -> AppData {
        let file = File::open("C:\\Users\\Leo\\AppData\\LocalLow\\Mediatonic\\FallGuys_client\\Player.log").unwrap();
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

impl Default for AppData{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (500, 1000), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text:"", size: (280, 1000), position: (10, 0))]
    text_box : nwg::Label,
    #[nwg_control(text:"", size: (200, 1000), position: (300, 0))]
    team_box : nwg::Label,

    #[nwg_control(interval: time::Duration::from_millis(100), active: true)]
    #[nwg_events(OnTimerTick: [BasicApp::update])]
    timer : nwg::AnimationTimer,

    data : RefCell<AppData>,
}

impl BasicApp {
    
    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }

    fn update(&self) {
        let mut data = self.data.borrow_mut();
        data.update();
        self.text_box.set_text(&format!("{}", data.game));
        self.team_box.set_text(&data.game.print_team());
    }   
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let data: RefCell<AppData> = RefCell::new(AppData::new());
    let _app = BasicApp::build_ui(BasicApp { data: data, ..Default::default() }).expect("Failed to build UI");

    println!("Hello, world!");
    nwg::dispatch_thread_events();
}
