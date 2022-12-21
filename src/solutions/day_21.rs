use std::{collections::HashMap, panic};

#[derive(Clone, Debug)]
enum Calculation {
    Value(i64),
    Op(Operation)
}

impl Calculation {
    fn calculate(&self, monkeys: &HashMap<String, Calculation>) -> i64 {
        match self {
            Self::Value(x) => *x,
            Self::Op(operation) => {
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

    fn get_lhs(&self, monkeys: &HashMap<String, Calculation>) -> Option<Calculation> {
        match self {
            Self::Value(_) => None,
            Self::Op(x) => Some(monkeys.get(&x.lhs).unwrap().clone())
        }
    }

    fn get_rhs(&self, monkeys: &HashMap<String, Calculation>) -> Option<Calculation> {
        match self {
            Self::Value(_) => None,
            Self::Op(x) => Some(monkeys.get(&x.rhs).unwrap().clone())
        }
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Clone, Debug)]
struct Operation {
    lhs: String,
    rhs: String,
    op: Operator
}

impl Operation {
    fn find_humn(&self, path: &Vec<u8>, monkeys: &HashMap<String, Calculation>) -> Option<Vec<u8>> {
        if self.lhs == "humn".to_string() {
            let mut path = path.clone();
            path.push(0);
            return Some(path);
        }
        else if self.rhs == "humn".to_string() {
            let mut path = path.clone();
            path.push(1);
            return Some(path);
        }

        let lhs_monkey = monkeys.get(&self.lhs).unwrap();
        let rhs_monkey = monkeys.get(&self.rhs).unwrap();
        for (direction, monkey) in [lhs_monkey, rhs_monkey].iter().enumerate() {
            let val = match monkey {
                Calculation::Value(_) => None,
                Calculation::Op(operation) => {
                    let mut path = path.clone();
                    path.push(direction as u8);
                    operation.find_humn(&path, monkeys)
                }
            };

            if let Some(_) = val {
                return val;
            }
        }

        None
    }
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
    part_two(&monkeys);
}

fn part_one(monkeys: &HashMap<String, Calculation>) {
    let ans = monkeys.get(&"root".to_string()).unwrap().calculate(monkeys);
    println!("Part one: {ans}");
}

fn part_two(monkeys: &HashMap<String, Calculation>) {
    let monkey = monkeys.get(&"root".to_string()).unwrap();
    let path = match monkey {
        Calculation::Value(_) => panic!(),
        Calculation::Op(x) => x.find_humn(&vec![], monkeys).unwrap()
    };

    let (mut monkey, mut ans) = if path[0] == 0 {
            (monkey.get_lhs(monkeys).unwrap(), monkey.get_rhs(monkeys).unwrap().calculate(monkeys))
        } else {
            (monkey.get_rhs(monkeys).unwrap(), monkey.get_lhs(monkeys).unwrap().calculate(monkeys))
        };

    for &direction in &path[1..] {
        let operation = match &monkey {
            Calculation::Value(i) => { println!("{i}"); panic!() },
            Calculation::Op(x) => x.clone()
        };
        
        let lhs = monkey.get_lhs(monkeys).unwrap();
        let rhs = monkey.get_rhs(monkeys).unwrap();
        let val = if direction == 0 {
            rhs.calculate(monkeys)
        } else {
            lhs.calculate(monkeys)
        };
        
        ans = match operation.op {
            Operator::Add => ans - val,
            Operator::Subtract => if direction == 0 { ans + val } else { val - ans },
            Operator::Multiply => ans / val,
            Operator::Divide => if direction == 0 { ans * val } else { val / ans }
        };
        
        monkey = if direction == 0 { monkey.get_lhs(monkeys).unwrap() } else { rhs }
    }

    println!("Part two: {ans}");
}
