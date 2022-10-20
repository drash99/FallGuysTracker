use super::*;
use regex::Regex;
use std::fs::*;
use std::io::*;

#[derive(Debug)]
pub struct Parser {
    pub file: File,
    pub parsed: Vec<ParseLineResult>,
    regex_spawn: Regex,
    regex_spawn2: Regex,
    regex_spawn_match: Regex,
    regex_unspawn: Regex,
    regex_success: Regex,
    regex_game_status: Regex,
    regex_loaded_stage: Regex,
}

impl Parser {
    pub fn new(file: File) -> Parser {
        Parser {
            file,
            parsed: Vec::new(),
            regex_spawn: Regex::new(r"\d{4}-\d{2}-\d{2}: \[\w*\] Finalising spawn for player FallGuy \[(\d*)\] ([^(]*) \((\w*)\)+").unwrap(),
            regex_spawn2: Regex::new(r"\d{4}-\d{2}-\d{2}: \[\w*\] Adding Spectator target \w*_([^(]*) \((\w*)\) with Party ID: ([\d ]*) Squad ID: (\d*) and playerID: (\d*)+").unwrap(),
            regex_spawn_match: Regex::new(r"\d{4}-\d{2}-\d{2}: \[\w*\] Handling bootstrap for \w* player FallGuy \[(\d*)\] \([\w\d.]*\), playerID = (\d*), squadID = (\d*)+").unwrap(),
            regex_unspawn : Regex::new(r"\d{4}-\d{2}-\d{2}: \[\w*\] Handling unspawn for player FallGuy \[(\d*)\]").unwrap(),
            regex_success : Regex::new(r"\d{4}-\d{2}-\d{2}: ClientGameManager::HandleServerPlayerProgress PlayerId=(\d*) is succeeded=(\w*)").unwrap(),
            regex_game_status : Regex::new(r"\d{4}-\d{2}-\d{2}: \[ClientGameSession\] NumPlayersAchievingObjective=(\d*)").unwrap(),
            regex_loaded_stage : Regex::new(r"\d{4}-\d{2}-\d{2}: \[\w*\] Loading game level scene ([\w_]*)").unwrap(),
        }
    }

    fn get_lines(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        let mut buffer = String::new();
        self.file.read_to_string(&mut buffer).unwrap();
        for line in buffer.lines() {
            lines.push(line.to_string());
        }
        lines
    }

    fn parse_line(&self, line: String) -> ParseLineResult {
        if !line.contains(' ') {
            return ParseLineResult::Misc;
        }
        let mut split = line.split(" ");
        let _first = split.next().unwrap();
        let second = split.next().unwrap();
        match second {
            "[ClientGameSession]" => {
                let captured = self.regex_game_status.captures(&line).unwrap_or_else(|| panic!("Regex match failed for {}", line));
                ParseLineResult::NumPlayersAchievingObjective(captured.get(1).unwrap().as_str().parse::<usize>().unwrap())
                
            },
            "[ClientGameManager]" => match split.next().unwrap() {
                "Shutdown" => ParseLineResult::Shutdown,
                "Finalising" => {
                    let captured = self
                        .regex_spawn
                        .captures(&line)
                        .unwrap_or_else(|| panic!("Failed to parse line: {}", line));
                    ParseLineResult::Spawned2(
                        captured
                            .get(2)
                            .map_or(String::new(), |m| m.as_str().to_string()),
                        captured
                            .get(3)
                            .map_or(String::new(), |m| m.as_str().to_string()),
                        captured
                            .get(1)
                            .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                        0,
                    )
                }
                "Handling" => match split.next().unwrap() {
                    "bootstrap" => {
                        let captured = self
                            .regex_spawn_match
                            .captures(&line)
                            .unwrap_or_else(|| panic!("Failed to parse line: {}", line));
                        ParseLineResult::SpawnMatch(
                            captured
                                .get(1)
                                .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                            captured
                                .get(2)
                                .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                            captured
                                .get(3)
                                .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                        )
                    }
                    "unspawn" => {
                        let captured = self
                            .regex_unspawn
                            .captures(&line)
                            .unwrap_or_else(|| panic!("Failed to parse line: {}", line));
                        ParseLineResult::Unspawn(
                            captured
                                .get(1)
                                .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                        )
                    }
                    _ => ParseLineResult::Unhandled(line),
                },
                _ => ParseLineResult::Unhandled(line),
            },
            "[CameraDirector]" => match split.next().unwrap() {
                "Adding" => {
                    let captured = self
                        .regex_spawn2
                        .captures(&line)
                        .unwrap_or_else(|| panic!("Failed to parse line: {}", line));
                    ParseLineResult::Spawned(
                        captured
                            .get(1)
                            .map_or(String::new(), |m| m.as_str().to_string()),
                        captured
                            .get(2)
                            .map_or(String::new(), |m| m.as_str().to_string()),
                        captured
                            .get(5)
                            .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                        captured
                            .get(4)
                            .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                    )
                }
                _ => ParseLineResult::Unhandled(line),
            },
            "ClientGameManager::HandleServerPlayerProgress" => {
                let captured = self
                    .regex_success
                    .captures(&line)
                    .unwrap_or_else(|| panic!("Failed to parse line: {}", line));
                ParseLineResult::Success(
                    captured
                        .get(1)
                        .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                    if captured.get(2).map_or(false, |m| m.as_str() == "True") {
                        true
                    } else {
                        false
                    },
                )
            },
            "[StateGameLoading]" => {
                match split.next().unwrap() {
                    "Loading" => {
                        let captured = self.regex_loaded_stage.captures(&line).unwrap_or_else(|| panic!("Failed to capture loaded stage"));
                        ParseLineResult::LoadedStage(captured.get(1).unwrap().as_str().to_string())
                    },
                    "Starting" => ParseLineResult::Start,
                    _ => ParseLineResult::Unhandled(line),
                }
            },
            "Player" => {
                let playernum = split.next().unwrap().parse::<usize>().unwrap_or_else(|_| panic!("Failed to parse playernum {}",line));
                let _ = split.next().unwrap();
                let _ = split.next().unwrap();
                let score = split.next().unwrap().parse::<usize>().unwrap();
                ParseLineResult::Score(playernum, score)
            },
            _ => ParseLineResult::Misc,
        }
    }

    pub fn parse(&mut self) {
        let lines = self.get_lines();
        for line in lines {
            let result = self.parse_line(line);
            self.parsed.push(result);
        }
    }
}
