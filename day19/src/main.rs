// Advent of code 2020
// day 19
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use regex::{RegexSet, Regex};

// define regex constants
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RULE_RE: RegexSet = RegexSet::new(&[
        // Rule::Val
        r#"^"\S+"$"#,
        // Rule::Single
        r"^\d+$",
        // Rule::Double
        r"^\d+ \d+$",
        // Rule::Or
        r"^(\d+ ){1,2}|( \d+){1,2}$",
    ]).unwrap();
}

#[derive(Debug, Clone)]
enum Rule {
    Val(char),
    Single(usize),
    Double(usize, usize),
    Or(Box<Rule>, Box<Rule>),
    
    MemoRegex(String),
}

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());

    // parse rules
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    for line in &mut lines{
        // break when we get to end of rules section
        if line == "" {
            break;
        }
        let split_line: Vec<&str> = line.split(": ").collect();
        let rule_idx: usize = split_line[0].parse().expect("Unable to parse rule's index");
        let rule_str: &str = split_line[1];
        let rule: Rule = parse_rule_str(rule_str);

        rules.insert(rule_idx, rule);
    }

    // parse messages
    let mut messages: Vec<String> = Vec::new();
    for line in lines {
        messages.push(line);
    }

    println!("{}", part1(rules.clone(), &messages));
    println!("{}", part2(rules.clone(), &messages));
}

/*
fn part1(mut rules: HashMap<usize, Rule>, messages: &Vec<String>) -> usize {
    let all_strings: Vec<String> = evaluate_rule(0, &mut rules);
    let all_strings: HashSet<String> = HashSet::from_iter(all_strings);

    let mut num_matches: usize = 0;
    for message in messages {
        if all_strings.contains(message) {num_matches += 1};
    }
    num_matches
}
*/

/*
fn part2(mut rules: HashMap<usize, Rule>, messages: &Vec<String>) -> usize {
    let children_of_8: Vec<String> = evaluate_rule(8, &mut rules);
    let regex_of_8: Vec<String> = Vec::from_iter(children_of_8.iter()
                                                              .map(|s| format!("({})+", s)));
    //println!("{:?}", regex_of_8);
    let children_of_42: Vec<String> = evaluate_rule(42, &mut rules);
    let children_of_31: Vec<String> = evaluate_rule(31, &mut rules);
    // Generate every string in arbitrary recursion depth (choosing 4 here)
    let mut regex_of_11: Vec<String> = Vec::new();
    for i in 1..2 {
        let mut regex_at_depth: Vec<String> = children_of_42.iter()
                                                            .cartesian_product(children_of_31.iter())
                                                            .map(|(l,r)| format!("{}{{{depth}}}{}{{{depth}}}", l, r, depth = i))
                                                            .collect();
        regex_of_11.append(&mut regex_at_depth);
    }
    //println!("{:?}", regex_of_11.len());
    // create all regex of 8 AND 11
    let regex_of_0: Vec<String> = regex_of_8.iter()
                                            .cartesian_product(regex_of_11.iter())
                                            .map(|(l,r)| format!("^{}{}$", l, r))
                                            .collect();

    println!("Created vector");
    let regex_of_0: RegexSet = RegexSet::new(regex_of_0.iter()).unwrap();
    println!("{:?}", regex_of_0);
    5
}
*/
fn part1(rules: HashMap<usize, Rule>, messages: &Vec<String>) -> usize {
    let regex_string: Regex = Regex::new(format!("^{}$", regexify_rule(0, &mut rules.clone())).as_str())
                                    .unwrap();
    let mut num_matches: usize = 0;
    for message in messages {
        if regex_string.is_match(message) {num_matches += 1;}
    }
    num_matches
}

fn part2(rules: HashMap<usize, Rule>, messages: &Vec<String>) -> usize {
    /*
    let regex_8: Regex = Regex::new(format!("{}+", regexify_rule(42, &mut rules.clone())).as_str())
                                    .unwrap();
    println!("regex_8: {}", regex_8);
    */
    let regex_31: Regex = Regex::new(format!("{}", regexify_rule(31, &mut rules.clone())).as_str())
                                    .unwrap();
    let regex_42: Regex = Regex::new(format!("{}", regexify_rule(42, &mut rules.clone())).as_str())
                                    .unwrap();
    let mut regex_strings_11: Vec<String> = Vec::new();
    for i in 1..7 {
        //let regex_string = format!("{}+", regex_8);
        let regex_string = format!("({}){{{n}}}({}){{{n}}}", regex_42, regex_31, n = i);
        //println!("{}", regex_string);
        regex_strings_11.push(regex_string);
    }
    let regex_strings: Vec<String> = regex_strings_11.iter()
                                                     .map(|s| format!("^({})+{}$", regex_42, *s))
                                                     .collect();
    let regex_set: RegexSet = RegexSet::new(regex_strings.iter()).unwrap();

    let mut num_matches: usize = 0;
    for message in messages {
        if regex_set.is_match(message) {num_matches += 1};
    }
    num_matches
}

fn regexify_rule(rule_idx: usize, rules: &mut HashMap<usize, Rule>) -> String {
    let rule: &Rule = rules.get(&rule_idx)
                           .expect("Attempted to look up rule that doesn't exist");
    let evaluated: String;
    match rule {
        Rule::MemoRegex(m) => {
            // value has already been memoized. return immediately
            return m.clone();
        },
        // otherwise, calculate it and replace with memoized value
        Rule::Val(v) => {
            evaluated = v.to_string();
        }
        Rule::Single(child_idx) => {
            let child_evaluated = regexify_rule(*child_idx, rules);
            evaluated = child_evaluated;
        }
        Rule::Double(left_idx, right_idx) => {
            let left_idx = *left_idx;
            let right_idx = *right_idx;
            let left_evaluated = regexify_rule(left_idx, rules);
            let right_evaluated = regexify_rule(right_idx, rules);
            evaluated = format!("{}{}", left_evaluated, right_evaluated);
        }
        Rule::Or(left_rule, right_rule) => {
            let left_rule = left_rule.clone();
            let right_rule = right_rule.clone();
            let left_evaluated: String;
            let right_evaluated: String;
            match *left_rule {
                Rule::Single(child_idx) => {
                    left_evaluated = regexify_rule(child_idx, rules);
                },
                Rule::Double(left_idx, right_idx) => {
                    let left_idx = left_idx.clone();
                    let right_idx = right_idx.clone();
                    let sub_left_evaluated = regexify_rule(left_idx, rules);
                    let sub_right_evaluated = regexify_rule(right_idx, rules);
                    left_evaluated = format!("{}{}", sub_left_evaluated, sub_right_evaluated);
                },
                _ => {panic!("Or expression encountered with unexpected options");},
            }
            match *right_rule {
                Rule::Single(child_idx) => {
                    right_evaluated = regexify_rule(child_idx, rules);
                },
                Rule::Double(left_idx, right_idx) => {
                    let left_idx = left_idx.clone();
                    let right_idx = right_idx.clone();
                    let sub_left_evaluated = regexify_rule(left_idx, rules);
                    let sub_right_evaluated = regexify_rule(right_idx, rules);
                    right_evaluated = format!("{}{}", sub_left_evaluated, sub_right_evaluated);
                },
                _ => {panic!("Or expression encountered with unexpected options");},
            }
            evaluated = format!("({}|{})", left_evaluated, right_evaluated);
        }
    };
    let memoized_rule: Rule = Rule::MemoRegex(evaluated.clone());
    rules.insert(rule_idx, memoized_rule);
    evaluated
}

/*
fn evaluate_rule(rule_idx: usize, rules: &mut HashMap<usize, Rule>) -> Vec<String> {
    let rule: &Rule = rules.get(&rule_idx)
                           .expect("Attempted to look up rule that doesn't exist");
    let mut evaluated: Vec<String>;
    match rule {
        Rule::MemoRegex(_) => {
            panic!("Error, ran into regex memoized value in non-regex recursion");
        }
        Rule::Memo(m) => {
            // value has already been memoized. return immediately
            return m.clone();
        },
        // otherwise, calculate it and replace with memoized value
        Rule::Val(v) => {
            evaluated = vec![v.to_string()];
        }
        Rule::Single(child_idx) => {
            let child_evaluated = evaluate_rule(*child_idx, rules);
            evaluated = child_evaluated;
        }
        Rule::Double(left_idx, right_idx) => {
            let left_idx = *left_idx;
            let right_idx = *right_idx;
            let left_evaluated = evaluate_rule(left_idx, rules);
            let right_evaluated = evaluate_rule(right_idx, rules);
            evaluated = left_evaluated.iter()
                                      .cartesian_product(right_evaluated.iter())
                                      .map(|(l,r)| format!("{}{}", l, r))
                                      .collect();
        }
        Rule::Or(left_rule, right_rule) => {
            let left_rule = left_rule.clone();
            let right_rule = right_rule.clone();
            evaluated = Vec::new();
            for rule in [left_rule, right_rule].iter() {
                match **rule {
                    Rule::Single(child_idx) => {
                        let mut child_evaluated = evaluate_rule(child_idx, rules);
                        evaluated.append(&mut child_evaluated);
                    },
                    Rule::Double(left_idx, right_idx) => {
                        let left_idx = left_idx.clone();
                        let right_idx = right_idx.clone();
                        let left_evaluated = evaluate_rule(left_idx, rules);
                        let right_evaluated = evaluate_rule(right_idx, rules);
                        evaluated.append(&mut left_evaluated.iter()
                                                            .cartesian_product(right_evaluated.iter())
                                                            .map(|(l,r)| format!("{}{}", l, r))
                                                            .collect());
                    },
                    _ => {panic!("Or expression encountered with unexpected options");},
                }
            }
        }
    };
    let memoized_rule: Rule = Rule::Memo(evaluated.clone());
    rules.insert(rule_idx, memoized_rule);
    //println!("{:?}", evaluated);
    evaluated
}
*/

fn parse_rule_str(rule_str: &str) -> Rule {
    let rule_str = rule_str.trim();
    let rule_match: usize = RULE_RE.matches(rule_str)
                                   .into_iter()
                                   .next()
                                   .expect(format!("Unable to match \"{}\"", rule_str).as_str());
    // return match on this string
    match rule_match {
        0 => {
            // Rule::Val
            let val: char = rule_str.chars()
                                    .nth(1)
                                    .expect("Unable to parse char from value string");
            Rule::Val(val)
        }
        1 => {
            // Rule::Single
            let child: usize = rule_str.parse()
                                       .expect("Unable to parse child int value");
            Rule::Single(child)
        }
        2 => {
            // Rule::Double
            let mut split_str = rule_str.split(' ');
            let left_child: usize = split_str.next()
                                             .unwrap()
                                             .parse()
                                             .unwrap();
            let right_child: usize = split_str.next()
                                              .unwrap()
                                              .parse()
                                              .unwrap();
            Rule::Double(left_child, right_child)
        }
        3 => {
            // Rule::Or
            let mut split_str = rule_str.split('|');

            let left_rule_str = split_str.next().unwrap();
            let right_rule_str = split_str.next().unwrap();

            let left_rule: Box<Rule> = Box::new(parse_rule_str(left_rule_str));
            let right_rule: Box<Rule> = Box::new(parse_rule_str(right_rule_str));

            Rule::Or(left_rule, right_rule)
        }
        _ => {panic!("Error with regex matching");}
    }
}

