// Advent of code 2020
// day 10
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::min;

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut series: Vec<usize> = Vec::new();
    // add starting outlet to seies (0)
    series.push(0);
    // parse input file into series data
    for line in reader.lines().map(|l| l.unwrap()){
        let number = line.parse().expect("Unable to parse number");
        series.push(number);
    }
    series.sort();
    // add your device to the series (3 more than max)
    series.push(series.last().unwrap() + 3);

    println!("{}", part1(&series));
    println!("{}", part2(&series));
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

fn part2(series: &Vec<usize>) -> usize {
    // create vector to keep track of each adapter's valid configs so far
    let mut configs: Vec<usize> = vec![0; series.len()];
    configs[0] = 1; // one valid way to get to starting socket
    for (idx, current_jolts) in series.iter().enumerate(){
        let end_idx = min(idx + 4, series.len()); // watching out for array bounds
        //print!("{}|", current_jolts);
        // for all adapters in the 3 following our current one
        for next_idx in idx+1..end_idx {
            // determine if we can reach adapter from our current one
            let next_jolts = series[next_idx];
            if next_jolts > current_jolts + 3 {
                break;
            }
            // subsequent adapter can be reached by all valid subconfigs of current adapter
            configs[next_idx] += configs[idx];
            //print!("({}, {})", next_jolts, configs[next_idx]);
        }
        //println!();
    }
    let total_configs = configs.pop().unwrap();
    total_configs
}
