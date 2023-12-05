use crate::safe_unpack;

pub fn run(input_raw: &str) -> Result<(), String> {
    let solutions = parse(input_raw)?;

    let answer_part_one = part_one(&solutions)?;
    println!("Part one: {}", answer_part_one);

    let answer_part_two = part_two(&solutions)?;
    println!("Part two: {}", answer_part_two);

    Ok(())
}

fn parse(input_raw: &str) -> Result<(Vec<u32>, Vec<Vec<[u32; 3]>>), String> {
    let mut input = input_raw.lines();

    let seeds: Vec<u32> = input
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    input.next();

    let mut i = 0;
    let mut maps: Vec<Vec<[u32; 3]>> = vec![vec![]];
    for line in input {
        if line == "" {
            i += 1;
            maps.push(vec![]);
            continue;
        }

        if line.chars().nth(0).unwrap().is_alphabetic() {
            continue;
        }

        safe_unpack!(line.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()), a, b, c);
        maps[i].push([a, b, c]);
    }

    Ok((seeds, maps))
}

fn part_one(solutions: &(Vec<u32>, Vec<Vec<[u32; 3]>>)) -> Result<u32, String> {
    let (mut seeds, maps) = solutions.clone();

    for map in maps {
        let mut out: Vec<u32> = vec![];
        for seed in &seeds {
            let out_len_start = out.len();
            for mapping in &map {
                safe_unpack!(mapping.iter(), dest_start, source_start, search_range);
                if seed > source_start && seed - source_start < *search_range {
                    out.push(dest_start + (seed - source_start));
                    break;
                }
            }
            if out.len() == out_len_start {
                out.push(*seed);
            }
        }
        if out.len() != seeds.len() {
            let error_message = String::from("Failed to apply map on all seeds");
            return Err(error_message);
        }
        seeds = out;
    }
    Ok(*seeds.iter().min().unwrap())
}

fn part_two(solutions: &(Vec<u32>, Vec<Vec<[u32; 3]>>)) -> Result<u32, String> {
    let (seeds, maps) = solutions.clone();

    // each tuple represents (start_of_range, end_of_range)
    let mut seed_ranges: Vec<(u32, u32)> = seeds
        .chunks(2)
        .map(|range| (range[0], range[0] + range[1] - 1))
        .collect();

    for map in maps {
        let mut out: Vec<(u32, u32)> = vec![];
        for seed_range in seed_ranges {
            let mut remaining_ranges: Vec<(u32, u32)> = vec![seed_range];
            for mapping in &map {
                safe_unpack!(mapping.iter(), dest_start, source_start, search_range);
                let (dest_start, source_start, search_range) = (*dest_start, *source_start, *search_range);
                let source_end = source_start + search_range - 1;

                for (range_start, range_end) in remaining_ranges.clone() {
                    if source_start <= range_start && source_end >= range_end {
                        let out_start = range_start - source_start + dest_start;
                        let out_end = range_end - source_start + dest_start;

                        remaining_ranges.pop();
                        out.push((out_start, out_end));
                    }
                    else if source_start > range_start && source_end < range_end {
                        out.push((dest_start, dest_start + search_range - 1));

                        remaining_ranges.pop();
                        remaining_ranges.push((range_start, source_start - 1));
                        remaining_ranges.push((source_end + 1, range_end));
                    }
                    else if (source_start..=source_end).contains(&range_start) { // source_end < range_end
                        let out_start = range_start - source_start + dest_start;
                        let out_end = source_end - source_start + dest_start;
                        out.push((out_start, out_end));

                        remaining_ranges.pop();
                        remaining_ranges.push((source_end + 1, range_end));
                    }
                    else if (source_start..=source_end).contains(&range_end) { // source_start > range_start
                        let out_start = dest_start;
                        let out_end = range_end - source_start + dest_start;
                        out.push((out_start, out_end));

                        remaining_ranges.pop();
                        remaining_ranges.push((range_start, source_start - 1));
                    } 
                }
            }
            for range in remaining_ranges {
                out.push(range);
            }
        }
        seed_ranges = out;
    }
    Ok(seed_ranges.iter().map(|range| range.0).min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_5_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_one(&parsed);
        assert_eq!(result, Ok(35));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_5_1.txt");

        let parsed = parse(&example).unwrap();
        let result = part_two(&parsed);
        assert_eq!(result, Ok(46));
    }
}
