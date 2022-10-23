mod reconstruct;

pub use reconstruct::Reconstruct;

#[derive(Debug, Clone, Eq, PartialEq)]
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

impl Default for Player {
    fn default() -> Self {
        Player {
            name: String::new(),
            platform: String::new(),
            squadid: 0,
            score: 0,
            finished: false,
            died: false,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({}) team {}", self.name, self.platform, self.squadid)?;
        Ok(())
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let out = self.squadid.cmp(&other.squadid);
        if out == std::cmp::Ordering::Equal {
            other.score.cmp(&self.score)
        } else {
            out
        }
    }
}
