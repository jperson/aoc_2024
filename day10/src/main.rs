use anyhow::Error;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use utils::grid::Grid;

fn main() {
    let input = include_str!("../../input/day10/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Grid<i32> {
    let vs = src
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    Grid::new(vs)
}

fn walk_trail(start: (i32, i32), tm: &Grid<i32>) -> (i32, i32) {
    let dirs = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut visited: FxHashSet<(i32, i32)> = FxHashSet::default();

    q.push_back((start.0, start.1));

    let mut rating = 0;
    let mut score = 0;

    while let Some(pt) = q.pop_front() {
        let cur = *tm.at_unsafe(pt.0, pt.1);

        if cur == 9 {
            if !visited.contains(&(pt.0, pt.1)) {
                score += 1;
                visited.insert((pt.0, pt.1));
            }
            rating += 1;
        } else {
            for d in &dirs {
                if tm.in_bounds(pt.0 + d.0, pt.1 + d.1)
                    && *tm.at_unsafe(pt.0 + d.0, pt.1 + d.1) == cur + 1
                {
                    q.push_back((pt.0 + d.0, pt.1 + d.1));
                }
            }
        }
    }

    (score, rating)
}

fn part1(src: &str) -> Result<i32, Error> {
    let trails = parse_input(src);

    let trailheads = trails
        .iter_points()
        .filter(|(x, y)| *trails.at_unsafe(*x, *y) == 0)
        .collect::<Vec<(i32, i32)>>();

    Ok(trailheads
        .iter()
        .fold(0, |acc, start| acc + walk_trail(*start, &trails).0))
}

fn part2(src: &str) -> Result<i32, Error> {
    let trails = parse_input(src);

    let trailheads = trails
        .iter_points()
        .filter(|(x, y)| *trails.at_unsafe(*x, *y) == 0)
        .collect::<Vec<(i32, i32)>>();

    Ok(trailheads
        .iter()
        .fold(0, |acc, start| acc + walk_trail(*start, &trails).1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        assert_eq!(36, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        assert_eq!(81, part2(src).unwrap());
    }
}
