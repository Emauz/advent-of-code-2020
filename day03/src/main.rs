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

    println!("{}", part1(&forest, 1, 3));
    println!("{}", part2(&forest));
}

fn part1(forest: &Vec<Vec<bool>>, delta_row: usize, delta_col: usize) -> usize{
    let mut row = 0;
    let mut col = 0;
    let mut num_collisions = 0;
    let forest_width = forest[0].len();
    loop  {
        if forest[row][col % forest_width] 
            { num_collisions += 1 };
        row += delta_row;
        col += delta_col;
        if row >= forest.len() { break; }
    }
    num_collisions
}

fn part2(forest: &Vec<Vec<bool>>) -> usize {
    let mut product = 1;
    product *= part1(forest, 1, 1);
    product *= part1(forest, 1, 3);
    product *= part1(forest, 1, 5);
    product *= part1(forest, 1, 7);
    product *= part1(forest, 2, 1);
    product
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
