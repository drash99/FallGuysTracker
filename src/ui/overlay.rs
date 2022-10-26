extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use crate::AppData;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct Overlay {
    #[nwg_control(size: (200, 520), position: (300, 300), title: "Overlay", flags: "WINDOW|VISIBLE", topmost: true)]
    #[nwg_events( OnWindowClose: [Overlay::say_goodbye] )]
    pub window: nwg::Window,

    #[nwg_control(text:"", size: (200, 70), position: (0, 0))]
    pub infos: nwg::Label,

    #[nwg_control(text:"", size: (200, 450), position: (0, 70))]
    pub team_box: nwg::Label,

    #[nwg_control(interval: time::Duration::from_millis(100), active: true)]
    #[nwg_events(OnTimerTick: [Overlay::update])]
    pub timer: nwg::AnimationTimer,

    pub data: Arc<Mutex<AppData>>,
}

impl Overlay {
    pub fn popup(data: Arc<Mutex<AppData>>) -> thread::JoinHandle<()> {
        let data = data.clone();
        thread::spawn(move || {
            let _app = Overlay::build_ui(Overlay {
                data: data,
                ..Default::default()
            })
            .expect("failed to build overlay");
            nwg::dispatch_thread_events();
        })
    }
    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }

    fn update(&self) {
        let mut data = self.data.lock().expect("failed to get a lock");
        data.update();
        self.infos.set_text(&data.game.print_infos());
        let mut teamdata = data.game.print_team();
        teamdata.push_str(&data.game.print_my_team());
        self.team_box.set_text(&teamdata);
    }
}
