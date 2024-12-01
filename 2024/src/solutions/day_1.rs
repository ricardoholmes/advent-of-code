pub fn parse(input_raw: &str) -> Result<&str, String> {
    Ok(input_raw)
}

pub fn part_one(input_str: &&str) -> Result<u32, String> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in input_str.lines() {
        let line_split = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left.push(line_split[0].parse().unwrap());
        right.push(line_split[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut out = 0;
    for i in 0..left.len() {
        out += left[i].abs_diff(right[i])
    }

    Ok(out)
}

pub fn part_two(input_str: &&str) -> Result<u32, String> {
    Ok(1)
}
