// Advent of code 2020
// day 2
// Eric Moss

use std::env;
use std::fs::read_to_string;
use std::ops::RangeInclusive;
use regex::Regex;

pub struct PasswordData {
    range: RangeInclusive<usize>,
    req_char: char,
    password: String,
}

fn main() {
    let filename = env::args().nth(1).expect("No filename given");
    let file_string = read_to_string(filename).unwrap();
    // define regex to extract data from lines
    let re: Regex = 
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>[a-z]): (?P<psswd>.*)$")
            .unwrap();
    // Create vector of password structs
    let mut passwords: Vec<PasswordData> = Vec::new();
    for line in file_string.lines() {
        let captures = re.captures(line).unwrap();
        let min = captures["min"].parse().unwrap();
        let max = captures["max"].parse().unwrap();
        let req_char = captures["char"].parse().unwrap();
        let password = captures["psswd"].parse().unwrap();

        let range = RangeInclusive::new(min,max);
        let new_pw_data = PasswordData { range, req_char, password };
        passwords.push(new_pw_data);
    }

    part1(&passwords);
    part2(&passwords);
}

fn part1(passwords: &Vec<PasswordData>) {
    let mut num_matches = 0;
    for pw in passwords {
        let num_req_chars = pw.password.chars()
                                       .filter(|x| x == &pw.req_char)
                                       .count();
        if pw.range.contains(&num_req_chars) {
            num_matches += 1;
        }
    }
    println!("{}", num_matches);
}

fn part2(passwords: &Vec<PasswordData>) {
    let mut num_matches = 0;
    for pw in passwords {
        let start = pw.range.start();
        let end = pw.range.end();
        let mut chars = pw.password.chars();
        let ch1 = chars.nth(start-1).unwrap();
        let ch2 = chars.nth((end-start)-1).expect(&format!("Failed on {}", pw.password));
        if (ch1 == pw.req_char) ^ (ch2 == pw.req_char) {
            num_matches += 1;
        }
    }
    println!("{}", num_matches);
}
