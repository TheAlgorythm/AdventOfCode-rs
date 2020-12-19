use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

type InternalNum = i64;

trait Eval: std::fmt::Debug {
    fn calculate_latin_order(&self) -> InternalNum;
    fn calculate_reversed_order(&self) -> InternalNum;
}

#[derive(Debug)]
struct Num {
    internal: InternalNum,
}

impl Eval for Num {
    fn calculate_latin_order(&self) -> InternalNum {
        self.internal
    }

    fn calculate_reversed_order(&self) -> InternalNum {
        self.internal
    }
}

impl FromStr for Num {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Num {
            internal: s.trim().parse()?,
        })
    }
}

#[derive(Debug)]
enum Operation {
    NoOp,
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "" => Ok(Operation::NoOp),
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => panic!("Unknown operation!"),
        }
    }
}

#[derive(Debug)]
struct Expression {
    sub_evaluations: Vec<(Operation, Box<dyn Eval>)>,
}

impl Eval for Expression {
    fn calculate_latin_order(&self) -> InternalNum {
        let (_no_op, first_eval) = &self.sub_evaluations[0];
        if self.sub_evaluations.len() == 1 {
            return (*first_eval).calculate_latin_order();
        }
        self.sub_evaluations.iter().skip(1).fold(
            (*first_eval).calculate_latin_order(),
            |acc, (operation, sub_evaluation)| match operation {
                Operation::Add => acc + (*sub_evaluation).calculate_latin_order(),
                Operation::Multiply => acc * (*sub_evaluation).calculate_latin_order(),
                Operation::NoOp => panic!("Invalid NoOp!"),
            },
        )
    }

    fn calculate_reversed_order(&self) -> InternalNum {
        let mut product = 1;
        let mut current_index = 0;
        loop {
            match self.sub_evaluations.get(current_index) {
                Some((_sign, sub_evaluation)) => {
                    let mut additive_part_value = sub_evaluation.calculate_reversed_order();
                    current_index += 1;
                    loop {
                        match self.sub_evaluations.get(current_index) {
                            Some((Operation::Add, summand)) => {
                                additive_part_value += summand.calculate_reversed_order();
                                current_index += 1;
                            }
                            _ => break,
                        }
                    }
                    product *= additive_part_value;
                }
                None => break,
            }
        }

        product
    }
}

fn take_inner<'a>(elements: &'a mut VecDeque<&str>) -> VecDeque<&'a str> {
    let mut bracket_stack = 1_u32;
    let mut inner = VecDeque::new();
    loop {
        let element = elements
            .pop_front()
            .expect("Expression ended before matching closing bracket found!");
        match element.chars().nth(0).expect("Empty element!") {
            '(' => {
                inner.push_back(element);
                bracket_stack += 1;
            }
            ')' => {
                bracket_stack -= 1;
                if bracket_stack == 0 {
                    break;
                } else {
                    inner.push_back(element);
                }
            }
            _ => inner.push_back(element),
        }
    }
    inner
}

impl From<VecDeque<&str>> for Expression {
    fn from(mut elements: VecDeque<&str>) -> Self {
        let mut sub_evaluations: Vec<(Operation, Box<dyn Eval>)> = Vec::new();

        loop {
            let element = match elements.pop_front() {
                Some(element) => element,
                None => break,
            };
            match element.chars().nth(0).expect("Empty element!") {
                '0'..='9' => sub_evaluations.push((
                    Operation::NoOp,
                    Box::new(element.parse::<Num>().expect("Couldn't parse number!")),
                )),
                '(' => {
                    sub_evaluations.push((
                        Operation::NoOp,
                        Box::new(Expression::from(take_inner(&mut elements))),
                    ));
                }
                '+' | '*' => {
                    let sign = element;
                    let element = elements.pop_front().expect("No expression after sign!");
                    sub_evaluations.push((
                        sign.parse().expect("Couldn't parse sign!"),
                        match element.chars().nth(0).expect("Empty element!") {
                            '0'..='9' => {
                                Box::new(element.parse::<Num>().expect("Couldn't parse number!"))
                            }
                            '(' => Box::new(Expression::from(take_inner(&mut elements))),
                            character => panic!("Unidentified character '{}'!", character),
                        },
                    ))
                }
                character => panic!("Unidentified character '{}'!", character),
            }
        }

        Expression { sub_evaluations }
    }
}

fn parse_expressions(input: &str) -> Vec<Expression> {
    input
        .replace("(", "( ")
        .replace(")", " )")
        .lines()
        .map(|line| Expression::from(line.split(' ').collect::<VecDeque<&str>>()))
        .collect()
}

fn solve_part_one(expressions: &Vec<Expression>) {
    let sum_of_evals: InternalNum = expressions
        .iter()
        .map(|expression| expression.calculate_latin_order())
        .sum();
    println!(
        "The sum of the resulting values is {} calculated in latin order.",
        sum_of_evals
    );
}

fn solve_part_two(expressions: &Vec<Expression>) {
    let sum_of_evals: InternalNum = expressions
        .iter()
        .map(|expression| expression.calculate_reversed_order())
        .sum();
    println!(
        "The sum of the resulting values is {} calculated in reversed order.",
        sum_of_evals
    );
}

fn main() {
    let input = include_str!("18_data.txt");

    let expressions = parse_expressions(&input);

    solve_part_one(&expressions);
    solve_part_two(&expressions);
}
