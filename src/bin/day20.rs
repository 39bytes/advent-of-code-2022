use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    let input: Vec<i64> = read_to_string("inputs/day20.txt")
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    println!("Solution to part 1: {}", part1(&input));
    println!("Solution to part 2: {}", part2(&input));
}

fn part1(nums: &Vec<i64>) -> i64 {
    let mut nums: VecDeque<(usize, &i64)> = nums.iter().enumerate().collect();
    mix(&mut nums);
    get_grove_coord_sum(&nums)
}

fn part2(nums: &Vec<i64>) -> i64 {
    const DECRYPTION_KEY: i64 = 811589153;
    let decrypted: Vec<i64> = nums.iter().map(|x| x * DECRYPTION_KEY).collect();
    let mut nums: VecDeque<(usize, &i64)> = decrypted.iter().enumerate().collect();
    for _ in 0..10 {
        mix(&mut nums);
    }
    get_grove_coord_sum(&nums)
}

fn mix(nums: &mut VecDeque<(usize, &i64)>) {
    for i in 0..nums.len() {
        let index = find_index(i, &nums).expect("Index was not in the list");
        move_num(index, nums);
    }
}

fn move_num(index: usize, nums: &mut VecDeque<(usize, &i64)>) {
    let elem = nums.remove(index).unwrap();
    let length = nums.len();
    let x = index as i64 + elem.1;
    let move_to = if x == 0 {
        length
    } else {
        x.rem_euclid(length as i64) as usize
    };
    nums.insert(move_to, elem);
}

fn find_index<'a>(index: usize, nums: &VecDeque<(usize, &i64)>) -> Option<usize> {
    for (i, (j, _)) in nums.iter().enumerate() {
        if *j == index {
            return Some(i);
        }
    }
    None
}

fn get_grove_coord_sum(nums: &VecDeque<(usize, &i64)>) -> i64 {
    let zero_pos = nums.iter().position(|x| *x.1 == 0).unwrap();
    let first = nums[(zero_pos + 1000) % nums.len()].1;
    let second = nums[(zero_pos + 2000) % nums.len()].1;
    let third = nums[(zero_pos + 3000) % nums.len()].1;

    first + second + third
}
