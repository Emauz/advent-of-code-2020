// Advent of code 2020
// day 15
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let line = reader.lines()
                     .next()
                     .unwrap()
                     .unwrap();
    let starting_numbers: Vec<usize> = line.split(',')
                                           .map(|x| x.parse().unwrap())
                                           .collect();

    println!("{}", part1(&starting_numbers, 2020));
    println!("{}", part1(&starting_numbers, 30_000_000));
}

fn part1(starting_numbers: &Vec<usize>, target_num: usize) -> usize {
    let last_starting_num: usize = starting_numbers[starting_numbers.len() - 1];
    let starting_numbers = &starting_numbers[0 .. starting_numbers.len() - 1];
    let mut spoken_record: HashMap<usize, usize> = HashMap::new();
    for (i, num) in starting_numbers.iter().enumerate() {
        spoken_record.insert(*num, i);
    }
    let mut next_number: usize = starting_numbers.len() - spoken_record.get(&last_starting_num)
                                                                       .unwrap_or(&starting_numbers.len());
    spoken_record.insert(last_starting_num, starting_numbers.len());
    for i in (starting_numbers.len() + 1) .. target_num - 1 {
        let current_number = next_number;
        next_number = match spoken_record.insert(current_number, i) {
            Some(n) => {i - n},
            None => 0,
        }
    }
    next_number
}

