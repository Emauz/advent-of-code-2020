// Advent of code 2020
// day 6
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    // initialize struct to hold group data
    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut current_group: HashSet<char> = HashSet::new();
    for line in reader.lines().map(|l| l.unwrap()){
        if line.is_empty() {
            // current group is complete. push and make a new one.
            groups.push(current_group);
            current_group = HashSet::new();
        } else {
            // find all answers from current group and add to hashset
            let answers = line.chars();
            for data in answers {
                current_group.insert(data);
            }
        }
    }

    println!("{}", part1(&groups));
    //println!("{}", part2(&passports));
}

fn part1(groups: &Vec<HashSet<char>>) -> usize {
    let mut sum = 0;
    for group in groups {
        sum += group.len();
    }
    sum
}
