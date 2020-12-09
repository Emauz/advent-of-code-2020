// Advent of code 2020
// day 9
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, VecDeque};

//static PREAMBLE_LENGTH: usize = 5;
static PREAMBLE_LENGTH: usize = 25;

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut series: Vec<usize> = Vec::new();
    // parse input file into series data
    for line in reader.lines().map(|l| l.unwrap()){
        let number = line.parse().expect("Unable to parse number");
        series.push(number);
    }

    println!("{}", part1(&series).expect("Unable to get result for part 1"));
    //println!("{}", part2();
}

fn part1(series: &Vec<usize>) -> Option<usize> {
    let mut series = series.iter();
    // create struct to hold each valid number and it's valid successors
    let mut previous: VecDeque<(usize, HashSet<usize>)> = VecDeque::new();
    // initialize valid struct with preamble
    for _ in 0..PREAMBLE_LENGTH {
        let current = series.next().expect("Series has less elements than preamble length");
        for (prev, set) in &mut previous {
            set.insert(*prev + current);
        }
        previous.push_front((*current, HashSet::new()));
    }
    // Insert elements until one isn't valid
    for current in series {
        let mut valid = false;
        for (prev, set) in &mut previous {
            if set.contains(current) {
                valid = true;
            }
            set.insert(*prev + current);
        }
        if !valid {
            return Some(*current);
        }
        // add new one and cycle out oldest entry
        previous.push_front((*current, HashSet::new()));
        previous.pop_back();
    }
    None
}
