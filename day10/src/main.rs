// Advent of code 2020
// day 10
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

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
    series.sort();
    // add your device to the series (3 more than max)
    series.push(series.last().unwrap() + 3);

    println!("{}", part1(&series));
    //println!("{}", part2(&series).expect("Unable to get result for part 2"));
}

fn part1(series: &Vec<usize>) -> usize {
    let mut prev_jolts = 0;
    let mut num_1s = 0;
    let mut num_3s = 0;
    for adapter in series.iter() {
        match adapter - prev_jolts {
            1 => {num_1s += 1},
            3 => {num_3s += 1},
            _ => ()
        }
        prev_jolts = *adapter;
    }
    num_1s * num_3s
}
