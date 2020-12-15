// Advent of code 2020
// day 14
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

enum Instruction {
    Mem { addr: u64, val: u64 },
    Mask { zero_mask: u64, one_mask: u64 },
}

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut line = line.split(" = ");
        let instr = line.next().unwrap();
        let val = line.next().unwrap();
        match instr {
            "mask" => {
                let zero_mask: String = val.chars()
                                           .map(|c| if c == '0' {'1'} else {'0'})
                                           .collect();
                let zero_mask = u64::from_str_radix(zero_mask.as_str(), 2).unwrap();
                let one_mask: String = val.chars()
                                           .map(|c| if c == '1' {'1'} else {'0'})
                                           .collect();
                let one_mask = u64::from_str_radix(one_mask.as_str(), 2).unwrap();
                instructions.push(Instruction::Mask { zero_mask, one_mask });
            },
            _ => {
                let addr = &instr[4..instr.len()-1];
                let addr: u64 = addr.parse().unwrap();
                let val: u64 = val.parse().unwrap();
                instructions.push(Instruction::Mem { addr, val });
            },
        }
    }

    println!("{}", part1(&instructions));
    //println!("{}", part2(&bus_ids));
}

fn part1(instructions: &Vec<Instruction>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_zero_mask: u64 = 0;
    let mut current_one_mask: u64 = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Mem { addr, val } => {
                // apply zero mask with val & (val NAND mask)
                let mut mapped_val = val & (!current_zero_mask);
                mapped_val = mapped_val | current_one_mask;
                memory.insert(*addr, mapped_val);
            },
            Instruction::Mask { zero_mask, one_mask } => {
                current_zero_mask = *zero_mask;
                current_one_mask = *one_mask;
            },
        }
    }
    memory.values().fold(0, |acc, x| acc + x)
}

