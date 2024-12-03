pub fn parse(input_raw: &str) -> Result<(Vec<u32>, Vec<u32>), String> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in input_raw.lines() {
        let line_split = line.split_ascii_whitespace().collect::<Vec<&str>>();
        left.push(line_split[0].parse().unwrap());
        right.push(line_split[1].parse().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    Ok((left, right))
}

pub fn part_one(cols: &(Vec<u32>, Vec<u32>)) -> Result<u32, String> {
    let (left, right) = cols;

    Ok(
        left.iter()
            .zip(right)
            .fold(0,
                |acc,(&l,&r)| acc + l.abs_diff(r)
            )
    )
}

pub fn part_two(cols: &(Vec<u32>, Vec<u32>)) -> Result<usize, String> {
    let (left, right) = cols;

    let mut left = left.iter();
    let mut right = right.iter();

    let mut l = left.next();
    let mut r = left.next();
    let mut total = 0;
    while l.is_some() && r.is_some() {
        let lx = *l.unwrap();
        let rx = *r.unwrap();
        match lx.cmp(&rx) {
            std::cmp::Ordering::Less => {
                let rx = r.unwrap();
                let _ = left.by_ref().skip_while(|&x| x < rx);
                l = left.next();
            },
            std::cmp::Ordering::Greater => {
                let lx = l.unwrap();
                let _ = right.by_ref().skip_while(|&x| x < lx);
                r = right.next();
            },
            std::cmp::Ordering::Equal => {
                let n = l.unwrap();

                let l_count = 1 + left.by_ref().take_while(|&x| x == n).count(); // this seems to only ever be 1 but whatever
                let r_count = 1 + right.by_ref().take_while(|&x| x == n).count();
                total += l_count * r_count;

                l = left.next();
                r = right.next();
            },
        }
    }

    Ok(total)
}
