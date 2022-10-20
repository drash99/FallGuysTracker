use super::*;
use crate::parsing::ParseLineResult;

pub struct Reconstruct {
    pub playermap: [usize; 4000],
    pub players: [Option<Player>; 100],
    pub stage: String,
    pub running: bool,
    pub finishedplayers: usize,
}

impl Reconstruct {
    pub fn new() -> Reconstruct {
        const INIT: Option<Player> = None;
        Reconstruct {
            playermap: [0; 4000],
            players: [INIT; 100],
            stage: String::new(),
            running: false,
            finishedplayers: 0,
        }
    }
    pub fn push(&mut self, line:ParseLineResult) {
        match line {
            ParseLineResult::Spawned(name, platform, playerid, squadid) => {
                self.players[playerid] = Some(Player::new(name, platform, squadid));
            },
            ParseLineResult::SpawnMatch(playernum, playerid, _squadid) => {
                self.playermap[playernum] = playerid;
            },
            ParseLineResult::Unspawn(playernum) => {
                if let Some(player) = &mut self.players[self.playermap[playernum]] {
                    player.died = true;
                }
            },
            ParseLineResult::Success(playernum, finished) => {
                let playerid = self.playermap[playernum];
                if let Some(player) = &mut self.players[playerid] {
                    player.finished = finished;
                }
            },
            ParseLineResult::NumPlayersAchievingObjective(num) => {
                self.finishedplayers = num;
            },
            ParseLineResult::LoadedStage(stage) => {
                self.stage = stage;
            },
            ParseLineResult::UnhandledGameSession(_) => {},
            ParseLineResult::Unhandled(_) => {},
            ParseLineResult::Score(playernum, score) => {
                let playerid = self.playermap[playernum];
                if let Some(player) = &mut self.players[playerid] {
                    player.score = score;
                }
            },
            ParseLineResult::Start => {
                self.running = true;
            },
            ParseLineResult::Shutdown => {
                self.running = false;
            },
            ParseLineResult::Misc => {},
            _=> {},
        }
    }
}

impl std::fmt::Display for Reconstruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stage: {}\n", self.stage)?;
        write!(f, "Finished players: {}\n", self.finishedplayers)?;
        for player in self.players.iter() {
            if let Some(player) = player {
                write!(f, "{}: {}\n", player, player.score)?;
            }
        }
        Ok(())
    }
}