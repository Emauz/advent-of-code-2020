// Advent of code 2020
// day 1
// Eric Moss

use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("From file: {}", filename);
    let contents = fs::read_to_string(filename).expect("Unable to read file");

    let values: HashSet<i32> = contents.lines()
                         .map(|line| line.parse::<i32>().unwrap())
                         .collect();
    part1(&values);
}

fn part1(values: &HashSet<i32>) {
    for value in values{
        let diff = 2020 - value;
        if values.contains(&diff) {
            println!("{}", diff * value);
            return;
        }
    }
}
