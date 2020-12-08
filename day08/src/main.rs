// Advent of code 2020
// day 8
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


// Types of instructions supported by machine
#[derive(Debug, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

#[derive(Default, Clone)]
struct Thread {
    text: Vec<Instruction>,
    pc: usize,
    acc: isize,
}

// runs next instruction in thread
fn execute(thread: &mut Thread) {
    match thread.text[thread.pc] {
        Instruction::Nop(_) => thread.pc += 1,
        Instruction::Acc(arg) => {
            thread.acc += arg;
            thread.pc += 1;
        },
        Instruction::Jmp(arg) => {
            // scary casts between signed and unsigned are unfortunately needed
            thread.pc = ((thread.pc as isize) + arg) as usize
        },
    }
}

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut thread = Thread::default();
    for line in reader.lines().map(|l| l.unwrap()){
        let mut words = line.split_whitespace();
        let op: &str = words.next()
                            .expect("Error in reading op");
        let arg: isize = words.next()
                              .expect("Error in reading argument")
                              .parse()
                              .expect("Unable to parse argument to int");
        let instruction = match op {
            "nop" => Instruction::Nop(arg),
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            _ => panic!("Attempted to parse unknown operation"),
        };
        thread.text.push(instruction);
    }

    println!("{}", part1(thread.clone()));
    println!("{}", part2(thread.clone()).expect("Got no result for part 2"));
}

fn part1(mut thread: Thread) -> isize {
    let mut successors: HashMap<usize, usize> = HashMap::new();
    // execute until current pc is a value it has been before
    let mut pc_before = thread.pc;
    while !successors.contains_key(&thread.pc) {
        execute(&mut thread);
        successors.insert(pc_before, thread.pc);
        pc_before = thread.pc;
    }
    thread.acc
}

fn part2(thread: Thread) -> Option<isize> {
    for (i, instruction) in thread.text.iter().enumerate() {
        let mut modified_thread = thread.clone();
        // swap nop and jmp
        modified_thread.text[i] = match instruction {
            Instruction::Nop(arg) => Instruction::Jmp(*arg),
            Instruction::Jmp(arg) => Instruction::Nop(*arg),
            _ => continue
        };
        let mut successors: HashMap<usize, usize> = HashMap::new();
        // execute until loop or complete
        let mut pc_before: usize;
        loop {
            pc_before = modified_thread.pc;
            execute(&mut modified_thread);
            if successors.contains_key(&modified_thread.pc) {
                // we've just looped
                break;
            }
            if modified_thread.pc >= modified_thread.text.len() {
                // thread completed execution
                return Some(modified_thread.acc);
            }
            successors.insert(pc_before, modified_thread.pc);
        }
    }
    None
}
