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

fn calc_new_pos(old_pos: i32, fold_pos: i32) -> i32 {
    if old_pos < fold_pos {
        return old_pos;
    }
    fold_pos - (old_pos - fold_pos)
}

fn fold(current_paper: &HashSet<(i32, i32)>, pos: i32, horiz: bool) -> HashSet<(i32, i32)> {
    let mut new_paper = HashSet::new();
    for p in current_paper.iter() {
        if horiz {
            new_paper.insert((calc_new_pos(p.0, pos), p.1));
        } else {
            new_paper.insert((p.0, calc_new_pos(p.1, pos)));
        }
    }
    new_paper
}

fn draw_paper(current_paper: &HashSet<(i32, i32)>) {
    let w = current_paper.iter().max_by_key(|x| x.0).unwrap().0;
    let h = current_paper.iter().max_by_key(|x| x.1).unwrap().1;
    for y in 0..=h {
        for x in 0..=w {
            let p = (x, y);
            if current_paper.contains(&p) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();

    let mut current_paper = HashSet::new();
    for x in values.iter().filter(|x| x.contains(",")) {
        let c: Vec<&str> = x.split(",").collect();
        let p: (i32, i32) = (parse_input(c[0]), parse_input(c[1]));
        current_paper.insert(p);
        // println!("{:?}", x);/
    }

    for (i, x) in values.iter().filter(|x| x.contains("=")).enumerate() {
        let c: Vec<&str> = x.split("=").collect();
        let pos: i32 = parse_input(c[1]);
        let horiz = c[0].chars().last().unwrap() == 'x';
        current_paper = fold(&current_paper, pos, horiz);
        if i == 0 {
            println!("{:?}", current_paper.len());
        }
    }
    draw_paper(&current_paper);
}
