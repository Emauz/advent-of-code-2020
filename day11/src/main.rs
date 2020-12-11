// Advent of code 2020
// day 11
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone, PartialEq)]
enum Position {
    Floor,
    Empty,
    Taken,
}

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut ferry: Vec<Vec<(Position, u8)>> = Vec::new();
    // parse input file into seats data
    for line in reader.lines().map(|l| l.unwrap()){
        let mut current_row: Vec<(Position, u8)> = Vec::new();
        for char in line.chars() {
            let current_pos = match char {
                '.' => Position::Floor,
                'L' => Position::Empty,
                '#' => Position::Taken,
                _ => {
                    panic!("Unable to parse current char!");
                }
            };
            current_row.push((current_pos, 0));
        }
        ferry.push(current_row);
    }

    println!("{}", part1(&ferry));
    //println!("{}", part2(&series));
}

fn part1(ferry: &Vec<Vec<(Position, u8)>>) -> i32 {
    let mut ferry = ferry.clone();
    // loop until seats in a stable position
    let mut prev_taken = -1;
    loop {
        calculate_adjacency(&mut ferry);
        swap_seats(&mut ferry);
        let mut total_taken = 0;
        for row in &ferry {
            for pos in row {
                if pos.0 == Position::Taken {
                    total_taken += 1;
                }
            }
        }
        if total_taken == prev_taken {
            break
        }
        prev_taken = total_taken;
    }
    prev_taken
}

// Calculates number of adjacent taken seats for all positions in ferry
fn calculate_adjacency(ferry: &mut Vec<Vec<(Position, u8)>>) {
    let mut new_ferry = ferry.clone();
    for (i, row) in ferry.iter().enumerate() {
        // perform bounds checks
        let min_row = if i == 0 {0} else {i-1};
        let max_row = if i == ferry.len()-1 {ferry.len()-1} else {i+1};
        let row_bounds = min_row..=max_row;
        for (j, pos) in row.iter().enumerate() {
            if pos.0 == Position::Floor {
                continue;
            }
            let min_col = if j == 0 {0} else {j-1};
            let max_col = if j == row.len()-1 {row.len()-1} else {j+1};
            let col_bounds = min_col..=max_col;
            //iterate through all neighbors (in bounds, of course)
            let mut sum: u8 = 0;
            for other_row in row_bounds.clone() {
                for other_col in col_bounds.clone() {
                    // make sure we don't include current seat
                    /*
                    if other_row == 0 && other_col == 0 {
                        continue
                    }
                    */
                    if ferry[other_row][other_col].0 == Position::Taken {
                        sum += 1;
                    }
                }
            }
            new_ferry[i][j].1 = sum;
        }
    }
    *ferry = new_ferry
}

fn swap_seats(ferry: &mut Vec<Vec<(Position, u8)>>) {
    for row in ferry {
        for mut pos in row {
            match pos.0 {
                Position::Floor => continue,
                Position::Empty => {
                    if pos.1 == 0 {
                        pos.0 = Position::Taken;
                    }
                },
                Position::Taken => {
                    if pos.1 >= 5 {
                        pos.0 = Position::Empty;
                    }
                },
            }
        }
    }
}

fn print_ferry(ferry: &Vec<Vec<(Position, u8)>>) {
    for row in ferry {
        for pos in row {
            print!("{}", match pos.0 {
                Position::Floor => '.',
                Position::Empty => 'L',
                Position::Taken => '#',
            });
        }
        println!();
    }
    println!();
}
