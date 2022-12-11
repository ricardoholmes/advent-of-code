#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    true_throw: usize,
    false_throw: usize,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    operand: Option<u64>,
}

impl Operation {
    fn calculate(&self, old: u64) -> u64 {
        let operand = match self.operand {
            Some(operand) => operand,
            None => old
        };

        match self.operator {
            Operator::Add => old + operand,
            Operator::Multiply => old * operand,
        }
    }
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_11.txt");

    let input: Vec<Monkey> = input_str
        .split("\n\n")
        .map(|lines| {
            let line: Vec<&str> = lines
                .lines()
                .collect();

            let items: Vec<u64> = line[1]
                .chars()
                .skip_while(|i| !i.is_ascii_digit())
                .collect::<String>()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect();

            let operation_vec: Vec<&str> = line[2]
                .split(' ')
                .rev()
                .take(2)
                .collect();

            let operation: Operation = Operation {
                operator: match operation_vec[1] {
                    "+" => Operator::Add,
                    "*" => Operator::Multiply,
                    _ => panic!("Invalid operator '{}'", operation_vec[1]),
                },
                operand: if let Ok(x) = operation_vec[0].parse::<u64>() { Some(x) } else { None },
            };

            let divisible_by: u64 = line[3]
                .split_ascii_whitespace()
                .last().unwrap()
                .parse().unwrap();

            let true_throw: usize = line[4]
                .split_ascii_whitespace()
                .last().unwrap()
                .parse().unwrap();

            let false_throw: usize = line[5]
                .split_ascii_whitespace()
                .last().unwrap()
                .parse().unwrap();

            Monkey {
                items,
                operation,
                divisible_by,
                true_throw,
                false_throw,
            }
        })
        .collect();

    part_one(&input);
    part_two(&input)
}

fn part_one(input: &Vec<Monkey>) {
    let mut monkeys: Vec<Monkey> = input.clone();
    let mut inspections: Vec<u64> = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i].items.len() as u64;

            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items[0];
                let new_worry_level = monkeys[i].operation.calculate(item) / 3;
                monkeys[i].items.remove(0);

                let throw_index: usize;
                if new_worry_level % monkeys[i].divisible_by == 0 {
                    throw_index = monkeys[i].true_throw;
                }
                else {
                    throw_index = monkeys[i].false_throw;
                }
                monkeys[throw_index].items.push(new_worry_level);
            }
        }
    }

    inspections.sort_unstable();
    inspections.reverse();

    println!("Part one: {}", inspections[0] * inspections[1]);
}

fn part_two(input: &Vec<Monkey>) {
    let mut monkeys: Vec<Monkey> = input.clone();
    let mut inspections: Vec<u64> = vec![0; monkeys.len()];

    let mut safe_mod: u64 = 1;
    for monkey in &monkeys {
        safe_mod = lcm(safe_mod, monkey.divisible_by);
    }

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i].items.len() as u64;

            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items[0];
                let new_worry_level = monkeys[i].operation.calculate(item) % safe_mod;
                monkeys[i].items.remove(0);

                let throw_index: usize;
                if new_worry_level % monkeys[i].divisible_by == 0 {
                    throw_index = monkeys[i].true_throw;
                }
                else {
                    throw_index = monkeys[i].false_throw;
                }
                monkeys[throw_index].items.push(new_worry_level);
            }
        }
    }

    inspections.sort_unstable();
    inspections.reverse();

    println!("Part two: {}", inspections[0] * inspections[1]);
}

fn lcm(a: u64, b: u64) -> u64 {
    // get gcd
    let mut gcd: u64 = 1;
    for i in (1..=a).rev() {
        if a % i == 0 && b % i == 0 {
            gcd = i;
            break;
        }
    }

    a * b / gcd
}
