pub fn run() {
    let input_str = include_str!("../../inputs/input_25.txt");

    let mut sum: i64 = input_str
        .lines()
        .map(|line| {
            let mut num = 0;
            let mut power_of_five = 1;
            for c in line.chars().rev() {
                num += power_of_five * match c {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => panic!()
                };
                power_of_five *= 5;
            }
            num
        }).sum();

    let mut base_five: Vec<i64> = vec![];
    while sum > 0 {
        base_five.insert(0, sum % 5);
        sum /= 5;
    }

    for i in (0..base_five.len()).rev() {
        if base_five[i] >= 3 {
            if i == 0 {
                base_five.insert(0, 1);
            }
            base_five[i-1] += 1;
            base_five[i] = base_five[i] - 5;
        }
    }

    let snafu: String = base_five
        .iter()
        .map(|&i| match i {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        })
        .collect();

    println!("Answer: {snafu}");
}