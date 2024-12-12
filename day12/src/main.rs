use anyhow::Error;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use utils::grid::Grid;

fn main() {
    let input = include_str!("../../input/day12/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Grid<char> {
    let vs = src
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Grid::new(vs)
}

fn find_regions(grid: &Grid<char>) -> i64 {
    let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited: FxHashSet<(i32, i32)> = FxHashSet::default();
    let mut start: (i32, i32) = (0, 0);
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();

    let mut cost: i64 = 0;
    while visited.len() < grid.size() {
        for (x, y) in grid.iter_points() {
            if !visited.contains(&(x, y)) {
                start = (x, y);
                break;
            }
        }
        q.push_front(start);
        visited.insert((start.0, start.1));
        let plant = grid.at_unsafe(start.0, start.1);

        let mut area: i64 = 1;
        let mut perimeter: i64 = 0;
        let mut vv: Vec<(i32, i32)> = Vec::new();

        while let Some((x, y)) = q.pop_front() {
            for d in dirs.iter() {
                if !visited.contains(&(x + d.0, y + d.1))
                    && grid.in_bounds(x + d.0, y + d.1)
                    && grid.at_unsafe(x + d.0, y + d.1) == plant
                {
                    perimeter += 1;
                    q.push_front((x + d.0, y + d.1));
                    vv.push((x, y));
                }
            }
            if !visited.contains(&(x, y)) {
                area += 1;
                visited.insert((x, y));
            }
        }
        cost += area * ((4 * area) - (2 * perimeter));
    }
    cost
}

fn find_regions2(grid: &Grid<char>) -> i64 {
    let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited: FxHashSet<(i32, i32)> = FxHashSet::default();
    let mut start: (i32, i32) = (0, 0);
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();

    let mut cost: i64 = 0;
    while visited.len() < grid.size() {
        for (x, y) in grid.iter_points() {
            if !visited.contains(&(x, y)) {
                start = (x, y);
                break;
            }
        }
        q.push_front(start);
        visited.insert((start.0, start.1));
        let plant = grid.at_unsafe(start.0, start.1);

        let mut area: i64 = 0;
        let mut cpts: FxHashMap<(i32, i32), i64> = FxHashMap::default();
        let mut region: FxHashSet<(i32, i32)> = FxHashSet::default();

        while let Some((x, y)) = q.pop_front() {
            for d in dirs.iter() {
                if !region.contains(&(x + d.0, y + d.1))
                    && grid.in_bounds(x + d.0, y + d.1)
                    && grid.at_unsafe(x + d.0, y + d.1) == plant
                {
                    q.push_front((x + d.0, y + d.1));
                }
            }
            if !region.contains(&(x, y)) {
                area += 1;
                region.insert((x, y));
                visited.insert((x, y));

                *cpts.entry((x, y + 1)).or_default() += 1;
                *cpts.entry((x + 1, y + 1)).or_default() += 1;
                *cpts.entry((x, y)).or_default() += 1;
                *cpts.entry((x + 1, y)).or_default() += 1;
            }
        }

        let dia = |x, y, v| -> bool {
            if v == 2 {
                if region.contains(&(x, y))
                    && region.contains(&(x - 1, y - 1))
                    && (!region.contains(&(x - 1, y)) || !region.contains(&(x, y - 1)))
                {
                    assert!(v == 2);
                    return true;
                }
                if region.contains(&(x - 1, y))
                    && region.contains(&(x, y - 1))
                    && (!region.contains(&(x, y)) || !region.contains(&(x - 1, y - 1)))
                {
                    assert!(v == 2);
                    return true;
                }
                return false;
            }
            v % 2 != 0
        };

        let perimeter = cpts
            .clone()
            .into_iter()
            .filter(|((x, y), v)| dia(*x, *y, *v))
            .fold(0, |acc, (_, v)| acc + if v % 2 != 0 { 1 } else { 2 });

        cost += area * perimeter;
    }
    cost
}

fn part1(src: &str) -> Result<i64, Error> {
    let grid = parse_input(src);
    let total = find_regions(&grid);

    Ok(total)
}

fn part2(src: &str) -> Result<i64, Error> {
    let grid = parse_input(src);
    let total = find_regions2(&grid);

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        assert_eq!(1930, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        assert_eq!(1206, part2(src).unwrap());

        let src = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(236, part2(src).unwrap());

        let src = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(436, part2(src).unwrap());

        let src = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(368, part2(src).unwrap());
    }
}

//878098 <
//939698 >
