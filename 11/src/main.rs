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

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn update_grid(grid: &mut HashMap<(i32, i32), i32>) -> i32 {
    let mut queue = VecDeque::new();
    for (k, v) in grid.iter_mut() {
        *v += 1;
        if *v > 9 {
            queue.push_back(k.clone());
        }
    }
    let mut flashed = HashSet::new();
    while queue.is_empty() == false {
        let p = queue.pop_front().unwrap();
        if flashed.insert(p) == false {
            continue;
        }
        for x in DIRECTIONS.iter() {
            let np = (p.0 + x.0, p.1 + x.1);
            if flashed.contains(&np) || (grid.contains_key(&np) == false) {
                continue;
            }
            let cc = grid.entry(np).or_default();
            *cc += 1;
            if *cc > 9 {
                queue.push_back(np);
            }
        }
    }
    for x in flashed.iter() {
        *grid.entry(*x).or_default() = 0;
    }
    flashed.len() as i32
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
    let mut flashes = 0;
    let mut p1ans = 0;
    let mut p2ans = 0;
    for step in 1.. {
        let nflashes = update_grid(&mut grid);
        flashes += nflashes;
        if step == 100 {
            p1ans = flashes;
        }
        if nflashes == grid.len() as i32 {
            p2ans = step;
            break;
        }
    }
    println!("{:?}", p1ans);
    println!("{:?}", p2ans);
}
