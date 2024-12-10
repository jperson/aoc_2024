use anyhow::Error;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../input/day9/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Vec<i64> {
    let input: Vec<i64> = src
        .chars()
        .filter(|p| p.is_ascii_digit())
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut output: Vec<i64> = Vec::new();
    let mut id = 0;

    let mut input = &input[..];
    while let [b, f, rest @ ..] = &input[..] {
        if *b > 0 {
            let block = (0..*b).into_iter().map(|_| id);
            output.extend(block); //&block.join("");
        }

        if *f > 0 {
            let free = (0..*f).into_iter().map(|_| -1);
            output.extend(free);
        }

        input = &rest[..];
        id += 1;
    }

    if let [b] = &input[..] {
        if *b > 0 {
            let block = (0..*b).into_iter().map(|_| id);
            output.extend(block);
        }
    }
    output
}

fn part1(src: &str) -> Result<i64, Error> {
    let mut blocks = parse_input(src);

    let mut start = 0;
    let mut end = blocks.len() - 1;

    while start < end {
        while blocks[start] != -1 {
            start += 1;
        }

        while blocks[end] == -1 {
            end -= 1;
        }
        blocks[start] = blocks[end];
        blocks[end] = -1;

        start += 1;
        end -= 1;
    }

    let checksum = blocks
        .iter()
        .filter(|x| **x >= 0)
        .enumerate()
        .fold(0, |acc, (i, b)| acc + (b * i as i64));
    Ok(checksum)
}

fn part2(src: &str) -> Result<i64, Error> {
    let blocks = parse_input(src);
    let mut groups: VecDeque<(usize, i64, bool)> = blocks
        .into_iter()
        .dedup_with_count()
        .map(|(a, b)| (a, b, false))
        .collect();

    let mut end = groups.len() - 1;

    'outer: while (end as isize) >= 0 {
        while (end as isize) >= 0 && groups[end].1 == -1 {
            end -= 1;
        }
        let b = groups[end];

        for start in 0..end {
            if groups[start].1 == -1 && groups[start].0 >= b.0 {
                let m = groups[start];

                groups[start] = (b.0, b.1, true);
                groups[end] = (b.0, -1, true);

                if m.0 > b.0 {
                    groups.insert(1 + start, (m.0 - b.0, -1, true));
                }
                continue 'outer;
            }
        }
        end -= 1;
    }

    let checksum = groups
        .iter()
        .map(|(r, v, _)| itertools::repeat_n(v, *r).collect_vec())
        .flatten()
        .map(|n| if *n < 0 { 0 } else { *n })
        .enumerate()
        .fold(0, |acc, (i, b)| acc + (b * i as i64));

    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "2333133121414131402";

        assert_eq!(1928, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "2333133121414131402";

        assert_eq!(2858, part2(src).unwrap());
    }
}
