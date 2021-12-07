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

fn sub_move_p1(init_pos: &Vec<i64>, final_pos: i64) -> i64 {
    init_pos.iter().map(|x| (x - final_pos).abs()).sum()
}

fn sub_move_p2(init_pos: &Vec<i64>, final_pos: i64) -> i64 {
    init_pos
        .iter()
        .map(|x| {
            let n = (x - final_pos).abs();
            (n * (n + 1)) / 2
        })
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<Vec<i64>> = stdin
        .lock()
        .lines()
        .map(|input| input.unwrap().split(",").map(|x| parse_input(x)).collect())
        .collect();

    let mut hpos = values[0].clone();
    hpos.sort();
    let median = hpos[hpos.len() / 2];
    let ans = sub_move_p1(&hpos, median);
    println!("{:?}", ans);

    let count = hpos.len() as i64;
    let total: i64 = hpos.iter().sum();
    let mean = (total) / count;
    println!(
        "{:?}",
        std::cmp::min(sub_move_p2(&hpos, mean), sub_move_p2(&hpos, mean + 1))
    );
}
