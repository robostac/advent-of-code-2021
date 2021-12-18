#![allow(dead_code, unused_macros, unused_imports)]

use std::collections::*;
use std::io;
use std::io::prelude::*;

use std::fmt::Debug;
use std::ops::Not;
use std::panic;
use std::str::FromStr;

fn parse_input<Input, Type>(input: Input) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
    Input: AsRef<str>,
{
    input.as_ref().trim().parse().unwrap()
}

struct InputData {
    s: Vec<char>,
    start: usize,
}

impl InputData {
    fn new(s: String) -> InputData {
        InputData {
            s: s.chars().collect(),
            start: 0,
        }
    }

    fn extract_number(&mut self) -> u64 {
        let mut val: u64 = 0;
        while self.start < self.s.len() && (self.s[self.start] != ',' && self.s[self.start] != ']')
        {
            val *= 10;
            val += self.s[self.start].to_digit(10).unwrap() as u64;
            self.start += 1;
        }
        val
    }

    fn skip(&mut self) {
        self.start += 1;
    }

    fn get_next(&mut self) -> NodeContents {
        if self.s[self.start] != '[' {
            NodeContents::Literal(self.extract_number())
        } else {
            NodeContents::SnailFish(Box::new(SnailFish::extract_from_input(self)))
        }
    }
}

#[derive(Debug, Clone)]
enum NodeContents {
    Literal(u64),
    SnailFish(Box<SnailFish>),
}

#[derive(Debug, Default, Clone)]
struct SnailFish {
    left: NodeContents,
    right: NodeContents,
}

impl Default for NodeContents {
    fn default() -> Self {
        NodeContents::Literal(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExplodeType {
    None,
    Explode((u64, u64)),
    Prop((Option<u64>, Option<u64>)),
}

impl SnailFish {
    fn add(&self, other: &SnailFish) -> SnailFish {
        SnailFish {
            left: NodeContents::SnailFish(Box::new((*self).clone())),
            right: NodeContents::SnailFish(Box::new((*other).clone())),
        }
    }

    fn debug(&self) -> String {
        format!("[{},{}]", self.left.debug(), self.right.debug())
    }

    fn extract_from_input(input: &mut InputData) -> SnailFish {
        let left;
        let right;
        input.skip(); //'['
        left = input.get_next();
        input.skip(); //","
        right = input.get_next();
        input.skip(); //"]"
        SnailFish { left, right }
    }

    fn add_number(&mut self, p: u64, towards_left: bool) {
        let node = if towards_left {
            &mut self.right
        } else {
            &mut self.left
        };
        match *node {
            NodeContents::Literal(ref mut val) => *val += p,
            NodeContents::SnailFish(ref mut x) => x.add_number(p, towards_left),
        }
    }

    fn check_explosions_dir(&mut self, depth: u64, left: bool) -> ExplodeType {
        let node = if left {
            &mut self.left
        } else {
            &mut self.right
        };
        match *node {
            NodeContents::Literal(_) => return ExplodeType::None,
            NodeContents::SnailFish(ref mut sf) => match sf.check_explosions(depth + 1) {
                ExplodeType::None => return ExplodeType::None,
                ExplodeType::Explode((l, r)) => {
                    *node = NodeContents::Literal(0);
                    return self.explode_prop(Some(l), Some(r), left);
                }
                ExplodeType::Prop((l, r)) => return self.explode_prop(l, r, left),
            },
        }
    }

    fn explode_prop(&mut self, l: Option<u64>, r: Option<u64>, from_left: bool) -> ExplodeType {
        if from_left {
            if let Some(r) = r {
                match self.right {
                    NodeContents::Literal(val) => {
                        self.right = NodeContents::Literal(val + r);
                    }
                    NodeContents::SnailFish(ref mut x) => x.add_number(r, false),
                }
            }
            return ExplodeType::Prop((l, None));
        } else {
            if let Some(l) = l {
                match self.left {
                    NodeContents::Literal(val) => self.left = NodeContents::Literal(val + l),
                    NodeContents::SnailFish(ref mut x) => x.add_number(l, true),
                }
            }
            return ExplodeType::Prop((None, r));
        }
    }

    fn check_explosions(&mut self, depth: u64) -> ExplodeType {
        if depth >= 4 {
            return ExplodeType::Explode((self.left.get_value(), self.right.get_value()));
        }
        let x = self.check_explosions_dir(depth, true);
        if x != ExplodeType::None {
            return x;
        }
        self.check_explosions_dir(depth, false)
    }

    fn check_split(&mut self) -> bool {
        for node in [&mut self.left, &mut self.right].iter_mut() {
            match **node {
                NodeContents::Literal(p) => {
                    if p >= 10 {
                        **node = NodeContents::SnailFish(Box::new(SnailFish::new_pair(
                            p / 2,
                            (p + 1) / 2,
                        )));
                        return true;
                    }
                }
                NodeContents::SnailFish(ref mut x) => {
                    if x.check_split() {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn reduce(&mut self) {
        loop {
            if self.check_explosions(0) == ExplodeType::None {
                if self.check_split() == false {
                    break;
                }
            }
        }
    }

    fn new_pair(x: u64, y: u64) -> SnailFish {
        SnailFish {
            left: NodeContents::Literal(x),
            right: NodeContents::Literal(y),
        }
    }

    fn magnitude(&self) -> u64 {
        return self.left.get_magnitude() * 3 + self.right.get_magnitude() * 2;
    }

    fn add_reduce(&self, other: &SnailFish) -> SnailFish {
        let mut newsf = self.add(other);
        newsf.reduce();
        newsf
    }
}

impl NodeContents {
    fn get_value(&self) -> u64 {
        match self {
            NodeContents::Literal(p) => *p,
            NodeContents::SnailFish(_) => panic!("not a literal"),
        }
    }

    fn get_magnitude(&self) -> u64 {
        match self {
            NodeContents::Literal(val) => *val,
            NodeContents::SnailFish(ref x) => x.magnitude(),
        }
    }

    fn debug(&self) -> String {
        match self {
            NodeContents::Literal(p) => format!("{}", p),
            NodeContents::SnailFish(ref x) => x.debug(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|input| input.unwrap()).collect();
    let sf: Vec<SnailFish> = values
        .iter()
        .map(|x| {
            let mut input = InputData::new(x.clone());
            SnailFish::extract_from_input(&mut input)
        })
        .collect();

    let mut answer = sf[0].clone();

    for extra in sf[1..].iter() {
        answer = answer.add_reduce(extra);
    }
    println!("{}", answer.debug());

    println!("{}", answer.magnitude());

    let mut max = 0;
    for (i, sf1) in sf.iter().enumerate() {
        for sf2 in sf[(i + 1)..].iter() {
            max = std::cmp::max(max, sf1.add_reduce(sf2).magnitude());
            max = std::cmp::max(max, sf2.add_reduce(sf1).magnitude());
        }
    }
    println!("{}", max);
}
