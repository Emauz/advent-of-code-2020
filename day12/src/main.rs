// Advent of code 2020
// day 12
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

#[derive(Debug)]
enum Heading {
    East = 0,
    North = 90,
    West = 180,
    South = 270,
}

#[derive(Debug)]
struct Boat {
    x: i32,
    y: i32,
    heading: Heading,
}

#[derive(Default)]
struct Waypoint {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),

    L(i32),
    R(i32),
    F(i32),
}

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut instructions: Vec<Instruction> = Vec::new();
    let re = Regex::new(r"^([NSEWLRF])(\d+)$").unwrap();
    // parse all instructions
    for line in reader.lines().map(|l| l.unwrap()){
        let captures = re.captures(&line).unwrap();
        let heading = captures.get(1)
                              .unwrap()
                              .as_str();
        let val: i32 = captures.get(2)
                              .unwrap()
                              .as_str()
                              .parse()
                              .unwrap();
        instructions.push(match heading {
            "N" => Instruction::N(val),
            "S" => Instruction::S(val),
            "E" => Instruction::E(val),
            "W" => Instruction::W(val),

            "L" => Instruction::L(val),
            "R" => Instruction::R(val),
            "F" => Instruction::F(val),
            _ => panic!("Attempted to parse invalid heading"),
        });
    }

    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut boat = Boat { x: 0, y: 0, heading: Heading::East };
    for instruction in instructions {
        match instruction {
            Instruction::N(val) => { boat.y += val },
            Instruction::S(val) => { boat.y -= val },
            Instruction::E(val) => { boat.x += val },
            Instruction::W(val) => { boat.x -= val },

            Instruction::L(val) => {
                let new_heading_degrees = (boat.heading as i32 + val) % 360;
                boat.heading = deg_to_heading(new_heading_degrees);
            },
            Instruction::R(val) => {
                let new_heading_degrees = (boat.heading as i32 - val) % 360;
                boat.heading = deg_to_heading(new_heading_degrees);
            },
            Instruction::F(val) => {
                match boat.heading {
                    Heading::North => { boat.y += val },
                    Heading::South => { boat.y -= val },
                    Heading::East => { boat.x += val },
                    Heading::West => { boat.x -= val },
                };
            },
        }
    }
    let manhattan_distance = boat.x.abs() + boat.y.abs();
    manhattan_distance
}

fn part2(instructions: &Vec<Instruction>) -> i32 {
    let mut boat = Boat { x: 0, y: 0, heading: Heading::East };
    let mut waypoint = Waypoint { x: 10, y: 1 };
    for instruction in instructions {
        match instruction {
            Instruction::N(val) => { waypoint.y += val },
            Instruction::S(val) => { waypoint.y -= val },
            Instruction::E(val) => { waypoint.x += val },
            Instruction::W(val) => { waypoint.x -= val },

            Instruction::L(val) => {
                // rotate number of times equal to right angles in degrees
                for _ in 0..(((val/90) % 4) + 4) % 4 {
                    let mut new_waypoint = Waypoint::default();
                    new_waypoint.x = -waypoint.y;
                    new_waypoint.y = waypoint.x;
                    waypoint = new_waypoint;
                }
            },
            Instruction::R(val) => {
                // rotate number of times equal to right angles in degrees
                for _ in 0..(((val/90) % 4) + 4) % 4 {
                    let mut new_waypoint = Waypoint::default();
                    new_waypoint.x = waypoint.y;
                    new_waypoint.y = -waypoint.x;
                    waypoint = new_waypoint;
                }
            },
            Instruction::F(val) => {
                for _ in 0..*val {
                    boat.x += waypoint.x;
                    boat.y += waypoint.y;
                }
            }
        }
    }
    let manhattan_distance = boat.x.abs() + boat.y.abs();
    manhattan_distance
}

fn deg_to_heading(deg: i32) -> Heading {
    let deg = ((deg % 360) + 360) % 360; // get the actual modulus, (not remainder)
    match deg {
        0 => Heading::East,
        90 => Heading::North,
        180 => Heading::West, 
        270 => Heading::South,
        _ => panic!("Degrees not a multiple of 90"),
    }
}
