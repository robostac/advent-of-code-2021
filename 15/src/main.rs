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

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn find_risk_path(grid: &HashMap<(i32, i32), i32>, w: i32, h: i32) -> i32 {
    let start = (0, 0);
    let mut mins = HashMap::new();
    let mut explored = VecDeque::new();
    explored.push_back(start);
    mins.insert(start, 0);
    while let Some(pos) = explored.pop_front() {
        let risk = mins[&pos];
        for d in DIRECTIONS.iter() {
            let np = (d.0 + pos.0, d.1 + pos.1);
            match grid.get(&np) {
                None => {}
                Some(extra_risk) => {
                    let new_risk = risk + extra_risk;
                    let mr = mins.entry(np).or_insert(std::i32::MAX);
                    if new_risk < *mr {
                        *mr = new_risk;
                        explored.push_back(np);
                    }
                }
            }
        }
    }
    mins[&(w - 1, h - 1)]
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();

    let mut grid = HashMap::new();
    for (y, s) in values.iter().enumerate() {
        for (x, c) in s.chars().enumerate() {
            let p = (x as i32, y as i32);
            grid.insert(p, c.to_digit(10).unwrap() as i32);
        }
    }
    let w = values[0].len() as i32;
    let h = values.len() as i32;
    let p1ans = find_risk_path(&grid, w, h);

    println!("{:?}", p1ans);

    let nw = w * 5;
    let nh = h * 5;
    for x in 0..nw {
        for y in 0..nh {
            let inc = x / w + y / h;
            let mut new_risk = grid[&(x % w, y % h)] + inc;
            while new_risk > 9 {
                new_risk -= 9;
            }
            grid.insert((x, y), new_risk);
        }
    }
    let p2ans = find_risk_path(&grid, nw, nh);

    println!("{:?}", p2ans);
}
