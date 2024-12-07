type Parsed = Vec<char>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(input_raw.lines().map(|s| s.chars().collect()).collect())
}

pub fn part_one(input: &[Parsed]) -> Result<usize, String> {
    let rows = input.len();
    let cols = input[0].len(); // should be constant across all rows

    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if input[i][j] == 'X' { // check for XMAS
                let can_down = rows - i >= 4;

                // vertical down
                if can_down && input[i+1][j] == 'M' && input[i+2][j] == 'A' && input[i+3][j] == 'S' {
                    count += 1;
                }

                let can_right = cols - j >= 4;
                if !can_right {
                    // every check after here looks right
                    continue;
                }

                // horizontal right
                if input[i][j+1] == 'M' && input[i][j+2] == 'A' && input[i][j+3] == 'S' {
                    count += 1;
                }

                // diagonal right down
                if can_down && input[i+1][j+1] == 'M' && input[i+2][j+2] == 'A' && input[i+3][j+3] == 'S' {
                    count += 1;
                }
                
                let can_up = i >= 3;

                // diagonal right up
                if can_up && input[i-1][j+1] == 'M' && input[i-2][j+2] == 'A' && input[i-3][j+3] == 'S' {
                    count += 1;
                }
            }
            else if input[i][j] == 'S' { // check for SAMX
                let can_down = rows - i >= 4;

                // vertical down
                if can_down && input[i+1][j] == 'A' && input[i+2][j] == 'M' && input[i+3][j] == 'X' {
                    count += 1;
                }
                
                let can_right = cols - j >= 4;
                if !can_right {
                    continue;
                }

                // horizontal right
                if input[i][j+1] == 'A' && input[i][j+2] == 'M' && input[i][j+3] == 'X' {
                    count += 1;
                }

                // diagonal right down
                if can_down && input[i+1][j+1] == 'A' && input[i+2][j+2] == 'M' && input[i+3][j+3] == 'X' {
                    count += 1;
                }

                let can_up = i >= 3;

                // diagonal right up
                if can_up && input[i-1][j+1] == 'A' && input[i-2][j+2] == 'M' && input[i-3][j+3] == 'X' {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    let mut count = 0;
    for i in 1..input.len()-1 {
        for j in 1..input[i].len()-1 {
            if input[i][j] != 'A' {
                continue;
            }

            if !((input[i-1][j-1] == 'M' && input[i+1][j+1] == 'S') || (input[i-1][j-1] == 'S' && input[i+1][j+1] == 'M')) {
                continue;
            }

            if !((input[i+1][j-1] == 'M' && input[i-1][j+1] == 'S') || (input[i+1][j-1] == 'S' && input[i-1][j+1] == 'M')) {
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
