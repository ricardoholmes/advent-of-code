pub fn run() {
    let input_str = include_str!("../../inputs/input_20.txt");

    let input: Vec<i32> = input_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    part_one(&input);
}

fn part_one(input: &[i32]) {
    let input = input.to_vec();
    let mut indexes: Vec<usize> = (0..input.len()).collect();

    for (i, &value) in input.iter().enumerate() {
        let index = indexes.iter().position(|&index| index == i).unwrap();
        indexes.remove(index);
        let new_index = (index as i32 + value).rem_euclid(indexes.len() as i32) as usize;
        indexes.insert(new_index, i);
    }

    let zero_index_input = input.iter().position(|&num| num == 0).unwrap();
    let zero_pos = indexes.iter().position(|&num| num == zero_index_input).unwrap();
    println!("Part one: {}", [1000, 2000, 3000].iter().map(|&x| input[indexes[(zero_pos + x) % input.len()]]).sum::<i32>());
}
