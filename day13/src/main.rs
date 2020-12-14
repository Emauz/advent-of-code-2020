// Advent of code 2020
// day 13
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let mut lines = BufReader::new(file).lines();

    let start_time: i64 = lines.next()
                                         .unwrap()
                                         .unwrap()
                                         .parse()
                                         .unwrap();
    let bus_id_string: String = lines.next()
                                     .unwrap()
                                     .unwrap();
    let mut bus_ids: Vec<Option<i64>> = Vec::new();
    for id in bus_id_string.split(',') {
        match id {
            "x" => bus_ids.push(None),
            other => bus_ids.push(Some(other.parse().unwrap())),
        }
    }
                
    println!("{}", part1(start_time, &bus_ids));
    println!("{}", part2(&bus_ids));
}

fn part1(start_time: i64, bus_ids: &Vec<Option<i64>>) -> i64 {
    let (id, wait_time) = bus_ids.iter()
                                 .filter_map(|x| *x)
                                 .map(|x| (x, x - (start_time % x)) )
                                 .min_by_key(|x| x.1)
                                 .unwrap();
    id * wait_time
}

fn part2(bus_ids: &Vec<Option<i64>>) -> i64 {
    let mut bus_ids = bus_ids.iter();
    let a = bus_ids.next().unwrap().unwrap();
    let mut cycle = (0, a); // (preamble length, cycle length)
    let mut current_offset = 1; // offset from beginning
    for id in bus_ids {
        match id {
            None => {current_offset += 1},
            Some(id) => {
                cycle = calculate_cycle(cycle, *id, current_offset);
                current_offset += 1;
            },
        };
    }
    cycle.0
}

fn calculate_cycle(prev_cycle: (i64, i64), b: i64, offset: i64) -> (i64, i64) {
    println!("Calculating period of: {:?}, {}, {}: ", prev_cycle, b, offset);
    let mut time = prev_cycle.0;
    while (time + offset) % b != 0 {
        time += prev_cycle.1;
    }
    let new_cycle = (time, prev_cycle.1 * b);
    println!("{:?}", new_cycle);
    new_cycle
}

