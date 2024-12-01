use anyhow::Error;
use std::collections::HashMap;
use utils;

fn main() {
    let lines = utils::read_lines("./input/day1/input.txt").expect("Failed to read lines");
    let lines = utils::split_lines_ws(&lines).expect("Failed to split lines");

    println!("part 1: {}", part1(&lines).expect("part 1 failed"));
    println!("part 2: {}", part2(&lines).expect("part 2 failed"));
}

fn part1(lines: &Vec<(&str, &str)>) -> Result<i64, Error> {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for (a, b) in lines {
        left.push(a.parse::<i64>()?);
        right.push(b.parse::<i64>()?);
    }

    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + (l - r).abs()))
}

fn part2(lines: &Vec<(&str, &str)>) -> Result<i64, anyhow::Error> {
    let mut left: Vec<i64> = Vec::new();
    let mut hash: HashMap<i64, i64> = HashMap::new();

    for (a, b) in lines {
        left.push(a.parse::<i64>()?);
        *hash.entry(b.parse::<i64>()?).or_insert(0) += 1;
    }

    Ok(left
        .iter()
        .fold(0, |acc, l| acc + (l * hash.get(&l).unwrap_or(&0))))
}
