use crate::safe_unpack;

type Parsed = String;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let line = input_raw
        .lines()
        .next()
        .unwrap();

    let steps = line
        .split(',')
        .map(|step| step.to_string())
        .collect();

    Ok(steps)
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut total = 0;
    for step in input {
        total += get_hash(step);
    }
    Ok(total)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![];
    for _ in 0..256 {
        boxes.push(vec![]);
    }

    for step in input {
        if step.ends_with('-') {
            let label = &step[..step.len()-1].to_string();
            let hash = get_hash(label);

            if let Some(index) = boxes[hash].iter().position(|(lens, _)| lens == label) {
                boxes[hash].remove(index);
            }
        }
        else {
            safe_unpack!(step.split('='), label, focal_length);
            let label = label.to_string();
            let focal_length = focal_length.parse().unwrap();
            let hash = get_hash(&label);

            if let Some(index) = boxes[hash].iter().position(|(lens, _)| *lens == label) {
                boxes[hash][index].1 = focal_length;
            }
            else {
                boxes[hash].push((label.to_string(), focal_length));
            }
        }
    }

    let mut total = 0;
    for (box_index, lens_box) in boxes.iter().enumerate() {
        for (lens_index, &(_, focal_length)) in lens_box.iter().enumerate() {
            total += (box_index + 1) * (lens_index + 1) * focal_length;
        }
    }

    Ok(total)
}

fn get_hash(step: &String) -> usize {
    let mut hash = 0;
    for c in step.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_15_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(1320));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_15_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(145));
    }
}
