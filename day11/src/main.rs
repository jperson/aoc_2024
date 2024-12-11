use anyhow::Error;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use utils::grid::Grid;

fn main() {
    let input = include_str!("../../input/day11/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Vec<i64> {
    src.trim()
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn split_num(n: i64) -> (i64, i64) {
    let nds: u32 = ((1 + n.ilog10() as i64) / 2) as u32;
    let base: i64 = 10;
    let nds: i64 = base.pow(nds);
    (n / nds, n % nds)
}

fn blink(n: i64, steps: i64, end: i64, t: &mut FxHashMap<(i64, i64), i64>) -> i64 {
    if steps == end {
        1
    } else {
        if t.contains_key(&(n, steps)) {
            *t.get(&(n, steps)).unwrap()
        } else {
            let x = match n {
                0 => blink(1, steps + 1, end, t),
                _ if (1 + n.ilog10() as i64) % 2 == 0 => {
                    let (a, b) = split_num(n);
                    blink(a, steps + 1, end, t) + blink(b, steps + 1, end, t)
                }
                _ => blink(n * 2024, steps + 1, end, t),
            };
            t.insert((n, steps), x);
            x
        }
    }
}

fn part1(src: &str) -> Result<i64, Error> {
    let stones = parse_input(src);
    let mut table: FxHashMap<(i64, i64), i64> = FxHashMap::default();
    let mut total: i64 = 0;

    for s in stones {
        total += blink(s, 0, 25, &mut table);
    }

    Ok(total)
}

fn part2(src: &str) -> Result<i64, Error> {
    let stones = parse_input(src);
    let mut table: FxHashMap<(i64, i64), i64> = FxHashMap::default();
    let mut total: i64 = 0;

    for s in stones {
        total += blink(s, 0, 75, &mut table)
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "125 17";

        assert_eq!(55312, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "125 17";

        assert_eq!(55312, part2(src).unwrap());
    }
}
