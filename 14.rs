use std::collections::BTreeMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct BitMask {
    pattern: u64,
    value: u64,
    memory_mutations: Vec<u64>,
}

impl BitMask {
    fn new(pattern: u64, value: u64) -> Self {
        let memory_mutations = mutate_with_or(
            (0..36)
                .map(|exponent| 2_u64.pow(exponent))
                .filter(|position| (position & pattern) != 0)
                .collect(),
        );
        BitMask {
            pattern,
            value,
            memory_mutations,
        }
    }

    fn mask(&self, value: u64) -> u64 {
        (value & self.pattern) | self.value
    }

    fn decode_memory_address(&self, virtual_address: u64) -> Vec<u64> {
        let fixed_address = (virtual_address & !self.pattern) | self.value;
        self.memory_mutations
            .iter()
            .map(|mutation| mutation | fixed_address)
            .collect()
    }
}

impl FromStr for BitMask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mask = s.replace("mask = ", "");
        Ok(BitMask::new(
            u64::from_str_radix(&mask.replace('1', "0").replace('X', "1"), 2)?,
            u64::from_str_radix(&mask.replace('X', "0"), 2)?,
        ))
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

fn mutate_with_or(linear: Vec<u64>) -> Vec<u64> {
    let mut results = Vec::new();
    mutate_with_or_internal(0, linear, &mut results);
    results
}

fn mutate_with_or_internal(last: u64, mut linear: Vec<u64>, mut results: &mut Vec<u64>) {
    if linear.len() == 0 {
        results.push(last);
        return;
    }
    let current = linear.pop().expect("Empty linear!");
    mutate_with_or_internal(last, linear.to_vec(), &mut results);
    mutate_with_or_internal(last | current, linear, &mut results);
}

fn memory_sum(transactions: &Vec<Transaction>) -> u64 {
    let mut mask = BitMask::new(0, 0);
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

fn memory_sum_with_mad(transactions: &Vec<Transaction>) -> u64 {
    let mut mask = BitMask::new(0, 0);
    let mut memory: BTreeMap<u64, u64> = BTreeMap::new();
    transactions
        .iter()
        .for_each(|transaction| match transaction {
            Transaction::BitMask(new_mask) => mask = new_mask.clone(),
            Transaction::MemSet(address, value) => {
                mask.decode_memory_address(*address)
                    .iter()
                    .for_each(|physical_address| {
                        memory
                            .entry(*physical_address)
                            .and_modify(|val| *val = value.clone())
                            .or_insert(value.clone());
                    });
            }
        });
    memory.iter().map(|(_address, value)| *value).sum()
}

fn solve_part_two(transactions: &Vec<Transaction>) {
    let mem_sum = memory_sum_with_mad(&transactions);
    println!(
        "The memory residue in memory address decoder mode summed up is {}.",
        mem_sum
    );
}

fn main() {
    let input = include_str!("14_data.txt");

    let transactions = parse_mask_mem(&input);

    solve_part_one(&transactions);
    solve_part_two(&transactions);
}
