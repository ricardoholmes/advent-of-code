type Parsed = (Vec<String>, Vec<char>);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let lines: Vec<&str> = input_raw.lines().collect();
    let (line_ops, lines_nums) = lines.split_last().unwrap();

    let ops = line_ops.split_ascii_whitespace()
                                 .map(|i| i.chars().next().unwrap())
                                 .collect();

    Ok((lines_nums.iter().map(|x| x.to_string()).collect(), ops))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (lines_nums, ops) = input;

    let mut nums = vec![];
    for line in lines_nums {
        for (i, n) in line.split_ascii_whitespace().enumerate() {
            if nums.len() == i {
                nums.push(vec![]);
            }
            nums[i].push(n.parse().unwrap());
        }
    }

    let mut out = 0;
    for (ns, &op) in nums.iter().zip(ops) {
        out += ns.iter().fold(
            if op == '+' { 0 } else { 1 },
            |a, b| if op == '+' { a + b } else { a * b }
        );
    }

    Ok(out)
}

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    let (lines, ops) = input;

    let line_len = lines[0].len();
    let mut lines_digits: Vec<_> = lines.iter().map(|l| l.chars()).collect();
    let transposed = (0..line_len)
        .map(|_| lines_digits.iter_mut().map(|n| n.next().unwrap()).collect::<String>());
    
    let mut col = 0;
    let mut subtotal = if ops[col] == '+' { 0 } else { 1 };
    let mut total = 0;
    for n_str in transposed {
        if n_str.trim().is_empty() {
            total += subtotal;
            col += 1;
            subtotal = if ops[col] == '+' { 0 } else { 1 };
            continue;
        }

        let n: usize = n_str.trim().parse().unwrap();
        subtotal = if ops[col] == '+' { subtotal + n } else { subtotal * n };
    }
    total += subtotal;

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_6_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(4277556));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_6_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(3263827));
    }
}
