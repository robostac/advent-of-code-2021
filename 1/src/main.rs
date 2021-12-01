#![allow(dead_code, unused_macros, unused_imports)]

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

fn increases(a: &Vec<i64>) -> usize {
    a.iter()
        .zip(a.iter().skip(1))
        .filter(|(x, y)| y > x)
        .count()
}

fn sliding_window(a: &Vec<i64>, w_size: usize) -> Vec<i64> {
    let mut sum = a[..w_size].iter().sum();
    let mut v = vec![sum];
    for (p, x) in a.iter().enumerate().skip(w_size) {
        sum += x;
        sum -= a[p - w_size];
        v.push(sum);
    }
    v
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|input| parse_input(input.unwrap()))
        .collect();

    println!("{}", increases(&values));

    println!("{}", increases(&sliding_window(&values, 3)));
}
