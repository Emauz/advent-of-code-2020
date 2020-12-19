// Advent of code 2020
// day 18
// Eric Moss

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

// expressions evaluate to a value
#[derive(Debug)]
enum Expression {
    Const(u64),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

fn main() {
    // initialize file IO
    let filename = env::args().nth(1)
                              .expect("No filename given");
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut input: Vec<String> = Vec::new();
    // parse all lines into operation trees
    for line in reader.lines().map(|l| l.unwrap()) {
        let line: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        input.push(line);
    }

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn evaluate_expression(expr: &Expression) -> u64 {
    let evaluated = match expr {
        Expression::Const(c) => *c,
        Expression::Add(l, r) => {
            evaluate_expression(&*l) + evaluate_expression(&*r)
        }
        Expression::Mul(l, r) => {
            evaluate_expression(&*l) * evaluate_expression(&*r)
        }
    };
    evaluated
}

fn parse_tokens(tokens: &mut VecDeque<char>) -> Box<Expression> {
    // parse from right to left, so that expression gets evaluated left to right
    let right_expr: Box<Expression> = match tokens.pop_back().unwrap() {
        ')' => {
            parse_tokens(tokens)
        },
        c => {
            let const_value: u64 = c.to_digit(10)
                                    .expect("Unable to convert const to integer")
                                    as u64;
            Box::new(Expression::Const(const_value))
        },
    };
    // Base case: Parsed entire problem
    if tokens.is_empty() {
        return right_expr;
    }

    let arithmetic_operation: char = tokens.pop_back().expect("Ran into unexpected operation token");
    // Base case: Evaluated entire block
    if arithmetic_operation == '(' {
        return right_expr;
    }

    // construct aritmhetic expression and return
    match arithmetic_operation {
        '+' => {
            let left_expr: Box<Expression> = parse_tokens(tokens);
            Box::new(Expression::Add(left_expr, right_expr))
        },
        '*' => {
            let left_expr: Box<Expression> = parse_tokens(tokens);
            Box::new(Expression::Mul(left_expr, right_expr))
        },
        _ => {
            panic!("Parsed unexpected arithmetic operation");
        },
    }


}

fn parenthesize_multiplication(tokens: &mut VecDeque<char>) -> VecDeque<char> {
    loop {
        // continue parenthesizing multiplication until none are left
        match tokens.iter().position(|c| *c == '+') {
            Some(idx) => {
                let mut l_idx = idx - 1;
                let mut r_idx = idx + 1;
                tokens[idx] = 't';
                // insert right parenthesis
                let mut right_depth = 0;
                loop {
                    if tokens[r_idx] == '(' {
                        right_depth += 1;
                    }
                    if tokens[r_idx] == ')' {
                        right_depth -= 1;
                    }
                    if right_depth <= 0 {
                        // insert parenthesis here!
                        tokens.insert(r_idx+1, ')');
                        break;
                    }
                    r_idx += 1;
                }
                // insert left parenthesis
                let mut left_depth = 0;
                loop {
                    if tokens[l_idx] == ')' {
                        left_depth += 1;
                    }
                    if tokens[l_idx] == '(' {
                        left_depth -= 1;
                    }
                    if left_depth <= 0 {
                        // insert parenthesis here!
                        tokens.insert(l_idx, '(');
                        break;
                    }
                    l_idx -= 1;
                }
            }
            None => {
                break;
            }
        }
    }
    // return all addition signs to a + (they've been replaced with 't')
    let tokens: VecDeque<char> = tokens.iter().map(|c| if *c == 't' {'+'} else {*c}).collect();
    tokens
}

fn part1(input: &Vec<String>) -> u64 {
    let mut homework: Vec<Expression> = Vec::new();
    for equation in input {
        let tokens: VecDeque<char> = equation.chars().collect();
        let parsed_expression = parse_tokens(&mut tokens.clone());
        homework.push(*parsed_expression);
    }
    let mut homework_sum = 0;
    for problem in homework.iter() {
        homework_sum += evaluate_expression(problem);
    }
    homework_sum
}

fn part2(input: &Vec<String>) -> u64 {
    let mut homework: Vec<Expression> = Vec::new();
    for equation in input {
        let mut tokens: VecDeque<char> = equation.chars().collect();
        // parenthesize multiplication first!
        tokens = parenthesize_multiplication(&mut tokens);
        let parsed_expression = parse_tokens(&mut tokens.clone());
        homework.push(*parsed_expression);
    }
    let mut homework_sum = 0;
    for problem in homework.iter() {
        homework_sum += evaluate_expression(problem);
    }
    homework_sum
}
