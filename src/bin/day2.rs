use itertools::Itertools;
use thiserror::Error;

enum Command {
    Up(u8),
    Down(u8),
    Forward(u8),
}

#[derive(Debug, Error)]
enum Error<'a> {
    #[error("Unknown command: {0}")]
    UnknownCommand(&'a str),
    #[error("Parse error")]
    ParseError(#[from] std::num::ParseIntError)
}
impl Command {
    fn parse(line: &str) -> Result<Self, Error> {
        let tokens = line.split(' ').collect::<Vec<_>>();
        let amount = tokens[1].parse::<u8>()?;
        match tokens[0] {
            "up" => Ok(Command::Up(amount)),
            "down" => Ok(Command::Down(amount)),
            "forward" => Ok(Command::Forward(amount)),
            _ => Err(Error::UnknownCommand(tokens[0])),
        }
    }
}

fn main() {
    let commands = include_str!("../../inputs/commands.txt").lines()
        .map(|l| Command::parse(l))
        .collect::<Result<Vec<Command>, Error>>().unwrap();
    let final_position = commands.into_iter()
        .fold((0usize, 0usize), |(h, d), cmd| {
            match cmd {
                Command::Forward(amt) => (h + amt as usize, d),
                Command::Up(amt) => (h, d - amt as usize),
                Command::Down(amt) => (h, d + amt as usize),
            }
        });
    println!("Final position = {:?}", final_position);
}
