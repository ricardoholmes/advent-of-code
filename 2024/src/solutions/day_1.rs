pub fn parse(input_raw: &str) -> Result<(Vec<u32>, Vec<u32>), String> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in input_raw.lines() {
        let line_split = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left.push(line_split[0].parse().unwrap());
        right.push(line_split[1].parse().unwrap());
    }

    Ok((left, right))
}

pub fn part_one(cols: &(Vec<u32>, Vec<u32>)) -> Result<u32, String> {
    let (mut left, mut right) = cols.clone();
    left.sort();
    right.sort();

    let mut out = 0;
    for i in 0..left.len() {
        out += left[i].abs_diff(right[i])
    }

    Ok(out)
}

pub fn part_two(cols: &(Vec<u32>, Vec<u32>)) -> Result<u32, String> {
    let (left, right) = cols.clone();
    let mut total = 0;
    for n in left {
        total += n * right.iter().filter(|&&x| x == n).count() as u32
    }
    Ok(total)
}
