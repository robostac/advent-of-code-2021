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

fn is_big_cave(s: &str) -> bool {
    s.chars().all(|x| x.is_ascii_lowercase()) == false
}

#[derive(Debug, Clone)]
struct Route {
    r: String,
    visited: HashSet<String>,
    current: String,
    second: bool,
}

impl Route {
    fn visit(&self, x: &str) -> Option<Route> {
        let mut second = self.second;
        if is_big_cave(x) == false {
            if self.visited.contains(x) {
                if second == false && (x != "start") {
                    second = true;
                } else {
                    return None;
                }
            }
        }
        let mut nr = self.clone();
        nr.visited.insert(x.to_owned());
        nr.r += x;
        nr.current = x.to_owned();
        nr.second = second;
        Some(nr)
    }

    fn new() -> Route {
        Route {
            r: String::new(),
            visited: HashSet::new(),
            current: String::new(),
            second: false,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<Vec<String>> = stdin
        .lock()
        .lines()
        .map(|input| input.unwrap().split("-").map(|x| parse_input(x)).collect())
        .collect();
    let mut routes = HashMap::new();
    for x in values.iter() {
        {
            let cc = routes.entry(x[0].clone()).or_insert(HashSet::new());
            cc.insert(x[1].clone());
        }
        {
            let cc = routes.entry(x[1].clone()).or_insert(HashSet::new());
            cc.insert(x[0].clone());
        }
    }

    let mut unique_routes = HashSet::new();
    let start = Route::new();
    let mut queue = VecDeque::new();
    queue.push_back(start.visit("start").unwrap());
    while queue.is_empty() == false {
        let v = queue.pop_front().unwrap();
        for dests in routes[&v.current].iter() {
            if dests == "end" {
                unique_routes.insert(v.r.to_owned());
            } else {
                match v.visit(dests) {
                    Some(p) => queue.push_back(p),
                    None => {}
                }
            }
        }
    }
    println!("{:?}", unique_routes.len());
}
