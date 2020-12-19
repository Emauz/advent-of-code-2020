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

    let mut homework: VecDeque<Expression> = VecDeque::new();
    // parse all lines into operation trees
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut tokens: VecDeque<char> = line.chars().filter(|c| !c.is_whitespace()).collect();
        let parsed_expression = parse_tokens(&mut tokens);
        homework.push_back(*parsed_expression);
    }

    println!("{}", part1(&homework));
    //println!("{}", part2(&space_4d));
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

    let left_expr: Box<Expression> = parse_tokens(tokens);

    // construct aritmhetic expression and return
    match arithmetic_operation {
        '+' => {
            Box::new(Expression::Add(left_expr, right_expr))
        },
        '*' => {
            Box::new(Expression::Mul(left_expr, right_expr))
        },
        _ => {panic!("Parsed unexpected arithmetic operation");},
    }
}



fn part1(homework: &VecDeque<Expression>) -> u64 {
    let mut homework_sum = 0;
    for problem in homework.iter() {
        homework_sum += evaluate_expression(problem);
    }
    homework_sum
}

