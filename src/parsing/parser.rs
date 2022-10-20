use super::*;
use std::fs::*;
use std::io::*;
use regex::Regex;



#[derive(Debug)]
pub struct Parser {
    pub file: File,
    pub parsed: Vec<ParseLineResult>,
    regex_spawn : Regex,
    regex_spawn2 : Regex,
    regex_spawn_match : Regex,
}

impl Parser {
    pub fn new(file: File) -> Parser {
        Parser {
            file,
            parsed: Vec::new(),
            regex_spawn: Regex::new(r"\d{4}-\d{2}-\d{2}: \[\w*\] Finalising spawn for player FallGuy \[(\d*)\] ([^(]*) \((\w*)\)+").unwrap(),
            regex_spawn2: Regex::new(r"\d{4}-\d{2}-\d{2}: \[\w*\] Adding Spectator target \w*_([^(]*) \((\w*)\) with Party ID: ([\d ]*) Squad ID: (\d*) and playerID: (\d*)+").unwrap(),
            regex_spawn_match: Regex::new(r"\d{4}-\d{2}-\d{2}: \[\w*\] Handling bootstrap for \w* player FallGuy \[(\d*)\] \([\w\d.]*\), playerID = (\d*), squadID = (\d*)+").unwrap(),

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
        if !line.contains(' '){
            return ParseLineResult::Misc;
        }
        let mut split = line.split(" ");
        let _first = split.next().unwrap();
        let second = split.next().unwrap();
        match second {
            "[ClientGameSession]" => ParseLineResult::UnhandledGameSession(line),
            "[ClientGameManager]" => match split.next().unwrap() {
                "Finalising" => {
                    let captured = self.regex_spawn.captures(&line).unwrap();
                    ParseLineResult::Spawned(
                        captured.get(2).map_or(String::new(), |m| m.as_str().to_string()),
                        captured.get(3).map_or(String::new(), |m| m.as_str().to_string()), 
                        captured.get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap()), 0)
                },
                "Handling" => {
                    if split.next().unwrap() == "bootstrap"{
                        let captured = self.regex_spawn_match.captures(&line).unwrap_or_else(|| panic!("Failed to parse line: {}", line));
                        ParseLineResult::SpawnMatch(
                            captured.get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                            captured.get(2).map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                            captured.get(3).map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                        )
                    }else {
                        ParseLineResult::Unhandled(line)
                    }
                }
                _ => ParseLineResult::Unhandled(line),
            },
            "[CameraDirector]" => match split.next().unwrap() {
                "Adding" => {
                    let captured = self.regex_spawn2.captures(&line);
                    if captured.is_none() {
                        return ParseLineResult::Unhandled(line);
                    }
                    let captured = captured.unwrap();
                    ParseLineResult::Spawned(
                        captured.get(1).map_or(String::new(), |m| m.as_str().to_string()),
                        captured.get(2).map_or(String::new(), |m| m.as_str().to_string()), 
                        captured.get(5).map_or(0, |m| m.as_str().parse::<usize>().unwrap()), 
                        captured.get(4).map_or(0, |m| m.as_str().parse::<usize>().unwrap()))
                }
                _ => ParseLineResult::Unhandled(line),
            }
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