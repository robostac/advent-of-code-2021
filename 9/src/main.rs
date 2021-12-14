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

fn calc_basin(grid: &HashMap<(i32, i32), u32>, p: (i32, i32)) -> usize {
    let mut basin = HashSet::new();
    basin.insert(p);
    let mut current = VecDeque::new();
    current.push_back(p);

    while current.is_empty() == false {
        let p = current.pop_front().unwrap();
        for d in DIRECTIONS.iter() {
            let nd = (p.0 + d.0, p.1 + d.1);
            let v = *grid.get(&nd).unwrap_or(&9);
            if v != 9 && basin.insert(nd) {
                current.push_back(nd);
            }
        }
    }

    basin.len()
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let mut grid = HashMap::new();
    for (y, s) in values.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            let p = (x as i32, y as i32);
            let val = v.to_digit(10).unwrap();
            grid.insert(p, val);
        }
    }
    let low_points: Vec<_> = grid
        .iter()
        .filter(|(p, v)| {
            for d in DIRECTIONS.iter() {
                let nd = (p.0 + d.0, p.1 + d.1);
                if grid.get(&nd).unwrap_or(&10) <= *v {
                    return false;
                }
            }
            true
        })
        .collect();
    let p1ans: u32 = low_points.iter().map(|x| x.1 + 1).sum();
    println!("{:?}", p1ans);

    let mut basins: Vec<_> = low_points.iter().map(|x| calc_basin(&grid, *x.0)).collect();
    basins.sort();
    let p2ans: usize = basins[(basins.len() - 3)..].iter().product();
    println!("{:?}", p2ans);
}
