use super::*;
use parsing::ParseLineResult;

pub struct Reconstruct {
    pub playermap: [usize; 4000],
    pub players: [Option<Player>; 100],
    pub stage: String,
    pub running: bool,
    pub finishedplayers: usize,
}

impl Reconstruct {
    pub fn new() -> Reconstruct {
        Reconstruct {
            playermap: [0; 4000],
            players: [None; 100],
            stage: String::new(),
            running: false,
            finishedplayers: 0,
        }
    }
    pub fn push(line:ParseLineResult) {
        match line {
            ParseLineResult::Spawned(name, platform, playerid, squadid) => {
                self.players[squadid] = Some(Player::new(name, platform, squadid));
            },
            ParseLineResult::SpawnMatch(playernum, playerid, squadid) => {
                self.playermap[plyaernum] = playerid;
            },
            ParseLineResult::Unspawn(playernum) => {
                if let Some(player) = self.players[self.playermap[playernum]] {
                    player.died = true;
                }
            },
            ParseLineResult::Success(playernum, finished) => {
                let playerid = self.playermap[playernum];
                self.players[playerid].finished = finished;
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
                self.players[self.playermap[playernum]].score = score;
            },
            ParseLineResult::Start => {
                self.running = true;
            },
            ParseLineResult::Shutdown => {
                self.running = false;
            },
            ParseLineResult::Misc => {},
        }
    }
}