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

struct BingoBoard {
    mask: u32,
    numbers: Vec<i64>,
}

impl BingoBoard {
    fn parse_input(&mut self, s: &[String]) {
        for i in 0..5 {
            for p in s[i].split_whitespace() {
                self.numbers.push(parse_input(p));
            }
        }
    }

    fn new() -> BingoBoard {
        BingoBoard {
            mask: 0,
            numbers: Vec::new(),
        }
    }

    fn is_won(&self) -> bool {
        let mut horiz_mask = 0b11111;
        let mut vert_mask = 0b100001000010000100001;
        for _ in 0..5 {
            if (self.mask & horiz_mask) == horiz_mask {
                return true;
            }
            if (self.mask & vert_mask) == vert_mask {
                return true;
            }
            horiz_mask <<= 5;
            vert_mask <<= 1;
        }
        false
    }

    fn make_move(&mut self, m: i64) -> bool {
        if self.is_won() {
            return false;
        }
        for (p, _) in self.numbers.iter().enumerate().find(|(_, v)| **v == m) {
            self.mask |= 1 << p;
        }
        self.is_won()
    }

    fn unused_sum(&self) -> i64 {
        self.numbers
            .iter()
            .enumerate()
            .map(|(p, x)| if (self.mask & (1 << p)) > 0 { 0 } else { *x })
            .sum()
    }
}

fn winning_board_score(boards: &mut Vec<BingoBoard>, moves: &Vec<i64>) -> i64 {
    for x in moves.iter() {
        for b in boards.iter_mut() {
            if b.make_move(*x) {
                let val = *x * b.unused_sum();
                return val;
            }
        }
    }
    0
}

fn losing_board_score(boards: &mut Vec<BingoBoard>, moves: &Vec<i64>) -> i64 {
    let mut score = 0;
    for x in moves.iter() {
        for b in boards.iter_mut() {
            if b.make_move(*x) {
                let val = *x * b.unused_sum();
                score = val;
            }
        }
    }
    score
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let input_vals = values[0]
        .split(',')
        .map(|x| parse_input(x))
        .collect::<Vec<i64>>();
    let mut boards = Vec::new();
    for p in (2..values.len()).step_by(6) {
        let mut b = BingoBoard::new();
        b.parse_input(&values[p..(p + 5)]);
        b.make_move(input_vals[0]);
        boards.push(b);
    }

    let win = winning_board_score(&mut boards, &input_vals);

    println!("{}", win);

    let lose = losing_board_score(&mut boards, &input_vals);

    println!("{}", lose);
}
