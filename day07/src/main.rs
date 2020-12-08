// Advent of code 2020
// day 7
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};

#[derive(Default)]
pub struct Bag {
    parents: HashSet<(String, String)>,
    children: HashSet<(usize, (String, String))>,
}

// Macro to create a tuple of next two strings in an interator.
// note: also consumes following entry in iterator (useless in input)
// 
// Takes: iterator
// Returns: (String, String)
// usage: create_desc_tuple!(iterator);
macro_rules! create_desc_tuple {
    ( $iter:expr ) => {{
        let adj: String = $iter.next()
                       .unwrap()
                       .to_string();
        let color: String = $iter.next()
                         .unwrap()
                         .to_string();
        $iter.next();
        (adj, color)
    }};
}

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    // initialize struct to hold bag relations
    let mut bags: HashMap<(String, String), Bag> = HashMap::new();
    for line in reader.lines().map(|l| l.unwrap()){
        let mut words = line.split_whitespace();
        let parent_desc = create_desc_tuple!(words);
        words.next(); // skips word "contain(s)" in input.

        // for each parent bag, add to current bag as parent,
        // and add child to parent
        loop {
            // parse number
            let num_bags: &str = match words.next() {
                Some(bags) => bags,
                None => break, // reached end of string
            };
            let num_bags: usize = match num_bags.parse() {
                Ok(number) => number,
                Err(_) => break, // bag contains no other bags
            };
            let child_desc = create_desc_tuple!(words);
            // add parent to child
            {
                let child_bag = bags.entry(child_desc.clone()).or_default();
                child_bag.parents.insert(parent_desc.clone());
                //println!("parent->child: {:?} {:?}", child_desc, parent_desc);
            }
            // add child to parent
            {
                let parent_bag = bags.entry(parent_desc.clone()).or_default();
                parent_bag.children.insert((num_bags, child_desc.clone()));
                //println!("child->parent: {:?} {:?}", parent_desc, child_desc);
            }
        }
    }

    let shiny_gold = ("shiny".to_string(), "gold".to_string());
    let part1_result = part1(&bags, shiny_gold.clone());
    println!("{}", part1_result.len());
    println!("{}", part2(&bags, shiny_gold.clone()));
}

fn part1(bags: &HashMap<(String, String), Bag>, desc: (String, String)) -> HashSet<(String, String)> {
    let my_bag = bags.get(&desc).expect("Unable to find bag with given description");
    let mut output_set: HashSet<(String, String)> = my_bag.parents.clone();
    for parent in my_bag.parents.iter() {
        output_set.extend(part1(bags, parent.clone()));
    }
    output_set
}

// calculates number of required child bags within each bag
fn part2(bags: &HashMap<(String, String), Bag>, desc: (String, String)) -> usize { 
    let my_bag = bags.get(&desc).expect("Unable to find bag with given description");
    let mut sum: usize = 0;
    for (num, child) in my_bag.children.iter() {
        sum += num * (1 + part2(bags, child.clone()))
    }
    sum
}
