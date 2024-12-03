use anyhow::Error;
use itertools::Itertools;

use utils;

fn main() {
    let lines = utils::read_lines("./input/day2/input.txt").expect("Failed to read lines");
    let lines = utils::split_lines_vec(&lines);

    let mut vs: Vec<Vec<i32>> = Vec::new();
    for l in lines {
        vs.push(
            l.into_iter()
                .map(|n| n.parse::<i32>().expect("parse i32"))
                .collect(),
        );
    }

    println!("part 1: {}", part1(&vs).expect("part 1 failed"));
    println!("part 2: {}", part2(vs).expect("part 2 failed"));
}

fn part1(lines: &Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut count = 0;

    for l in lines {
        let increasing = l[0] < l[1];
        let mut safe = true;

        for (a, b) in l.iter().tuple_windows() {
            safe &= increasing == (a < b) && check_pair(*a, *b);
        }

        if safe {
            count += 1;
        }
    }
    Ok(count)
}

fn part2(lines: Vec<Vec<i32>>) -> Result<i32, Error> {
    let mut vs: Vec<Vec<i32>> = Vec::new();

    for l in lines {
        if check_pairs(&l) && is_monotonic(&l) {
            vs.push(l);
        } else {
            for v in utils::remove_ith(&l) {
                if check_pairs(&v) && is_monotonic(&v) {
                    vs.push(v);
                    break;
                }
            }
        }
    }

    Ok(vs.len() as i32)
}

fn check_pair(a: i32, b: i32) -> bool {
    1 <= (a - b).abs() && (a - b).abs() <= 3
}

fn check_pairs(l: &Vec<i32>) -> bool {
    l.iter()
        .tuple_windows()
        .map(|(a, b)| check_pair(*a, *b))
        .all(|p| p)
}

fn is_monotonic(l: &Vec<i32>) -> bool {
    l.iter().tuple_windows().all(|(a, b)| a < b) || l.iter().tuple_windows().all(|(a, b)| a > b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(2, part1(&lines).unwrap());
    }

    #[test]
    fn test_part_2() {
        let lines: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(4, part2(lines).unwrap());
    }
}
