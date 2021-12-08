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

fn create_mapping(s: &str) -> HashMap<char, u8> {
    let mut hm = HashMap::new();
    let mut counter = HashMap::new();
    let diffs: Vec<String> = s.split(' ').map(|x| x.to_owned()).collect();
    for x in s.chars() {
        if x == ' ' {
            continue;
        }
        *counter.entry(x).or_insert(0) += 1;
    }
    let one = diffs.iter().find(|x| x.len() == 2).unwrap().to_owned();
    let four = diffs.iter().find(|x| x.len() == 4).unwrap().to_owned();
    for (k, v) in counter.iter() {
        if *v == 4 {
            hm.insert(*k, 4);
        } else if *v == 6 {
            hm.insert(*k, 1);
        } else if *v == 8 {
            if one.contains(*k) {
                hm.insert(*k, 2);
            } else {
                hm.insert(*k, 0);
            }
        } else if *v == 9 {
            hm.insert(*k, 5);
        } else if *v == 7 {
            if four.contains(*k) {
                hm.insert(*k, 3);
            } else {
                hm.insert(*k, 6);
            }
        }
    }
    hm
}

fn decode(s: &str, mapping: &HashMap<char, u8>) -> u32 {
    let mut out = String::new();
    for x in s.split_whitespace() {
        let mut value = 0;
        for c in x.chars() {
            value |= 1 << mapping[&c];
        }
        out += match value {
            0b1110111 => "0",
            0b0100100 => "1",
            0b1011101 => "2",
            0b1101101 => "3",
            0b0101110 => "4",
            0b1101011 => "5",
            0b1111011 => "6",
            0b0100101 => "7",
            0b1111111 => "8",
            0b1101111 => "9",
            _ => panic!("unknown display value {}", value),
        };
    }
    parse_input(out)
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<(String, String)> = stdin
        .lock()
        .lines()
        .map(|input| {
            let i = input.unwrap();
            let s: Vec<&str> = i.split(" | ").collect();
            (s[0].to_owned(), s[1].to_owned())
        })
        .collect();
    let mut counter = 0;
    for x in values.iter() {
        counter +=
            x.1.split(' ')
                .filter(|x| x.len() < 5 || x.len() > 6)
                .count();
    }
    println!("{}", counter);

    let mut total = 0;
    for x in values.iter() {
        let mapping = create_mapping(&x.0);
        let value = decode(&x.1, &mapping);
        total += value;
    }
    println!("{}", total);
}
