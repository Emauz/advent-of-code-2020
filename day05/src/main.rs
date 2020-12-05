// Advent of code 2020
// day 5
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
    let mut tickets: Vec<(usize, usize)> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()){
        let mut ch = line.chars();
        let mut row: usize = 0;
        let mut seat: usize = 0;
        // calculate row number by decoding binary num
        for i in (0..7).rev() {
            if ch.next().unwrap() == 'B' {
                row += 2_usize.pow(i);
            }
        }
        for i in (0..3).rev() {
            if ch.next().unwrap() == 'R' {
                seat += 2_usize.pow(i);
            }
        }
        tickets.push((row, seat));
    }

    println!("{}", part1(&tickets));
    println!("{}", part2(&tickets).unwrap());
}

fn part1(tickets: &Vec<(usize, usize)>) -> usize {
    let mut max_id: usize = 0;
    for (row, seat) in tickets {
        //println!("{}", (row * 8));
        let id: usize = (row * 8) + seat;
        if id > max_id {
            max_id = id;
        }
    }
    max_id
}

fn part2(tickets: &Vec<(usize, usize)>) -> Option<usize> {
    let mut ids: Vec<usize> = tickets.iter()
                                     .map(|(row, seat)| (row * 8) + seat)
                                     .collect();
    ids.sort();
    for (i, id) in ids.iter().enumerate() {
        if ids.get(i+1).unwrap() != &(id + 1) {
            return Some(id + 1);
        }
    }
    None
}
