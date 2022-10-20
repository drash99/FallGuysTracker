pub mod parsing;

use parsing::*;
use std::fs;

fn main() {
    let file = fs::File::open("C:\\Users\\Leo\\Desktop\\Rust\\test.log").unwrap();
    let mut parser = Parser::new(file);
    parser.parse();
    for parsed in parser.parsed {
        if parsed != ParseLineResult::Misc {
            println!("{:?}", parsed);
        }
    }
    println!("Hello, world!");
}
