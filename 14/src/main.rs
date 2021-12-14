#![allow(dead_code, unused_macros, unused_imports)]

use core::panic;
use std::collections::*;
use std::io;
use std::io::prelude::*;
use std::io::LineWriter;

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

type PolymerPair = (char, char);

fn expand(
    polymer: &HashMap<PolymerPair, u64>,
    rules: &HashMap<PolymerPair, char>,
) -> HashMap<PolymerPair, u64> {
    let mut newp = HashMap::new();
    for (&pp, v) in polymer.iter() {
        match rules.get(&pp) {
            Some(&p) => {
                let p1 = (pp.0, p);
                *newp.entry(p1).or_insert(0) += v;
                let p2 = (p, pp.1);
                *newp.entry(p2).or_insert(0) += v;
            }
            None => {
                *newp.entry(pp).or_insert(0) += v;
            }
        }
    }

    newp
}

fn calc_answer(polymer: &HashMap<PolymerPair, u64>) -> u64 {
    let mut counter = HashMap::new();
    for (x, v) in polymer.iter() {
        *counter.entry(x.0).or_insert(0) += v;
    }
    let most_common = counter.values().max().unwrap();
    let least_common = counter.values().min().unwrap();

    most_common - least_common
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let pchars: Vec<char> = values[0].chars().collect();
    let mut polymer = HashMap::new();
    for (i, c) in pchars.iter().enumerate() {
        let p = (*c, *pchars.get(i + 1).unwrap_or(&' '));
        *polymer.entry(p).or_insert(0) += 1;
    }
    let mut rules = HashMap::new();
    for x in &values[2..] {
        let c: Vec<char> = x.chars().collect();
        rules.insert((c[0], c[1]), *c.last().unwrap());
    }
    for _ in 0..10 {
        polymer = expand(&polymer, &rules);
    }
    println!("{}", calc_answer(&polymer));
    for _ in 10..40 {
        polymer = expand(&polymer, &rules);
    }
    println!("{}", calc_answer(&polymer));
}
