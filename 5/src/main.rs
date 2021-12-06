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

struct SubLine {
    start: (i64, i64),
    end: (i64, i64),
}

impl SubLine {
    fn parse_input(&mut self, s: &str) {
        let s = s.replace(" -> ", ",");
        let vals = s.split(",").map(|x| parse_input(x)).collect::<Vec<i64>>();
        self.start = (vals[0], vals[1]);
        self.end = (vals[2], vals[3]);
    }

    fn new() -> SubLine {
        SubLine {
            start: (0, 0),
            end: (0, 0),
        }
    }

    fn grad(&self) -> (i64, i64) {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        if dy == 0 {
            return (dx.signum(), 0);
        } else if dx == 0 {
            return (0, dy.signum());
        } else {
            //problem statement says lines will be 45 degree if diagonal
            return (dx.signum(), dy.signum());
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<SubLine> = stdin
        .lock()
        .lines()
        .map(|input| {
            let mut s = SubLine::new();
            s.parse_input(&input.unwrap());
            s
        })
        .collect();
    let mut p1counter = HashMap::new();
    let mut p2counter = HashMap::new();
    for p in values.iter() {
        let mut s = p.start;
        let grad = p.grad();
        loop {
            if grad.0 == 0 || grad.1 == 0 {
                *p1counter.entry(s).or_insert(0) += 1;
            }
            *p2counter.entry(s).or_insert(0) += 1;
            if s == p.end {
                break;
            }
            s.0 += grad.0;
            s.1 += grad.1;
        }
    }
    let ans_part_1 = p1counter.values().filter(|k| **k >= 2).count();
    println!("{}", ans_part_1);
    let ans_part_2 = p2counter.values().filter(|k| **k >= 2).count();
    println!("{}", ans_part_2);
}
