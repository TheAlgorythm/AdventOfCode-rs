use std::collections::BTreeMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct BitMask {
    pattern: u64,
    value: u64,
}

impl BitMask {
    fn mask(&self, value: u64) -> u64 {
        (value & !self.pattern) | self.value
    }
}

impl FromStr for BitMask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mask = s.replace("mask = ", "");
        Ok(BitMask {
            pattern: u64::from_str_radix(&mask.replace('0', "1").replace('X', "0"), 2)?,
            value: u64::from_str_radix(&mask.replace('X', "0"), 2)?,
        })
    }
}

#[derive(Debug)]
enum Transaction {
    BitMask(BitMask),
    MemSet(u64, u64),
}

impl FromStr for Transaction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "mask" => Ok(Transaction::BitMask(s.parse::<BitMask>()?)),
            "mem[" => {
                let elements: Vec<String> = s
                    .replace("mem[", "")
                    .splitn(2, "] = ")
                    .map(|s| s.to_string())
                    .collect();
                Ok(Transaction::MemSet(
                    elements[0].parse().expect("Can't parse index!"),
                    elements[1].parse().expect("Can't parse value!"),
                ))
            }
            _ => unreachable!(),
        }
    }
}

fn parse_mask_mem(input: &str) -> Vec<Transaction> {
    input
        .lines()
        .map(|line| {
            line.parse::<Transaction>()
                .expect("Couldn't parse Transaction!")
        })
        .collect()
}

fn memory_sum(transactions: &Vec<Transaction>) -> u64 {
    let mut mask = BitMask {
        pattern: 0,
        value: 0,
    };
    let mut memory: BTreeMap<u64, u64> = BTreeMap::new();
    transactions
        .iter()
        .for_each(|transaction| match transaction {
            Transaction::BitMask(new_mask) => mask = new_mask.clone(),
            Transaction::MemSet(address, value) => {
                let masked_value = mask.mask(*value);
                memory
                    .entry(*address)
                    .and_modify(|val| *val = masked_value)
                    .or_insert(masked_value);
            }
        });
    memory.iter().map(|(_address, value)| *value).sum()
}

fn solve_part_one(transactions: &Vec<Transaction>) {
    let mem_sum = memory_sum(&transactions);
    println!("The memory residue summed up is {}.", mem_sum);
}

fn solve_part_two() {}

fn main() {
    let input = include_str!("14_data.txt");

    let transactions = parse_mask_mem(&input);

    solve_part_one(&transactions);
    solve_part_two();
}
