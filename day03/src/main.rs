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
    let mut forest: Vec<Vec<bool>> = vec!(vec!());
    for line in reader.lines().map(|l| l.expect("Error reading line!")) {
        let mut trees: Vec<bool> = vec!();
        for ch in line.chars() {
            trees.push(if ch == '#' { true } else { false });
        }
        forest.push(trees);
    }

    //_test(&forest);

    //part1();
    //part2();
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
