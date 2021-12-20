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

fn print_image(grid: &HashMap<(i32, i32), bool>, xbounds: (i32, i32), ybounds: (i32, i32)) {
    for y in ybounds.0..=ybounds.1 {
        for x in xbounds.0..=xbounds.1 {
            let c;
            let value = grid.get(&(x, y)).unwrap_or(&false);
            if *value {
                c = '#';
            } else {
                c = '.';
            }
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn get_value(grid: &HashMap<(i32, i32), bool>, x: i32, y: i32, filler: bool) -> usize {
    let mut t = 0;
    let mut extra = 1 << 8;
    for y_off in -1..=1 {
        for x_off in -1..=1 {
            if *grid.get(&(x + x_off, y + y_off)).unwrap_or(&filler) {
                t += extra;
            }
            extra >>= 1;
        }
    }
    t
}

fn enhance_image(
    grid: &HashMap<(i32, i32), bool>,
    xbounds: &(i32, i32),
    ybounds: &(i32, i32),
    enhance: &Vec<bool>,
    filler: bool,
) -> HashMap<(i32, i32), bool> {
    let mut new_grid = HashMap::new();

    for y in ybounds.0..=ybounds.1 {
        for x in xbounds.0..=xbounds.1 {
            let p = (x, y);
            let idx = get_value(grid, x, y, filler);
            new_grid.insert(p, enhance[idx]);
        }
    }
    new_grid
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();

    let enhance: Vec<_> = values[0].chars().map(|x| x == '#').collect();

    let mut grid = HashMap::new();
    for (y, s) in values[2..].iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            let p = (x as i32, y as i32);

            grid.insert(p, v == '#');
        }
    }
    let num_iterations = 50;
    let mut xbounds = (-2, grid.keys().max_by_key(|x| x.0).unwrap().0 + 2);
    let mut ybounds = (-2, grid.keys().max_by_key(|x| x.1).unwrap().1 + 2);
    xbounds.0 -= num_iterations + 1;
    xbounds.1 += num_iterations + 1;
    ybounds.0 -= num_iterations + 1;
    ybounds.1 += num_iterations + 1;
    let mut filler = false;
    for _ in 0..2 {
        grid = enhance_image(&grid, &xbounds, &ybounds, &enhance, filler);
        if enhance[0] {
            filler = !filler;
        }
    }
    println!("{}", grid.values().filter(|x| **x).count());

    for _ in 2..num_iterations {
        grid = enhance_image(&grid, &xbounds, &ybounds, &enhance, filler);
        if enhance[0] {
            filler = !filler;
        }
    }
    println!("{}", grid.values().filter(|x| **x).count());
}
