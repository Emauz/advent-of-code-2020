// Advent of code 2020
// day 4
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

static NUM_FIELDS: usize = 7;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

// enables creating Field enum value from raw string
impl FromStr for Field {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Field, Self::Err> {
        match s {
            "byr" => Ok(Field::BirthYear),
            "iyr" => Ok(Field::IssueYear),
            "eyr" => Ok(Field::ExpirationYear),
            "hgt" => Ok(Field::Height),
            "hcl" => Ok(Field::HairColor),
            "ecl" => Ok(Field::EyeColor),
            "pid" => Ok(Field::PassportID),
            "cid" => Ok(Field::CountryID),
            _ => Err("Unable to convert string to field value"),
        }
    }
}

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    // initialize struct to hold passport data
    let mut passports: Vec<HashMap<Field, String>> = Vec::new();
    let mut current_passport: HashMap<Field, String> = HashMap::new();
    for line in reader.lines().map(|l| l.unwrap()){
        if line.is_empty() {
            // current passport data is complete. push and make a new one.
            // but first, remove country ID, it's not relevant.
            current_passport.remove(&Field::CountryID);
            passports.push(current_passport);
            current_passport = HashMap::new();
        } else {
            // find all datapoints in line and add to current passport
            let datapoints = line.split_whitespace();
            for data in datapoints {
                // parse key and value from datapoint
                let mut data = data.split(':');
                let key = data.next()
                              .unwrap();
                let key = Field::from_str(key)
                                .unwrap();
                let value = data.next()
                                .unwrap()
                                .to_string();
                current_passport.insert(key, value);
            }
        }
    }

    println!("{}", part1(&passports));
    println!("{}", part2(&passports));
}

fn part1(passports: &Vec<HashMap<Field, String>>) -> usize {
    let mut num_valid = 0;
    for passport in passports {
        // check if passport has all required fields
        let existing_fields = passport.keys().len();
        if existing_fields == (NUM_FIELDS) {
            num_valid += 1;
        }
    }
    num_valid
}

fn part2(passports: &Vec<HashMap<Field, String>>) -> usize {
    let mut num_valid = 0;
    for passport in passports {
        if check_validity(&passport){
            num_valid += 1;
        }
    }
    num_valid
}

fn check_validity(passport: &HashMap<Field, String>) -> bool {
    // check if all fields are valid, return false if any fields fail.
    for (key, val) in passport.iter() {
        match key {
            Field::BirthYear => {
                if val.len() != 4 {
                    return false
                }
                let year: usize = match val.parse() {
                    Ok(year) => year,
                    Err(_) => return false,
                };
                if (year > 2002) || (year < 1920) {
                    return false
                }
            }
            Field::IssueYear => {
                if val.len() != 4 {
                    return false
                }
                let year: usize = match val.parse() {
                    Ok(year) => year,
                    Err(_) => return false,
                };
                if (year > 2020) || (year < 2010) {
                    return false
                }
            }
            Field::ExpirationYear => {
                if val.len() != 4 {
                    return false
                }
                let year: usize = match val.parse() {
                    Ok(year) => year,
                    Err(_) => return false
                };
                if (year > 2030) || (year < 2020) {
                    return false
                }
            }
            Field::Height => {
                let re = Regex::new(r"^([0-9]+)(in|cm)$").unwrap();
                let caps = match re.captures(val){
                    Some(caps) => caps,
                    None => return false,
                };
                let number: usize = caps.get(1)
                                        .unwrap()
                                        .as_str()
                                        .parse()
                                        .unwrap();
                let unit: &str = caps.get(2)
                                    .unwrap()
                                    .as_str();
                match unit {
                    "cm" => if (number < 150) || (number > 193) {
                        return false
                    },
                    "in" => if (number < 59) || (number > 76) {
                        return false
                    },
                    _ => return false
                }
            }
            Field::HairColor => {
                let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                if !re.is_match(val) {
                    return false;
                }
            }
            Field::EyeColor => {
                let valid_colors: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                if !valid_colors.contains(&val.as_str()) {
                    return false
                }
            }
            Field::PassportID => {
                let re = Regex::new(r"^[0-9]{9}$").unwrap();
                if !re.is_match(val) {
                    return false;
                }
            }
            _ => ()
        }
    }
    // Check if # fields invalid
    let existing_fields = passport.keys().len();
    if existing_fields != (NUM_FIELDS) {
        return false
    }
    true
}
