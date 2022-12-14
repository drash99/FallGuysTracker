use std::cmp::Ordering;
use std::time::Instant;

use super::*;
use crate::parsing::ParseLineResult;

#[derive(Eq, Clone)]
pub struct Team {
    pub players: Vec<usize>,
    pub score: usize,
    pub num: usize,
}
fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
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
    pub teams: [Option<Team>; 60],
    pub myid: usize,
    pub myscore: usize,
    pub starttime: Instant,
}

impl Default for Reconstruct {
    fn default() -> Self {
        Self::new()
    }
}

impl Reconstruct {
    pub fn new() -> Reconstruct {
        const INIT: Option<Player> = None;
        const INIT_TEAM: Option<Team> = None;
        Reconstruct {
            playermap: [0; 4000],
            players: [INIT; 100],
            teams: [INIT_TEAM; 60],
            stage: String::new(),
            running: false,
            finishedplayers: 0,
            totalplayers: 0,
            myid: 0,
            myscore: 0,
            starttime: Instant::now(),
        }
    }
    fn recalc_team_score(&mut self) {
        for team in self.teams.iter_mut().flatten() {
            team.score = 0;
            for player in team.players.iter() {
                if let Some(player) = self.players[*player].as_ref() {
                    team.score += player.score;
                }
            }
        }
    }
    pub fn print_infos(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Stage: {}\n", truncate(&self.stage, 20)));
        out.push_str(&format!(
            "Finished players: {}/{}\n",
            self.finishedplayers, self.totalplayers
        ));
        if let Some(my) = self.players[self.myid].as_ref() {
            out.push_str(&format!("My score {}, #{}\n", my.score, self.myscore));
            if my.died {
                out.push_str(&format!("finish time : {:.2}\n", my.lifetime.as_secs_f32()));
            }
        }
        if self.running {
            out.push_str(&format!(
                "Current Time : {:.2}\n",
                self.starttime.elapsed().as_secs_f32()
            ));
        }
        out
    }
    pub fn print_my_team(&self) -> String {
        let mut out = String::new();
        let mut playervec = Vec::new();
        if let Some(my) = self.players[self.myid].as_ref() {
            for player in self.players.iter().flatten() {
                if player.squadid == my.squadid {
                    playervec.push(player);
                }
            }
        }
        playervec.sort();
        for player in playervec.iter() {
            if player.finished {
                out.push_str("??????");
            } else if player.died {
                out.push_str("??????");
            }
            out.push_str(&format!(
                "{:6}\t{}\n",
                truncate(&player.name, 6),
                player.score
            ));
        }
        out
    }
    pub fn print_team(&self) -> String {
        let mut myteam = 0;
        if let Some(my) = self.players[self.myid].as_ref() {
            myteam = my.squadid;
        }

        let mut out = String::new();
        let mut team = self
            .teams
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap().clone())
            .collect::<Vec<Team>>();
        team.sort();
        team.reverse();
        team.iter().enumerate().for_each(|(i, team)| {
            let p = team.players.len();
            let fin = team
                .players
                .iter()
                .filter(|x| self.players[**x].as_ref().unwrap().finished)
                .count();
            let died = team
                .players
                .iter()
                .filter(|x| self.players[**x].as_ref().unwrap().died)
                .count();
            if myteam == team.num {
                out.push_str(&format!("{}.??????", i + 1,));
            } else {
                out.push_str(&format!("{}. ", i + 1));
            }
            out.push_str(&format!(
                "Team {}:\t{}, {}/{}/{}\n",
                team.num, team.score, fin, died, p
            ))
        });
        out
    }
    pub fn push(&mut self, line: ParseLineResult) {
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
            }
            ParseLineResult::SpawnMatch(playernum, playerid, _squadid) => {
                self.playermap[playernum] = playerid;
            }
            ParseLineResult::SpawnMatchMe(playernum, playerid, _squadid) => {
                self.myid = playerid;
                self.playermap[playernum] = playerid;
            }
            ParseLineResult::Unspawn(playernum) => {
                if let Some(player) = &mut self.players[self.playermap[playernum]] {
                    player.died = true;
                    player.lifetime = self.starttime.elapsed();
                }
            }
            ParseLineResult::Success(playerid, finished) => {
                if let Some(player) = &mut self.players[playerid] {
                    player.finished = finished;
                    if finished && player.score == 0 {
                        //Assume no race condition
                        player.score = self.totalplayers - self.finishedplayers;
                        self.finishedplayers += 1;
                        if playerid == self.myid {
                            self.myscore = self.finishedplayers;
                        }
                        self.recalc_team_score();
                    }
                }
            }
            ParseLineResult::NumPlayersAchievingObjective(num) => {
                self.finishedplayers = num;
            }
            ParseLineResult::LoadedStage(stage) => {
                const INIT: Option<Player> = None;
                const INIT_TEAM: Option<Team> = None;
                self.playermap = [0; 4000];
                self.players = [INIT; 100];
                self.teams = [INIT_TEAM; 60];
                self.finishedplayers = 0;
                self.totalplayers = 0;
                self.stage = stage;
            }
            ParseLineResult::UnhandledGameSession(_) => {}
            ParseLineResult::Unhandled(_) => {}
            ParseLineResult::Score(playernum, score) => {
                let playerid = self.playermap[playernum];
                if let Some(player) = &mut self.players[playerid] {
                    player.score = score;
                }
                self.recalc_team_score();
            }
            ParseLineResult::Start => {
                self.running = true;
                self.starttime = Instant::now();
            }
            ParseLineResult::Shutdown => {
                self.running = false;
            }
            ParseLineResult::Misc => {}
            _ => {}
        }
        //println!("{:?}", l2);
    }
}

impl std::fmt::Display for Reconstruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Stage: {}", self.stage)?;
        writeln!(
            f,
            "Finished players: {}/{}",
            self.finishedplayers, self.totalplayers
        )?;
        if let Some(my) = self.players[self.myid].as_ref() {
            writeln!(f, "Me: {}: score {}, #{}", my, my.score, self.myscore)?;
            if my.died {
                writeln!(f, "time: {}", my.lifetime.as_secs())?;
            }
        }
        if self.running {
            writeln!(f, "Time: {:.2}", self.starttime.elapsed().as_secs_f32())?;
        }
        let mut playervec = self
            .players
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap().clone())
            .collect::<Vec<Player>>();
        playervec.sort();
        for player in playervec {
            writeln!(f, "{}: score {}", player, player.score)?;
        }
        Ok(())
    }
}
