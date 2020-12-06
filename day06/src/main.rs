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
    let mut groups: Vec<Vec<HashSet<char>>> = Vec::new();
    let mut current_group: Vec<HashSet<char>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()){
        if line.is_empty() {
            // current group is complete. push and make a new one.
            groups.push(current_group);
            current_group = Vec::new();
        } else {
            // find all answers from current line and add to group
            let answers: HashSet<char> = line.chars().collect();
            current_group.push(answers);
        }
    }

    println!("{}", part1(&groups));
    println!("{}", part2(&groups));
}

fn part1(groups: &Vec<Vec<HashSet<char>>>) -> usize {
    let mut sum = 0;
    for group in groups {
        // Combine all hashsets (individual answers) to calculate all unique answers
        let mut all_answers: HashSet<char> = HashSet::new();
        for answers in group {
            all_answers = all_answers.union(&answers)
                                     .copied()
                                     .collect();
        }
        sum += all_answers.len();
    }
    sum
}

fn part2(groups: &Vec<Vec<HashSet<char>>>) -> usize {
    let mut sum = 0;
    for group in groups {
        // Find intersection of all answers
        let mut all_answers: HashSet<char> = ('a'..='z').collect();
        for answers in group {
            all_answers = all_answers.intersection(&answers)
                                     .copied()
                                     .collect();
        }
        sum += all_answers.len();
    }
    sum
}
