// Advent of code 2020
// day 17
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

const INIT_SIZE: usize = 40;

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    


    // initialize a 3d vector to represent empty space
    let mut space: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; INIT_SIZE]; INIT_SIZE]; INIT_SIZE];
    // and a 4d vector for part 2, why not?
    let mut space_4d: Vec<Vec<Vec<Vec<bool>>>> = vec![vec![vec![vec![false; INIT_SIZE]; INIT_SIZE]; INIT_SIZE]; INIT_SIZE];

    // parse starting position
    for (i, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        for (j, char) in line.chars().enumerate() {
            let i = i + INIT_SIZE/2;
            let j = j + INIT_SIZE/2;
            let k = INIT_SIZE/2;
            let w = INIT_SIZE/2;
            match char {
                '.' => {
                    space[i][j][k] = false;
                    space_4d[i][j][k][w] = false;
                },
                '#' => {
                    space[i][j][k] = true;
                    space_4d[i][j][k][w] = true;
                },
                _ => panic!("attempted to parse value that isn't '.' or '#'"),
            }
        }
    }

    println!("{}", part1(&space));
    println!("{}", part2(&space_4d));
}

fn calculate_live_neighbors(space: &Vec<Vec<Vec<bool>>>, i: usize, j: usize, k: usize) -> usize {
    let mut num_adjacent: usize = 0;
    for i_offset in -1 ..= 1 {
        for j_offset in -1 ..= 1 {
            for k_offset in -1 ..= 1 {
                if i_offset == 0 && j_offset == 0 && k_offset == 0 {
                    continue
                }
                let shifted_i: usize = ((i as i32) + i_offset) as usize;
                let shifted_j: usize = ((j as i32) + j_offset) as usize;
                let shifted_k: usize = ((k as i32) + k_offset) as usize;
                if space[shifted_i][shifted_j][shifted_k] {
                    num_adjacent += 1;
                }
            }
        }
    }
    num_adjacent
}

fn calculate_live_neighbors_4d(space: &Vec<Vec<Vec<Vec<bool>>>>, i: usize, j: usize, k: usize, w: usize) -> usize {
    let mut num_adjacent: usize = 0;
    for i_offset in -1 ..= 1 {
        for j_offset in -1 ..= 1 {
            for k_offset in -1 ..= 1 {
                for w_offset in -1 ..= 1{
                    if i_offset == 0 && j_offset == 0 && k_offset == 0 && w_offset == 0 {
                        continue
                    }
                    let shifted_i: usize = ((i as i32) + i_offset) as usize;
                    let shifted_j: usize = ((j as i32) + j_offset) as usize;
                    let shifted_k: usize = ((k as i32) + k_offset) as usize;
                    let shifted_w: usize = ((w as i32) + w_offset) as usize;
                    if space[shifted_i][shifted_j][shifted_k][shifted_w] {
                        num_adjacent += 1;
                    }
                }
            }
        }
    }
    num_adjacent
}


fn simulate_timestep(current: &Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<bool>>> {
    let mut next = current.clone();
    for i in 1 .. INIT_SIZE - 1 {
        for j in 1 .. INIT_SIZE - 1 {
            for k in 1 .. INIT_SIZE - 1 {
                //println!("{}, {}, {}", i, j, k);
                let live_neighbors = calculate_live_neighbors(&current, i, j, k);
                // determine if cell will be alive in next generation
                match (current[i][j][k], live_neighbors) {
                    (true, 2..=3) => next[i][j][k] = true,
                    (false, 3) => next[i][j][k] = true,
                    _ => next[i][j][k] = false,
                }
            }
        }
    }
    next
}

fn simulate_timestep_4d(current: &Vec<Vec<Vec<Vec<bool>>>>) -> Vec<Vec<Vec<Vec<bool>>>> {
    let mut next = current.clone();
    for i in 1 .. INIT_SIZE - 1 {
        for j in 1 .. INIT_SIZE - 1 {
            for k in 1 .. INIT_SIZE - 1 {
                for w in 1 .. INIT_SIZE - 1 {
                    //println!("{}, {}, {}", i, j, k);
                    let live_neighbors = calculate_live_neighbors_4d(&current, i, j, k, w);
                    // determine if cell will be alive in next generation
                    match (current[i][j][k][w], live_neighbors) {
                        (true, 2..=3) => next[i][j][k][w] = true,
                        (false, 3) => next[i][j][k][w] = true,
                        _ => next[i][j][k][w] = false,
                    }
                }
            }
        }
    }
    next
}

fn part1(space: &Vec<Vec<Vec<bool>>>) -> usize {
    let mut current = space.clone();
    for _ in 0 .. 6 {
        current = simulate_timestep(&current);
    }
    let total_live = current.iter()
                            .fold(0, |acc, x| acc + x.iter()
                                                     .fold(0, |acc, y| acc + y.iter()
                                                                              .fold(0, |acc, z| if *z {acc + 1} else {acc})));

    //print_space(&current);
    total_live
}

fn part2(space: &Vec<Vec<Vec<Vec<bool>>>>) -> usize {
    let mut current = space.clone();
    for _ in 0 .. 6 {
        current = simulate_timestep_4d(&current);
    }
    let total_live = current.iter()
                            .fold(0, |acc, x| acc + x.iter()
                                                     .fold(0, |acc, y| acc + y.iter()
                                                                              .fold(0, |acc, w| acc + w.iter()
                                                                                                       .fold(0, |acc, z| if *z {acc + 1} else {acc}))));

    //print_space(&current);
    total_live
}

