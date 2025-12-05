type Parsed = i32;

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut output = vec![];
    for line in input_raw.lines() {
        let n = line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') {
            output.push(-n);
        }
        else {
            output.push(n);
        }
    }

    Ok(output)
}

pub fn part_one(input: &[Parsed]) -> Result<i32, String> {
    let mut val = 50;
    let mut count = 0;
    for i in input {
        val = (val + i + 100) % 100;
        if val == 0 {
            count += 1;
        }
    }

    Ok(count)
}

pub fn part_two(input: &[Parsed]) -> Result<usize, String> {
    let mut val = 50;
    let mut count = 0;
    for &i in input {
        let old_val = val;
        val += i;
        if val <= 0 {
            if old_val == 0 && i != 0 {
                val += 100
            }
            while val < 0 {
                val += 100;
                count += 1;
            }
            if val == 0 {
                count += 1;
            }
        }
        else if val >= 100 {
            while val >= 100 {
                val -= 100;
                count += 1;
            }
        }
    }

    Ok(count)
}
