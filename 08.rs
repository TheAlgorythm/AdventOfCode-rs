use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseInstructionError {
    UnknownInstruction,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParseInstructionError {
    fn from(err: ParseIntError) -> ParseInstructionError {
        ParseInstructionError::ParseInt(err)
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    NoOperation(i64),
    Jump(i64),
    Accumulate(i64),
}

impl Instruction {
    fn is_control_flow(&self) -> bool {
        match self {
            Instruction::Accumulate(_) => false,
            _ => true,
        }
    }

    fn negate_control_flow(&mut self) {
        match self {
            Instruction::NoOperation(val) => *self = Instruction::Jump(*val),
            Instruction::Jump(val) => *self = Instruction::NoOperation(*val),
            _ => panic!("Can only negate NoOP & Jump!"),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements: Vec<&str> = s.splitn(2, ' ').collect();

        let number: i64 = elements[1].parse()?;

        match elements[0] {
            "nop" => Ok(Instruction::NoOperation(number)),
            "jmp" => Ok(Instruction::Jump(number)),
            "acc" => Ok(Instruction::Accumulate(number)),
            _ => Err(ParseInstructionError::UnknownInstruction),
        }
    }
}

fn parse_asm(input: &str) -> Result<Vec<Instruction>, ParseInstructionError> {
    input.lines().map(str::parse::<Instruction>).collect()
}

#[derive(Debug)]
enum RuntimeError {
    InfiniteLoop(Vec<usize>, i64),
    SegmentationFault(usize),
}

fn run(instructions: &Vec<Instruction>) -> Result<i64, RuntimeError> {
    let return_instruction = instructions.len();
    let mut instruction_stack: Vec<usize> = Vec::new();
    let mut accumulator = 0_i64;
    let mut instruction_pointer = 0_usize;
    loop {
        if instruction_pointer == return_instruction {
            break;
        }
        if instruction_stack.contains(&instruction_pointer) {
            return Err(RuntimeError::InfiniteLoop(instruction_stack, accumulator));
        }
        let instruction = match instructions.get(instruction_pointer) {
            Some(instruction) => instruction,
            None => return Err(RuntimeError::SegmentationFault(instruction_pointer)),
        };
        instruction_stack.push(instruction_pointer);
        match instruction {
            Instruction::NoOperation(_) => instruction_pointer += 1,
            Instruction::Jump(offset) => {
                instruction_pointer = (instruction_pointer as i128 + *offset as i128) as usize
            }
            Instruction::Accumulate(value) => {
                accumulator += value;
                instruction_pointer += 1
            }
        }
    }
    Ok(accumulator)
}

fn solve_part_one(res: &Result<i64, RuntimeError>) {
    match res {
        Ok(accumulator) => println!("Returned with value {}.", accumulator),
        Err(RuntimeError::InfiniteLoop(_stacktrace, last_accumulator)) => println!(
            "There was an infinite loop with the last value {}.",
            last_accumulator
        ),
        Err(_) => println!("Other error!"),
    }
}

fn backtrace_infinite_loop(
    instructions: &Vec<Instruction>,
    stacktrace: &Vec<usize>,
) -> Result<(usize, i64), ()> {
    for instruction_index in stacktrace.iter().rev() {
        if !instructions[*instruction_index].is_control_flow() {
            continue;
        }
        let mut updated_instructions: Vec<Instruction> = instructions.to_vec();
        updated_instructions[*instruction_index].negate_control_flow();
        if let Ok(accumulator) = run(&updated_instructions) {
            return Ok((*instruction_index, accumulator));
        }
    }
    Err(())
}

fn solve_part_two(res: &Result<i64, RuntimeError>, instructions: &Vec<Instruction>) {
    let stacktrace = match res {
        Err(RuntimeError::InfiniteLoop(stacktrace, _last_accumulator)) => stacktrace,
        _ => {
            println!("No infinite loop detected!");
            return;
        }
    };
    match backtrace_infinite_loop(&instructions, &stacktrace) {
        Ok((negated_instruction, accumulator_result)) => println!(
            "Negating instruction {}, there is the result {}.",
            negated_instruction, accumulator_result
        ),
        Err(_) => println!("There is no way of stopping the infinite loop!"),
    }
}

fn main() -> Result<(), ParseInstructionError> {
    let input = include_str!("08_data.asm");

    let instructions = parse_asm(&input)?;

    let res = run(&instructions);

    solve_part_one(&res);
    solve_part_two(&res, &instructions);

    Ok(())
}
