use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let file = File::open("inputs/depths.txt").expect("Can't open input");
    let reader = BufReader::new(file);
    let increase_count: usize = reader.lines()
        .map(|l| l.unwrap().parse::<u16>().unwrap())
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum();
    println!("{:?}", increase_count);
}