#![allow(dead_code, unused_macros, unused_imports)]

use core::panic;
use std::collections::*;
use std::io;
use std::io::prelude::*;

use std::fmt::Debug;
use std::str::FromStr;

fn parse_input<Input, Type>(input: Input) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
    Input: AsRef<str>,
{
    input.as_ref().trim().parse().unwrap()
}

enum AnswerType {
    Syntax(i64),
    Incomplete(i64),
}

fn brackets_match(start: char, end: char) -> bool {
    match start {
        '(' => end == ')',
        '[' => end == ']',
        '{' => end == '}',
        '<' => end == '>',
        _ => panic!("invalid bracket {}", start),
    }
}

fn find_error(s: &str) -> AnswerType {
    let mut stack = Vec::new();
    for x in s.chars() {
        if x == '(' || x == '<' || x == '[' || x == '{' {
            stack.push(x);
        } else {
            let c = stack.pop().unwrap_or(' ');
            if brackets_match(c, x) {
                continue;
            }
            return match x {
                ')' => AnswerType::Syntax(3),
                ']' => AnswerType::Syntax(57),
                '}' => AnswerType::Syntax(1197),
                '>' => AnswerType::Syntax(25137),
                _ => panic!("invalid bracket {}", x),
            };
        }
    }
    AnswerType::Incomplete(auto_complete(&stack))
}

fn auto_complete(s: &Vec<char>) -> i64 {
    let mut score = 0;
    for x in s.iter().rev() {
        let val = match x {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("invalid bracket {}", x),
        };
        score = (score * 5) + val;
    }
    score
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();

    let mut errors = 0;
    let mut completions = Vec::new();
    for s in values.iter() {
        match find_error(s) {
            AnswerType::Incomplete(p) => completions.push(p),
            AnswerType::Syntax(p) => errors += p,
        }
    }

    println!("{:?}", errors);
    completions.sort();
    println!("{:?}", completions[completions.len() / 2]);
}
