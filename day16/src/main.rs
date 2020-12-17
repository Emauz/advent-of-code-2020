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
    println!("{}", part2(&valid_ranges, &my_ticket, &mut nearby_tickets));
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

fn part2(valid_ranges: &Vec<(String, Vec<RangeInclusive<usize>>)>, 
         my_ticket: &Vec<usize>,
         nearby_tickets: &mut Vec<Vec<usize>>) -> usize {

    // step 1: prune any invalid tickets from nearby
    let mut invalid_ticket_indices: Vec<usize> = Vec::new();
    for (i, ticket) in nearby_tickets.iter().enumerate() {
        'current_ticket_check: for value in ticket {
            // check if value in any of the valid ranges
            for range_double in valid_ranges.iter().map(|x| &x.1) {
                for range in range_double {
                    if range.contains(value) {
                        continue 'current_ticket_check;
                    }
                }
            }
            // value not found anywhere, this ticket is INVALID!
            invalid_ticket_indices.push(i);
            break 'current_ticket_check;
        }
    }
    for i in invalid_ticket_indices.iter().rev() {
        nearby_tickets.remove(*i);
    }
    // invalid nearby tickets have been pruned

    // step 2: Prune ranges
    // initialize possible fields for each field (what ticket we use doesn't matter)
    let mut possible_ranges: Vec<(usize, Vec<usize>)> = Vec::new();
    for (field_idx, value) in my_ticket.iter().enumerate() {
        possible_ranges.push( (field_idx, Vec::new()) );
        for (range_idx, range_double) in valid_ranges.iter().map(|x| &x.1).enumerate() {
            for range in range_double {
                if range.contains(value) {
                    possible_ranges[field_idx].1.push(range_idx);
                }
            }
        }
    }
    // prune ranges that don't match with further tickets
    for ticket in nearby_tickets {
        for (field_idx, field_value) in ticket.iter().enumerate() {
            let mut pruned_ranges: Vec<usize> = Vec::new(); // ranges that don't match with current value
            for (range_idx, range_tuple_idx) in possible_ranges[field_idx].1.iter().enumerate() {
                let range_tuple = &valid_ranges[*range_tuple_idx];
                // check if range is valid for current field value
                let mut range_valid: bool = false;
                for range in &range_tuple.1 {
                    if range.contains(field_value) {
                        range_valid = true;
                        break;
                    }
                }
                if !range_valid {
                    pruned_ranges.push(range_idx);
                }
            }
            for pruned_idx in pruned_ranges.iter().rev() {
                possible_ranges[field_idx].1.remove(*pruned_idx);
            }
        }
    }
    // incompatible ranges have been pruned.

    // step 3: ensure only one entry for each field
    possible_ranges.sort_by(|a,b| a.1.len().cmp(&b.1.len())); // sort by length of possible matches
    for i in 0 .. possible_ranges.len() - 1 {
        let target_num: usize = possible_ranges[i].1[0];
        for j in i + 1 .. possible_ranges.len() {
            let current_vec = &mut possible_ranges[j].1;
            let target_idx = current_vec.iter().position(|x| *x == target_num).unwrap();
            current_vec.remove(target_idx);
        }
    }
    possible_ranges.sort();

    // step 4: find all ranges that come from fields that include "Departure"
    let mut departure_ranges: Vec<usize> = Vec::new();
    for (idx, range) in valid_ranges.iter().enumerate() {
        if range.0.contains("departure") {
            departure_ranges.push(idx);
        }
    }

    // step 5: multiply target 6 values together
    let mut multiplied_value: usize = 1;
    for range in departure_ranges {
        // find the field for which that range is valid
        let valid_field_idx = possible_ranges.iter()
                                             .position(|x| x.1[0] == range)
                                             .unwrap();
        let valid_field_idx = possible_ranges[valid_field_idx].0;
        multiplied_value *= my_ticket[valid_field_idx];
    }

    multiplied_value
}
