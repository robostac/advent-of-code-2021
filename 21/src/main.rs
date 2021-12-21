#![allow(dead_code, unused_macros, unused_imports)]

use core::panic;
use std::borrow::Borrow;
use std::collections::*;
use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::io::prelude::*;
use std::iter::Scan;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

fn parse_input<Input, Type>(input: Input) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
    Input: AsRef<str>,
{
    input.as_ref().trim().parse().unwrap()
}

fn count_possibilities() -> Vec<i64> {
    let mut poss = vec![0; 10];
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                poss[a + b + c] += 1;
            }
        }
    }

    poss
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn main() {
    let stdin = io::stdin();
    let values: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|input| {
            let s = input.unwrap();
            let (_, v) = s.split_once(": ").unwrap();
            parse_input(v)
        })
        .collect();

    let mut positions = values.clone();
    positions[0] -= 1;
    positions[1] -= 1;
    let mut scores = [0; 2];
    let mut curp = 0;
    let mut dice = 1;
    let mut rolls = 0;
    while scores[0] < 1000 && scores[1] < 1000 {
        let dist = dice * 3 + 3;
        dice += 3;
        positions[curp] = (positions[curp] + dist) % 10;
        scores[curp] += positions[curp] + 1;
        curp = curp ^ 1;
        rolls += 3;
    }
    println!("{:?} {:?} {}", positions, scores, rolls);
    println!("P1 Ans: {}", rolls * scores.iter().min().unwrap());

    let possibilities = count_possibilities();

    let mut states = HashMap::new();
    states.insert((values[0] - 1, 0, values[1] - 1, 0), 1);
    let mut curp = 0;
    let mut wins = [0i64; 2];
    while states.len() > 0 {
        let mut newstatemap = HashMap::new();
        for (state, universes) in states.iter() {
            for i in 3..=9 {
                let newcount = universes * possibilities[i as usize];
                let (mut p1pos, mut p1score, mut p2pos, mut p2score) = state;
                let newpos;
                let newscore;
                if curp == 0 {
                    newpos = &mut p1pos;
                    newscore = &mut p1score;
                } else {
                    newpos = &mut p2pos;
                    newscore = &mut p2score;
                }
                *newpos = (*newpos + i) % 10;
                *newscore = *newscore + *newpos + 1;
                if *newscore < 21 {
                    *newstatemap
                        .entry((p1pos, p1score, p2pos, p2score))
                        .or_insert(0) += newcount;
                } else {
                    wins[curp] += newcount;
                }
            }
        }
        states = newstatemap;
        curp ^= 1;
    }
    println!("{:?}", wins);
    println!("P2Ans {:?}", wins.iter().max().unwrap());
}
