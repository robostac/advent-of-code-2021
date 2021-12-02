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
    let values: Vec<(String, i64)> = stdin
        .lock()
        .lines()
        .map(|input| {
            let input = input.unwrap();
            let s: Vec<&str> = input.split(' ').collect();
            (parse_input(s[0]), parse_input(s[1]))
        })
        .collect();

    let mut pos = 0;
    let mut depth = 0;
    for (dir, offset) in values.iter() {
        match dir.as_str() {
            "forward" => pos += offset,
            "down" => depth += offset,
            "up" => depth -= offset,
            p => panic!("unknown dir {}", p),
        }
    }

    // println!("{:?}", values);
    println!("{} {} {}", depth, pos, depth * pos);

    //part 2
    let mut aim = 0;
    let mut pos = 0;
    let mut depth = 0;
    for (dir, offset) in values.iter() {
        match dir.as_str() {
            "forward" => {
                pos += offset;
                depth += aim * offset;
            }
            "down" => aim += offset,
            "up" => aim -= offset,
            p => panic!("unknown dir {}", p),
        }
    }

    // println!("{:?}", values);
    println!("{} {} {}", depth, pos, depth * pos);
}
