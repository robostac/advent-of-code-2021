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

fn sum_all_values_between(start: i32, end: i32) -> i32 {
    let a = (end * (end + 1)) / 2;
    let b = (start * (start + 1)) / 2;
    a - b
}

fn is_valid_x(xbounds: &(i32, i32), initial: i32, time: i32) -> bool {
    let final_speed = std::cmp::max(0, initial - time);
    let final_pos = sum_all_values_between(final_speed, initial);
    final_pos >= xbounds.0 && final_pos <= xbounds.1
}

fn calc_y_time(ybounds: &(i32, i32), initial: i32) -> Option<(i32, i32)> {
    let mut time = 0;
    let mut speed = initial;
    if speed > 0 {
        time = speed * 2 + 1;
        speed = -speed - 1;
    }
    let mut pos = 0;
    let mut start = -1;

    while pos >= ybounds.0 {
        if start == -1 && pos <= ybounds.1 {
            start = time;
        }
        pos += speed;
        speed -= 1;
        time += 1;
    }
    if start == -1 {
        return None;
    }
    Some((start, time - 1))
}

fn find_all_velocites(xbound: &(i32, i32), ybound: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    for y in (ybound.0..=ybound.0.abs()).rev() {
        if let Some(b) = calc_y_time(&ybound, y) {
            for x in (0..=xbound.1).rev() {
                if (b.0..=b.1).any(|yt| is_valid_x(&xbound, x, yt)) {
                    v.push((x, y));
                }
            }
        }
    }
    v
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();

    let mut xbound = (0, 0);
    let mut ybound = (0, 0);
    for (i, v) in values[0].split("=").skip(1).enumerate() {
        let (c, _) = v.split_once(",").unwrap_or((v, ""));
        let (start, end) = c.split_once("..").unwrap();
        let v1: i32 = parse_input(start);
        let v2: i32 = parse_input(end);
        let bounds = (v1, v2);
        if i == 0 {
            xbound = bounds;
        } else {
            ybound = bounds;
        }
    }

    let valid_vel = find_all_velocites(&xbound, &ybound);

    let p1ans = valid_vel.iter().max_by_key(|x| x.1).unwrap();
    println!("{:?} ({:?})", sum_all_values_between(0, p1ans.1), p1ans);

    println!("{:?} ", valid_vel.len());
}
