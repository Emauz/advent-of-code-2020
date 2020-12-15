// Advent of code 2020
// day 14
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

enum Instruction {
    Mem { addr: u64, val: u64 },
    Mask { zero_mask: u64, one_mask: u64, x_mask: u64 },
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
                let x_mask: String = val.chars()
                                        .map(|c| if c == 'X' {'1'} else {'0'})
                                        .collect();
                let x_mask = u64::from_str_radix(x_mask.as_str(), 2).unwrap();
                instructions.push(Instruction::Mask { zero_mask, one_mask, x_mask });
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
    println!("{}", part2(&instructions));
}

fn part1(instructions: &Vec<Instruction>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_zero_mask: u64 = 0;
    let mut current_one_mask: u64 = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Mem { addr, val } => {
                let mut mapped_val = val & !current_zero_mask;
                mapped_val = mapped_val | current_one_mask;
                memory.insert(*addr, mapped_val);
            },
            Instruction::Mask { zero_mask, one_mask, .. } => {
                current_zero_mask = *zero_mask;
                current_one_mask = *one_mask;
            },
        }
    }
    memory.values().fold(0, |acc, x| acc + x)
}

fn get_floating_addrs(masked_addr: u64, x_mask: u64) -> Vec<u64> {
    // base case, no floating bits
    if x_mask == 0 {
        return vec![masked_addr];
    }
    let mut i = 0;
    let mut shifty_x_mask = x_mask;
    while shifty_x_mask % 2 != 1 {
        i += 1;
        shifty_x_mask = shifty_x_mask >> 1;
    }
    let lsb_x_mask = 1 << i;
    let new_x_mask = x_mask & !lsb_x_mask;
    let mut output: Vec<u64> = Vec::new();
    //get_floating_addrs(masked_addr & !lsb_x_mask, xm1);
    let zero_masked = masked_addr & !lsb_x_mask;
    let one_masked = masked_addr | lsb_x_mask;
    output.append(&mut get_floating_addrs(zero_masked, new_x_mask));
    output.append(&mut get_floating_addrs(one_masked, new_x_mask));
    output
}

fn part2(instructions: &Vec<Instruction>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_one_mask: u64 = 0;
    let mut current_x_mask: u64 = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Mem { addr, val } => {
                let masked_addr: u64 = addr | current_one_mask;
                for floated_addr in get_floating_addrs(masked_addr, current_x_mask) {
                    memory.insert(floated_addr, *val);
                }
            },
            Instruction::Mask { zero_mask: _ , one_mask, x_mask } => {
                current_one_mask = *one_mask;
                current_x_mask = *x_mask;
            },
        }
    }
    memory.values().fold(0, |acc, x| acc + x)
}

