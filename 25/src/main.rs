use std::collections::*;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Default, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Square {
    Empty,
    East,
    South,
}

fn get_next(grid: &HashMap<Point, Square>, w: i64, h: i64) -> (HashMap<Point, Square>, usize) {
    let mut next_grid = HashMap::new();
    let mut moves = 0;
    for (p, v) in grid.iter() {
        if *v == Square::East {
            let mut next = Point::new((p.x + 1) % w, p.y);
            if grid.contains_key(&next) {
                next_grid.insert(*p, Square::East);
            } else {
                next_grid.insert(next, Square::East);
                moves += 1;
            }
        } else {
            next_grid.insert(*p, Square::South);
        }
    }
    let grid = next_grid;
    let mut next_grid = HashMap::new();

    for (p, v) in grid.iter() {
        if *v == Square::South {
            let mut next = Point::new(p.x, (p.y + 1) % h);
            if grid.contains_key(&next) {
                next_grid.insert(*p, Square::South);
            } else {
                next_grid.insert(next, Square::South);
                moves += 1;
            }
        } else {
            next_grid.insert(*p, Square::East);
        }
    }
    (next_grid, moves)
}
fn main() {
    let stdin = io::stdin();
    let values = stdin
        .lock()
        .lines()
        .map(|input| input.unwrap())
        .collect::<Vec<_>>();

    let mut grid = HashMap::new();
    let mut w = 0;
    let mut h = 0;
    for (y, s) in values.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            let p = Point::new(x as i64, y as i64);
            w = std::cmp::max(p.x, w);
            h = std::cmp::max(p.y, h);
            match v {
                '>' => {
                    grid.insert(p, Square::East);
                }
                'v' => {
                    grid.insert(p, Square::South);
                }
                _ => {}
            }
        }
    }
    w += 1;
    h += 1;

    for steps in 1.. {
        let (next, moves) = get_next(&grid, w, h);
        grid = next;
        if moves == 0 {
            println!("{}", steps);
            break;
        }
    }
}
