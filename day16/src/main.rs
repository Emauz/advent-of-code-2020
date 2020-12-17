// Advent of code 2020
// day 16
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::RangeInclusive;

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    // parse valid ranges
    let mut valid_ranges: Vec<(String, Vec<RangeInclusive<usize>>)> = Vec::new();
    for line in &mut lines{
        // break when we get to end of valid ranges section
        if line == "" {
            break;
        }
        let split_line: Vec<&str> = line.split(": ").collect();
        let range_label: &str = split_line[0];
        let range_strings: Vec<&str> = split_line[1].split(" or ").collect();
        let mut ranges = Vec::<RangeInclusive<usize>>::new();
        for range in range_strings {
            let range: Vec<usize> = range.split('-')
                                         .map(|x| x.parse().unwrap())
                                         .collect();
            let range = range[0] ..= range[1];
            ranges.push(range);
        }
        valid_ranges.push( (range_label.to_string(), ranges) );
    }

    // parse user line
    lines.next(); // skip "your ticket" label
    let my_ticket: Vec<usize> = lines.next()
                                     .unwrap()
                                     .split(',')
                                     .map(|x| x.parse().unwrap())
                                     .collect();
    lines.next(); // skip empty line
    lines.next(); // skip "nearby tickets label"
    
    // parse nearby tickets
    let mut nearby_tickets: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let ticket: Vec<usize> = line.split(',')
                                     .map(|x| x.parse().unwrap())
                                     .collect();
        nearby_tickets.push(ticket);
    }


    println!("{}", part1(&valid_ranges, &nearby_tickets));
    //println!("{}", part2(&series).expect("Unable to get result for part 2"));
}

fn part1(valid_ranges: &Vec<(String, Vec<RangeInclusive<usize>>)>, nearby_tickets: &Vec<Vec<usize>>) 
         -> usize {

    let mut invalid_entries: Vec<usize> = Vec::new();
    for ticket in nearby_tickets {
        'current_ticket_check: for value in ticket {
            // check if value in any of the valid ranges
            for range_double in valid_ranges.iter().map(|x| &x.1) {
                for range in range_double {
                    if range.contains(value) {
                        continue 'current_ticket_check;
                    }
                }
            }
            // value not found anywhere
            invalid_entries.push(*value);
        }
    }
    // return sum of all invalid entries
    invalid_entries.iter().fold(0, |acc, x| acc + x)
}
