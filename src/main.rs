pub mod parsing;
pub mod game;

use parsing::*;
use std::fs;

fn main() {
    let file = fs::File::open("C:\\Users\\Leo\\Desktop\\fallguys_tracker\\test.log").unwrap();
    let mut parser = Parser::new(file);
    parser.parse();
    for parsed in parser.parsed {
        if parsed != ParseLineResult::Misc {
            println!("{:?}", parsed);
        }
    }
    println!("Hello, world!");
}
