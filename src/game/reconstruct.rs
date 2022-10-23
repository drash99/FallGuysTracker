use std::cmp::Ordering;

use super::*;
use crate::parsing::ParseLineResult;

#[derive (Eq, Clone)]
pub struct Team{
    pub players : Vec::<usize>,
    pub score : usize,
    pub num : usize,
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}
impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
pub struct Reconstruct {
    pub playermap: [usize; 4000],
    pub players: [Option<Player>; 100],
    pub stage: String,
    pub running: bool,
    pub finishedplayers: usize,
    pub totalplayers: usize,
    pub teams: [Option<Team>; 30],
}

impl Reconstruct {
    pub fn new() -> Reconstruct {
        const INIT: Option<Player> = None;
        const INIT_TEAM: Option<Team> = None;
        Reconstruct {
            playermap: [0; 4000],
            players: [INIT; 100],
            teams: [INIT_TEAM; 30],
            stage: String::new(),
            running: false,
            finishedplayers: 0,
            totalplayers: 0,
        }
    }
    fn recalc_team_score(&mut self) {
        for team in self.teams.iter_mut() {
            if let Some(team) = team {
                team.score = 0;
                for player in team.players.iter() {
                    if let Some(player) = self.players[*player].as_ref() {
                        team.score += player.score;
                    }
                }
            }
        }
    }
    pub fn print_team(&self) -> String {
        let mut out = String::new();
        let mut team = self.teams
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap().clone())
            .collect::<Vec<Team>>();
        team.sort();
        team.reverse();
        team.iter().enumerate().for_each(|(i, team)| {
            out.push_str(&format!("{}.Team {}: {}\n", i + 1,team.num, team.score));
        });
        out
    }
    pub fn push(&mut self, line:ParseLineResult) {
        //let l2 = line.clone();
        match line {
            ParseLineResult::Spawned(name, platform, playerid, squadid) => {
                self.totalplayers += 1;
                self.players[playerid] = Some(Player::new(name, platform, squadid));
                if let Some(team) = self.teams[squadid].as_mut() {
                    team.players.push(playerid);
                } else {
                    self.teams[squadid] = Some(Team {
                        players: vec![playerid],
                        score: 0,
                        num: squadid,
                    });
                }
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
                    if finished && player.score == 0 {
                        //Assume no race condition
                        player.score = self.totalplayers-self.finishedplayers;
                        self.finishedplayers += 1;
                        self.recalc_team_score();
                    }
                }
            },
            ParseLineResult::NumPlayersAchievingObjective(num) => {
                self.finishedplayers = num;
            },
            ParseLineResult::LoadedStage(stage) => {
                const INIT: Option<Player> = None;
                const INIT_TEAM: Option<Team> = None;
                self.playermap= [0; 4000];
                self.players= [INIT; 100];
                self.teams = [INIT_TEAM; 30];
                self.finishedplayers= 0;
                self.totalplayers= 0;
                self.stage = stage;
            },
            ParseLineResult::UnhandledGameSession(_) => {return;},
            ParseLineResult::Unhandled(_) => {return;},
            ParseLineResult::Score(playernum, score) => {
                let playerid = self.playermap[playernum];
                if let Some(player) = &mut self.players[playerid] {
                    player.score = score;
                }
                self.recalc_team_score();
            },
            ParseLineResult::Start => {
                self.running = true;
            },
            ParseLineResult::Shutdown => {
                self.running = false;
            },
            ParseLineResult::Misc => { return;},
            _=> {return;},
        }
        //println!("{:?}", l2);
    }
}

impl std::fmt::Display for Reconstruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stage: {}\n", self.stage)?;
        write!(f, "Finished players: {}/{}\n", self.finishedplayers, self.totalplayers)?;
        let mut playervec = self.players.iter().filter(|x| x.is_some()).map(|x| x.as_ref().unwrap().clone()).collect::<Vec<Player>>();
        playervec.sort();
        for player in playervec {
            write!(f, "{}: score {}\n", player, player.score)?;
        }
        Ok(())
    }
}