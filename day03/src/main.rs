// Advent of code 2020
// day 3
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut forest: Vec<Vec<bool>> = Vec::new();
    for line in reader.lines().map(|l| l.expect("Error reading line!")) {
        let mut trees: Vec<bool> = Vec::new();
        for ch in line.chars() {
            trees.push(if ch == '#' { true } else { false });
        }
        forest.push(trees);
    }

    //_test(&forest);

    part1(&forest);
    //part2();
}

fn part1(forest: &Vec<Vec<bool>>) {
    let mut row = 0;
    let mut col = 0;
    let delta_row = 1;
    let delta_col = 3;
    let mut num_collisions = 0;
    let forest_width = forest[0].len();
    loop  {
        if forest[row][col % forest_width] 
            { num_collisions += 1 };
        row += delta_row;
        col += delta_col;
        if row >= forest.len() { break; }
    }
    println!("{}", num_collisions);
}

// Function to test if input parsing went ok
fn _test(forest: &Vec<Vec<bool>>) {
    for line in forest {
        for tree in line {
            print!("{}", if *tree { "#" } else { "." });
        }
        println!("");
    }
}
