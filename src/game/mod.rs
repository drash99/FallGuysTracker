mod reconstruct;

pub use reconstruct::Reconstruct;

pub struct Player {
    name: String,
    platform: String,
    squadid: usize,
    score: usize,
    finished: bool,
    died: bool,

}

impl Player {
    pub fn new(name: String, platform: String, squadid: usize) -> Player {
        Player {
            name,
            platform,
            squadid,
            score: 0,
            finished: false,
            died: false,
        }
    }
}