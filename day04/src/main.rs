// Advent of code 2020
// day 4
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::str::FromStr;

static NUM_FIELDS: usize = 8;

#[derive(PartialEq, Eq, Hash)]
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

    println!("{}", part1(passports));
    //println!("{}", part2());
}

fn part1(passports: Vec<HashMap<Field, String>>) -> usize {
    let mut num_valid = 0;
    for passport in passports {
        // check if passport has all required fields
        let mut existing_fields = passport.keys().len();
        if passport.contains_key(&Field::CountryID) {
            existing_fields -= 1;
        }
        if existing_fields == (NUM_FIELDS - 1) {
            num_valid += 1;
        }
    }
    num_valid
}
