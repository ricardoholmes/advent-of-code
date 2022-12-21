use std::collections::HashMap;

enum Calculation {
    Value(i64),
    Op(Operation)
}

impl Calculation {
    fn calculate(&self, monkeys: &HashMap<String, Calculation>) -> i64 {
        match self {
            Calculation::Value(x) => *x,
            Calculation::Op(operation) => {
                let lhs = monkeys.get(&operation.lhs).unwrap().calculate(monkeys);
                let rhs = monkeys.get(&operation.rhs).unwrap().calculate(monkeys);
                match operation.op {
                    Operator::Add => lhs + rhs,
                    Operator::Subtract => lhs - rhs,
                    Operator::Multiply => lhs * rhs,
                    Operator::Divide => lhs / rhs
                }
            }
        }
    }
}

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

struct Operation {
    lhs: String,
    rhs: String,
    op: Operator
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_21.txt");

    let monkeys: HashMap<String, Calculation> = input_str
        .lines()
        .map(|line| {
            let line_split: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
            let name = line_split.get(0).unwrap();

            let calc = line_split.get(1).unwrap();
            let calculation = if calc.chars().all(|c| c.is_ascii_digit()) {
                Calculation::Value(calc.parse().unwrap())
            } else {
                let mut calc_split = calc.split_whitespace();
                let lhs = calc_split.next().unwrap().to_string();
                let op = match calc_split.next().unwrap() {
                    "+" => Operator::Add,
                    "-" => Operator::Subtract,
                    "*" => Operator::Multiply,
                    "/" => Operator::Divide,
                    _ => panic!()
                };
                let rhs = calc_split.next().unwrap().to_string();
                Calculation::Op(Operation { lhs, rhs, op })
            };

            (name.to_owned(), calculation)
        }).collect();

    part_one(&monkeys);
}

fn part_one(monkeys: &HashMap<String, Calculation>) {
    let ans = monkeys.get(&"root".to_string()).unwrap().calculate(monkeys);
    println!("Part one: {ans}");
}
