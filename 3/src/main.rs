#![allow(dead_code, unused_macros, unused_imports)]

use core::panic;
use std::collections::*;
use std::io;
use std::io::prelude::*;

use std::fmt::Debug;
use std::iter::FromIterator;
use std::str::FromStr;

fn parse_input<Input, Type>(input: Input) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
    Input: AsRef<str>,
{
    input.as_ref().trim().parse().unwrap()
}

fn has_more_ones(values: &Vec<Vec<char>>, p: usize) -> bool {
    let ones = values.iter().filter(|x| x[p] == '1').count();
    let zeros = values.len() - ones;
    ones >= zeros
}

fn part_2_detect(values: &Vec<Vec<char>>, most: bool) -> usize {
    let len = values[0].len();
    let mut values = values.clone();
    for p in 0..len {
        let keep = has_more_ones(&values, p) == most;
        values.retain(|x| ((x[p] == '1') == keep));
        if values.len() == 1 {
            break;
        }
    }
    let s: String = values[0].iter().collect();
    usize::from_str_radix(&s, 2).unwrap()
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|input| input.unwrap().chars().collect())
        .collect();
    let len = values[0].len();
    let mut gamma = 0;
    for p in 0..len {
        if has_more_ones(&values, p) {
            gamma += 2usize.pow((len - p - 1) as u32);
        }
    }
    let epsilon = (2usize.pow(len as u32) - 1) ^ gamma;
    println!("{} {} {}", gamma, epsilon, gamma * epsilon);

    let oxy = part_2_detect(&values, true);
    let co2 = part_2_detect(&values, false);
    println!("{} {} {}", oxy, co2, oxy * co2);
}
