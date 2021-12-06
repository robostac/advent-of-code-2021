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

fn main() {
    let stdin = io::stdin();
    let values: Vec<Vec<i64>> = stdin
        .lock()
        .lines()
        .map(|input| input.unwrap().split(",").map(|x| parse_input(x)).collect())
        .collect();

    const P1DAYS: usize = 80;
    const P2DAYS: usize = 256;
    let mut nfish = values[0].len();
    let mut spawns = [0; P2DAYS + 10];
    for p in values[0].iter() {
        spawns[*p as usize] += 1;
    }
    for x in 0..=P2DAYS {
        if x == P1DAYS || x == P2DAYS {
            println!("{}", nfish);
        }
        nfish += spawns[x];
        spawns[x + 9] += spawns[x];
        spawns[x + 7] += spawns[x];
    }
}
