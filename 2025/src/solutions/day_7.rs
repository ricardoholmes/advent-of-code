// x start, splitter map
type Parsed = (usize, Vec<Vec<bool>>);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut lines = input_raw.lines();

    let first_line = lines.next().unwrap();
    let start_x = first_line.find('S').unwrap();

    let mut splitters = vec![];
    for line in lines {
        let line_splitters: Vec<bool> = line.chars().map(|c| c == '^').collect();
        if line_splitters.iter().any(|&b| b) {
            splitters.push(line_splitters);
        }
    }

    Ok((start_x, splitters))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (start_x, splitters) = input;
    let width = splitters[0].len();

    let mut out = 0;
    let mut beams = vec![false; width];
    beams[*start_x] = true;
    for line in splitters {
        let mut next_beams = beams.clone();
        for x in (0..width).filter(|&i| beams[i]) {
            if line[x] {
                out += 1;
                next_beams[x] = false;
                next_beams[x-1] = true;
                next_beams[x+1] = true;
            }
        }
        beams = next_beams;
    }

    Ok(out)
}

pub fn part_two(input: &Parsed) -> Result<usize, String> {
    let (start_x, splitters) = input;
    let width = splitters[0].len();

    let mut counts = vec![0; width];
    counts[*start_x] = 1;
    for line in splitters {
        let mut next_counts = counts.clone();
        for x in (0..width).filter(|&i| counts[i] != 0) {
            if line[x] {
                next_counts[x] = 0;
                next_counts[x-1] += counts[x];
                next_counts[x+1] += counts[x];
            }
        }
        counts = next_counts;
    }

    Ok(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_7_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(21));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_7_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(40));
    }
}
