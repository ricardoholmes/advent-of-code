type Parsed = (u64, u64);

pub fn parse(input_raw: &str) -> Result<Vec<Parsed>, String> {
    let mut parsed = vec![];
    for range in input_raw.trim().split(',') {
        let mut split = range.split('-');
        parsed.push((
            split.next().unwrap().parse().unwrap(), 
            split.next().unwrap().parse().unwrap()
        ));
    }

    Ok(parsed)
}

pub fn part_one(input: &[Parsed]) -> Result<u64, String> {
    let mut sum = 0;
    for &(start, end) in input {
        for i in start..=end {
            let s = i.to_string();
            if s[..s.len()/2] == s[s.len()/2..] {
                sum += i;
            }
        }
    }
    Ok(sum)
}

pub fn part_two(input: &[Parsed]) -> Result<u64, String> {
    let mut sum = 0;
    for &(start, end) in input {
        for i in start..=end {
            let s = i.to_string();
            for factor in 2..=s.len() {
                if s.len() % factor != 0 {
                    continue;
                }
                let mut found = true;
                let len = s.len() / factor;
                for j in 1..factor {
                    if s[..len] != s[len*j..len*(j+1)] {
                        found = false;
                        break;
                    }
                }
                if found {
                    sum += i;
                    break;
                }
            }
        }
    }
    Ok(sum)
}
