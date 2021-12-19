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

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Default)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    fn rotate_x(&mut self) {
        let y = self.y;
        self.y = -self.z;
        self.z = y;
    }

    fn rotate_y(&mut self) {
        let x = self.x;
        self.x = -self.z;
        self.z = x;
    }

    fn rotate_z(&mut self) {
        let x = self.x;
        self.x = -self.y;
        self.y = x;
    }

    fn total(&self) -> i64 {
        self.x + self.y + self.z
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
    id: usize,
    location: Point,
}

struct ScannerIter {
    it: Scanner,
    pos: usize,
}

impl Iterator for ScannerIter {
    type Item = Scanner;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 24 {
            return None;
        }
        for p in self.it.beacons.iter_mut() {
            if self.pos == 8 || self.pos == 16 {
                p.rotate_z();
            }
            if self.pos == 16 {
                p.rotate_y();
            }
            if self.pos % 4 == 0 {
                p.rotate_y();
                p.rotate_y();
            }
            p.rotate_x();
        }
        self.pos += 1;

        Some(self.it.clone())
    }
}

impl Scanner {
    fn parse_input(input: &str, id: usize) -> Scanner {
        let mut beacons = Vec::new();
        for x in input.split("\n").skip(1) {
            let c: Vec<_> = x.split(',').collect();
            beacons.push(Point::new(
                parse_input(c[0]),
                parse_input(c[1]),
                parse_input(c[2]),
            ));
        }
        Scanner {
            beacons,
            id,
            location: Default::default(),
        }
    }

    fn rotations(&self) -> ScannerIter {
        ScannerIter {
            it: self.clone(),
            pos: 0,
        }
    }

    fn find_match(&self, other: &HashSet<Point>, min_count: usize) -> Option<Point> {
        for b in other.iter() {
            for ob in self.beacons.iter().skip(min_count - 1) {
                let count = self
                    .beacons
                    .iter()
                    .filter(|x| other.contains(&(**x - *ob + *b)))
                    .count();
                if count >= min_count {
                    return Some(*b - *ob);
                }
            }
        }
        None
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer);
    let values: Vec<String> = buffer.split("\n\n").map(|x| x.to_owned()).collect();

    let scanners: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(p, s)| Scanner::parse_input(s, p))
        .collect();

    let mut matched = Vec::new();
    matched.push(scanners[0].clone());

    let mut valid_beacons: HashSet<Point> = scanners[0].beacons.iter().cloned().collect();
    let mut rotated_scanners = Vec::new();
    for sc in &scanners[1..] {
        rotated_scanners.extend(sc.rotations());
    }
    let mut scanners = rotated_scanners;
    while scanners.is_empty() == false {
        let mut id = None;
        for sc in scanners.iter_mut() {
            if let Some(offset) = sc.find_match(&valid_beacons, 12) {
                sc.location = offset;
                sc.beacons = sc.beacons.iter().map(|x| *x + offset).collect();
                valid_beacons.extend(sc.beacons.iter());
                println!("{:?} {:?}", sc.id, sc.location);
                matched.push(sc.clone());
                id = Some(sc.id);
                break;
            }
        }
        if let Some(mid) = id {
            scanners.retain(|x| x.id != mid);
        } else {
            panic!("NO MATCH");
        }
    }
    println!("{:?}", valid_beacons.len());

    let mut max = 0;
    for sc1 in matched.iter() {
        for sc2 in matched.iter() {
            let dist = (sc1.location - sc2.location).total();
            max = std::cmp::max(max, dist);
        }
    }
    println!("{:?}", max);
}
