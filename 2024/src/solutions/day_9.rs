type Parsed = Option<usize>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut out = vec![];
    for (i, c) in input_raw.trim().chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 1 {
            out.extend((0..n).map(|_| None));
        }
        else {
            out.extend((0..n).map(|_| Some(i / 2)));
        }
    }
    Ok(out)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut input = input.to_vec();

    while input.last().is_none() {
        input.pop();
    }

    let mut total = 0;
    let mut i = 0;
    while i < input.len() {
        if let Some(x) = input[i] {
            total += i * x;
            i += 1;
            continue;
        }

        input[i] = input.pop().unwrap();
        while input.last().unwrap().is_none() {
            input.pop();
        }
    }

    Ok(total)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    let mut grouped: Vec<(Option<usize>,usize)> = vec![];
    let mut input_iter = input.iter();

    while input_iter.len() > 0 {
        let value = *input_iter.next().unwrap();
        let count = 1 + input_iter.clone().take_while(|v| **v == value).count();
        if count > 1 {
            input_iter.nth(count-2); // -2 since next consumed one already and 0 index
        }
        grouped.push((value, count));
    }

    let mut index = grouped.len() - 1;
    while index > 0 {
        let (id, len) = grouped[index];
        
        if id.is_some() {
            let mut dest = None;
            for (j, &(y, l)) in grouped[0..index].iter().enumerate() {
                if y.is_some() || l < len {
                    continue;
                }

                dest = Some(j);
                break;
            }

            if let Some(j) = dest {
                if grouped[j].1 == len {
                    grouped[index].0 = None;
                    grouped[j].0 = id;
                }
                else { // need to split the new segment
                    grouped[index].0 = None;
                    grouped[j].0 = None;
                    grouped[j].1 -= len;
                    grouped.insert(j, (id, len));
                    index += 1; // avoid decrementing index
                }
            }
        }

        index -= 1;
    }


    Ok(
        grouped.iter().fold((0,0), |(p,t), &(id,l)| {
            (
                p + l,
                if let Some(x) = id {
                    t + x * (p..p+l).sum::<usize>()
                } else {
                    t
                }
            )
        }).1
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_9_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(1928));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_9_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(2858));
    }
}

