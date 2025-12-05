type Parsed = Vec<char>;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    Ok(
        input_raw.lines()
                 .map(|line| line.chars().collect())
                 .collect()
    )
}

pub fn part_one(input: &[Parsed]) -> Result<u32, String> {
    let mut out = 0;
    for line in input {
        let mut first = line[0].to_digit(10).unwrap();
        let mut first_index = 0;
        for i in 1..line.len()-1 {
            let n = line[i].to_digit(10).unwrap();
            if n > first {
                first = n;
                first_index = i;
            }
        }

        let mut second = line[first_index+1].to_digit(10).unwrap();
        for i in first_index+2..line.len() {
            let n = line[i].to_digit(10).unwrap();
            if n > second {
                second = n;
            }
        }

        out += first * 10 + second
    }
    Ok(out)
}

pub fn part_two(input: &[Parsed]) -> Result<u64, String> {
    let mut out = 0;
    for line in input {
        let mut num = 0;
        let mut last_index = 1;
        for i in (0..12).rev() {
            num *= 10;
            let mut largest_index = if i == 11 { 0 } else { last_index + 1 };
            let mut largest = line[largest_index].to_digit(10).unwrap();
            for i in largest_index+1..line.len()-i {
                let x = line[i].to_digit(10).unwrap();
                if x > largest {
                    largest = x;
                    largest_index = i;
                }
            }
            last_index = largest_index;
            num += largest as u64;
        }
        out += num;
    }
    Ok(out)
}
