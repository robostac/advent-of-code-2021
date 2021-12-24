use core::panic;
use std::collections::*;
use std::fmt::Debug;
use std::io;
use std::io::prelude::*;
#[derive(Debug, Clone)]
enum Data {
    Literal(i64),
    Register(char),
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Data),
    Add(Data, Data),
    Mul(Data, Data),
    Div(Data, Data),
    Mod(Data, Data),
    Eql(Data, Data),
}

impl Data {
    fn from_string(s: &str) -> Data {
        let t = i64::from_str_radix(s, 10);
        if t.is_err() {
            Data::Register(s.chars().next().unwrap())
        } else {
            Data::Literal(t.unwrap())
        }
    }
}

impl Instruction {
    fn from_string(s: &str) -> Instruction {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "inp" => Instruction::Inp(Data::from_string(parts[1])),
            "mul" => Instruction::Mul(Data::from_string(parts[1]), Data::from_string(parts[2])),
            "add" => Instruction::Add(Data::from_string(parts[1]), Data::from_string(parts[2])),
            "div" => Instruction::Div(Data::from_string(parts[1]), Data::from_string(parts[2])),
            "mod" => Instruction::Mod(Data::from_string(parts[1]), Data::from_string(parts[2])),
            "eql" => Instruction::Eql(Data::from_string(parts[1]), Data::from_string(parts[2])),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct ALU {
    instructions: Vec<Vec<Instruction>>,
    registers: HashMap<char, i64>,
    pc: usize,
}

impl ALU {
    fn new() -> ALU {
        let mut al = ALU {
            instructions: Vec::new(),
            registers: HashMap::new(),
            pc: 0,
        };
        al.registers.insert('w', 0);
        al.registers.insert('x', 0);
        al.registers.insert('y', 0);
        al.registers.insert('z', 0);
        al
    }

    fn get_value(&self, d: &Data) -> i64 {
        match d {
            Data::Literal(p) => *p,
            Data::Register(a) => self.registers[a],
        }
    }

    fn step(&mut self, block_idx: usize) -> bool {
        let block = &self.instructions[block_idx];
        match &block[self.pc] {
            Instruction::Add(ad, bd) => {
                if let Data::Register(a) = ad {
                    *self.registers.entry(*a).or_default() += self.get_value(bd);
                }
            }
            Instruction::Div(ad, bd) => {
                if let Data::Register(a) = ad {
                    *self.registers.entry(*a).or_default() /= self.get_value(bd);
                }
            }
            Instruction::Mul(ad, bd) => {
                if let Data::Register(a) = ad {
                    *self.registers.entry(*a).or_default() *= self.get_value(bd);
                }
            }
            Instruction::Mod(ad, bd) => {
                if let Data::Register(a) = ad {
                    let av = self.get_value(ad);
                    *self.registers.entry(*a).or_default() = av % self.get_value(bd);
                }
            }
            Instruction::Eql(ad, bd) => {
                if let Data::Register(a) = ad {
                    let av = self.get_value(ad);
                    let bv = self.get_value(bd);

                    *self.registers.entry(*a).or_default() = if av == bv { 1 } else { 0 };
                }
            }
            Instruction::Inp(_) => {
                panic!();
            }
        }

        self.pc += 1;
        self.pc < block.len()
    }

    fn calc_val(&mut self, input: i64, z: i64, block_idx: usize) -> i64 {
        self.pc = 0;

        *self.registers.entry('z').or_default() = z;
        *self.registers.entry('w').or_default() = input;
        while self.step(block_idx) {}
        return self.registers[&'z'];
    }
}

fn is_valid(mut x: i64, idx: usize) -> bool {
    //not exact, but z is divided by 26 on some turns, otherwise increases and must be zero at the end
    for _ in idx..14 {
        x /= 26;
    }
    x == 0
}

fn part1_test(alu: &mut ALU, min: bool) {
    let mut hm = HashMap::new();
    hm.insert(0i64, 0i64);
    for idx in 0..14 {
        let mut nhm = HashMap::new();
        for (z, data) in hm.iter() {
            for i in 1..=9 {
                let res = alu.calc_val(i, *z, idx);
                if is_valid(res, idx) {
                    if min {
                        let c = nhm.entry(res).or_insert(std::i64::MAX);
                        *c = std::cmp::min(*c, data * 10 + i);
                    } else {
                        let c = nhm.entry(res).or_insert(0);
                        *c = std::cmp::max(*c, data * 10 + i);
                    }
                }
            }
        }
        hm = nhm;
        // println!("{:?} {:?}", idx, hm.len());
    }
    println!("{}", hm[&0]);
}

fn main() {
    let stdin = io::stdin();
    let values = stdin
        .lock()
        .lines()
        .map(|input| input.unwrap())
        .collect::<Vec<_>>();
    let mut inst = Vec::new();
    let mut cur = Vec::new();
    for x in values.iter() {
        let v = Instruction::from_string(x);
        if let Instruction::Inp(_) = v {
            if cur.len() > 0 {
                inst.push(cur);
            };
            cur = Vec::new();
        } else {
            cur.push(v);
        }
    }
    if cur.len() > 0 {
        inst.push(cur);
    }

    let mut alu = ALU::new();
    alu.instructions = inst;

    part1_test(&mut alu, false);
    part1_test(&mut alu, true);
}
