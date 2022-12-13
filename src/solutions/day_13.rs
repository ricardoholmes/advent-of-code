#[derive(Clone, Debug)]
enum Element {
    Value(i32),
    Elements(Vec<Element>),
}

pub fn run() {
    let input_str = include_str!("../../inputs/input_13.txt");

    let input: Vec<Vec<&str>> = input_str
        .split("\n\n")
        .map(|section| section.lines().collect::<Vec<&str>>())
        .collect();

    let mut pairs: Vec<Vec<Element>> = vec!();
    for section in input {
        let mut pair: Vec<Element> = vec!();
        for line in section {
            let mut l = line.chars();
            l.next();
            l.next_back();
            pair.push(parse(l.as_str()).0);
        }
        pairs.push(pair);
    }

    part_one(&pairs);
}

fn part_one(pairs: &Vec<Vec<Element>>) {
    let mut i = 0;
    let count = pairs
        .iter()
        .fold(0, |total, pair| total +
            if compare(pair.get(0).unwrap(), pair.get(1).unwrap()) < 0 {
                i += 1;
                i
            } else {
                i += 1;
                0
            }
        );

    println!("Part one {count}");
}

fn parse(line: &str) -> (Element, usize) {
    let mut packets: Vec<Element> = Vec::new();
    let mut skip = 0;
    let mut number = String::new();

    for (i, c) in line.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
        }
        else if c == '[' {
            let packet;
            (packet, skip) = parse(&line[i+1..]);
            skip += 1;
            packets.push(packet);
        }
        else if c == ']' {
            if !number.is_empty() {
                // println!("{number}");
                packets.push(Element::Value(number.parse().unwrap()));
                number.clear();
            }
            return (Element::Elements(packets), i);
        }
        else if c == ',' {
            if !number.is_empty() {
                packets.push(Element::Value(number.parse().unwrap()));
            }
            number.clear();
        }
        else {
            number.push(c);
        }
    }

    if !number.is_empty() {
        packets.push(Element::Value(number.parse().unwrap()));
        number.clear();
    }

    (Element::Elements(packets), 0)
}

fn compare(left: &Element, right: &Element) -> i32 {
    let left_num = match left {
        Element::Value(x) => *x,
        _ => -1,
    };
    
    let right_num = match right {
        Element::Value(x) => *x,
        _ => -1,
    };

    if left_num != -1 && right_num != -1 {
        return left_num - right_num;
    }

    else if left_num == -1 && right_num != -1 {
        let r: Element = Element::Elements(vec!(right.clone()));
        return compare(left, &r);
    }

    else if left_num != -1 && right_num == -1 {
        let l: Element = Element::Elements(vec!(left.clone()));
        return compare(&l, right);
    }

    else {
        let l = match left {
            Element::Elements(x) => x,
            _ => panic!(),
        };
        
        let r = match right {
            Element::Elements(x) => x,
            _ => panic!(),
        };

        if l.len() == 0 && r.len() == 0 {
            return 0;
        }
        else if l.len() == 0 {
            return -1;
        }
        else if r.len() == 0 {
            return 1;
        }
        else {
            let comp = compare(l.get(0).unwrap(), r.get(0).unwrap());

            if comp != 0 {
                return comp;
            }

            let left = Element::Elements(l[1..].to_owned());
            let right = Element::Elements(r[1..].to_owned());
            return compare(&left, &right);
        }
    }
}
