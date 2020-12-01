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

    match part1(&values, 2020) {
        Some(result) => println!("{}", result),
        None => println!("Error in part 1"),
    }
    match part2(&values) {
        Some(result) => println!("{}", result),
        None => println!("Error in part 2"),
    }
}

fn part1(values: &HashSet<i32>, target: i32) -> Option<i32> {
    for value in values{
        let diff = target - value;
        if values.contains(&diff) {
            return Some(diff * value);
        }
    }
    None
}

fn part2(values: &HashSet<i32>) -> Option<i32> {
    for value in values{
        let diff = 2020 - value;
        match part1(&values, diff){
            Some(result) => return Some(value * result),
            None => continue
        }
    }
    None
}
