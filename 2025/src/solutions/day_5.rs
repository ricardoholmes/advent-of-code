type Parsed = (Vec<(u64, u64)>, Vec<u64>);

pub fn parse(input_raw: &str) -> Result<Parsed, String> {
    let mut halves = input_raw.split("\n\n");

    let mut ranges = vec![];
    for range in halves.next().unwrap().trim().lines() {
        let mut split = range.split('-');
        ranges.push((
            split.next().unwrap().parse().unwrap(), 
            split.next().unwrap().parse().unwrap()
        ));
    }

    let mut values = vec![];
    for value in halves.next().unwrap().trim().lines() {
        values.push(value.parse().unwrap())
    }

    Ok((ranges, values))
}

pub fn part_one(input: &Parsed) -> Result<usize, String> {
    let (ranges, values) = input;

    let mut count = 0;
    for &val in values {
        let mut fresh = false;
        for &(start, end) in ranges {
            if val >= start && val <= end {
                fresh = true;
                break;
            }
        }
        if fresh {
            count += 1;
        }
    }
    Ok(count)
}

pub fn part_two(input: &Parsed) -> Result<u64, String> {
    let (input_ranges, _) = input;

    let mut ranges: Vec<(u64, u64)> = vec![];
    for &(start, end) in input_ranges {
        let mut new_start = start;
        let mut new_end = end;
        for i in (0..ranges.len()).rev() {
            let r0 = ranges[i].0;
            let r1 = ranges[i].1;
            if new_start <= r1 && new_end >= r0 {
                new_start = new_start.min(r0);
                new_end = new_end.max(r1);
                ranges.swap_remove(i);
            }
        }
        ranges.push((new_start, new_end));
    }

    let mut out = 0;
    for (r0, r1) in ranges {
        out += r1 - r0 + 1;
    }
    Ok(out)
}
