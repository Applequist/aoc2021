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
        .fold((0usize, 0usize, 0usize), |(a, h, d), cmd| {
            match cmd {
                Command::Forward(amt) => (a, h + amt as usize, d + a * amt as usize),
                Command::Up(amt) => (a - amt as usize, h, d),
                Command::Down(amt) => (a + amt as usize, h, d),
            }
        });
    println!("Final position = {:?}", final_position);
    println!("h * d = {}", final_position.1 * final_position.2);
}
