use anyhow::Error;
use rustc_hash::{FxHashMap, FxHashSet};
use utils::grid::Grid;

fn main() {
    let input = include_str!("../../input/day25/input.txt");

    println!("part 1: {}", part1(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Vec<Grid<char>> {
    let kls = src.split("\n\n").collect::<Vec<_>>();

    let mut vs: Vec<Grid<char>> = Vec::new();
    for kl in kls {
        let g: Vec<Vec<char>> = kl
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        vs.push(Grid::new(g));
    }
    vs
}

fn part1(src: &str) -> Result<i64, Error> {
    let kls = parse_input(src);
    let mut total: i64 = 0;

    let keys: Vec<&Grid<char>> = kls.iter().filter(|k| k.row(0).all(|p| *p == '.')).collect();
    let locks: Vec<&Grid<char>> = kls.iter().filter(|k| k.row(0).all(|p| *p == '#')).collect();

    let mut kvs: Vec<[i64; 5]> = vec![];
    for k in keys.iter() {
        let mut ps: [i64; 5] = [0; 5];
        for c in 0..keys[0].width {
            ps[c as usize] = k.col(c).filter(|p| **p == '#').count() as i64;
        }
        kvs.push(ps);
    }

    let mut lvs: Vec<[i64; 5]> = vec![];
    for l in locks.iter() {
        let mut ps: [i64; 5] = [0; 5];
        for c in 0..locks[0].width {
            ps[c as usize] = l.col(c).filter(|p| **p == '#').count() as i64;
        }
        lvs.push(ps);
    }

    for l in &lvs {
        for k in &kvs {
            if l.iter().zip(k.iter()).map(|(a, b)| a + b).all(|p| p <= 7) {
                total += 1;
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!(3, part1(src).unwrap());
    }
}
