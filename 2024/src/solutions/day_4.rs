type Parsed = String;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(input_raw.lines().map(|s| s.to_string()).collect())
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            /*
                XMAS
            */
            if input[i].len() - j >= 4 && &input[i][j..j+4] == "XMAS" {
                count += 1;
            }
            else if input[i].len() - j >= 4 && &input[i][j..j+4] == "SAMX" {
                count += 1;
            }

            /*
                X
                M
                A
                S
            */
            if input.len() - i >= 4 {
                if input[i].chars().nth(j).unwrap() == 'X' && input[i+1].chars().nth(j).unwrap() == 'M' && input[i+2].chars().nth(j).unwrap() == 'A' && input[i+3].chars().nth(j).unwrap() == 'S' {
                    count += 1;
                }
                else if input[i].chars().nth(j).unwrap() == 'S' && input[i+1].chars().nth(j).unwrap() == 'A' && input[i+2].chars().nth(j).unwrap() == 'M' && input[i+3].chars().nth(j).unwrap() == 'X' {
                    count += 1;
                }
            }

            /*
                X...
                .M..
                ..A.
                ...S
            */
            if input.len() - i >= 4 && input[i].len() - j >= 4 {
                if input[i].chars().nth(j).unwrap() == 'X' && input[i+1].chars().nth(j+1).unwrap() == 'M' && input[i+2].chars().nth(j+2).unwrap() == 'A' && input[i+3].chars().nth(j+3).unwrap() == 'S' {
                    count += 1;
                }
                else if input[i].chars().nth(j).unwrap() == 'S' && input[i+1].chars().nth(j+1).unwrap() == 'A' && input[i+2].chars().nth(j+2).unwrap() == 'M' && input[i+3].chars().nth(j+3).unwrap() == 'X' {
                    count += 1;
                }
            }

            /*
                ...S
                ..A.
                .M..
                X...
            */
            if i >= 3 && input[i].len() - j >= 4 {
                if input[i].chars().nth(j).unwrap() == 'X' && input[i-1].chars().nth(j+1).unwrap() == 'M' && input[i-2].chars().nth(j+2).unwrap() == 'A' && input[i-3].chars().nth(j+3).unwrap() == 'S' {
                    count += 1;
                }
                else if input[i].chars().nth(j).unwrap() == 'S' && input[i-1].chars().nth(j+1).unwrap() == 'A' && input[i-2].chars().nth(j+2).unwrap() == 'M' && input[i-3].chars().nth(j+3).unwrap() == 'X' {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    let input: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let mut count = 0;
    for i in 1..input.len()-1 {
        for j in 1..input[i].len()-1 {
            if input[i][j] != 'A' {
                continue;
            }

            if !(input[i-1][j-1] == 'M' && input[i+1][j+1] == 'S') && !(input[i-1][j-1] == 'S' && input[i+1][j+1] == 'M') {
                continue;
            }

            if !(input[i+1][j-1] == 'M' && input[i-1][j+1] == 'S') && !(input[i+1][j-1] == 'S' && input[i-1][j+1] == 'M') {
                continue;
            }

            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = include_str!("../../examples/day_4_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_one(&parsed);
        assert_eq!(solution, Ok(18));
    }

    #[test]
    fn test_part2() {
        let example = include_str!("../../examples/day_4_1.txt");
        let parsed = parse(example).unwrap();
        let solution = part_two(&parsed);
        assert_eq!(solution, Ok(9));
    }
}
