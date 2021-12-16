#![allow(dead_code, unused_macros, unused_imports)]

use core::panic;
use std::collections::*;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Debug)]
enum PacketType {
    Literal(u64),
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

struct InputData {
    s: String,
    start: usize,
}

impl InputData {
    fn new(s: String) -> InputData {
        InputData { s, start: 0 }
    }

    fn extract_data(&mut self, bits: usize) -> u64 {
        let res = u64::from_str_radix(&self.s[self.start..(self.start + bits)], 2).unwrap();
        self.start += bits;
        res
    }

    fn extract_packets_by_length(&mut self, length: usize) -> Vec<Packet> {
        let mut v = Vec::new();
        let start = self.start;
        while (self.start - start) != length {
            v.push(Packet::parse_input(self));
        }
        v
    }

    fn extract_packets_by_count(&mut self, count: usize) -> Vec<Packet> {
        (0..count).map(|_| Packet::parse_input(self)).collect()
    }
}

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    sub: Vec<Packet>,
}

impl Packet {
    fn parse_input(input: &mut InputData) -> Packet {
        let version = input.extract_data(3) as u8;
        let ptype = input.extract_data(3) as u8;
        let packet_type;
        let sub;
        if ptype == 4 {
            let mut val: u64 = 0;
            loop {
                let t = input.extract_data(1);
                val += input.extract_data(4);
                if t > 0 {
                    val *= 16;
                } else {
                    break;
                }
            }
            sub = Vec::new();
            packet_type = PacketType::Literal(val);
        } else {
            let sptype = input.extract_data(1);
            if sptype == 0 {
                let bitlength = input.extract_data(15) as usize;
                sub = input.extract_packets_by_length(bitlength);
            } else {
                let pcount = input.extract_data(11) as usize;
                sub = input.extract_packets_by_count(pcount);
            }
            packet_type = match ptype {
                0 => PacketType::Sum,
                1 => PacketType::Product,
                2 => PacketType::Min,
                3 => PacketType::Max,
                5 => PacketType::Greater,
                6 => PacketType::Less,
                7 => PacketType::Equal,
                _ => panic!("{:?} unknown packet type", ptype),
            };
        }
        Packet {
            version,
            packet_type,
            sub,
        }
    }

    fn version_sum(&self) -> u64 {
        let mut value = self.version as u64;
        value += self.sub.iter().map(|x| x.version_sum()).sum::<u64>();
        value
    }

    fn value(&self) -> u64 {
        match self.packet_type {
            PacketType::Equal => (self.sub[0].value() == self.sub[1].value()) as u64,
            PacketType::Greater => (self.sub[0].value() > self.sub[1].value()) as u64,
            PacketType::Less => (self.sub[0].value() < self.sub[1].value()) as u64,
            PacketType::Literal(p) => p,
            PacketType::Max => self.sub.iter().map(|x| x.value()).max().unwrap(),
            PacketType::Min => self.sub.iter().map(|x| x.value()).min().unwrap(),
            PacketType::Product => self.sub.iter().map(|x| x.value()).product::<u64>(),
            PacketType::Sum => self.sub.iter().map(|x| x.value()).sum::<u64>(),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin
        .lock()
        .lines()
        .map(|input| {
            let mut s = String::new();
            for c in input.unwrap().chars() {
                s += &format!("{:04b}", c.to_digit(16).unwrap());
            }

            s
        })
        .collect();
    let mut input = InputData::new(values[0].clone());
    let p = input.extract_packets_by_count(1);

    println!("{:?}", p[0].version_sum());
    println!("{:?}", p[0].value());
}
