use anyhow::Error;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utils::grid::Grid;

fn main() {
    let input = include_str!("../../input/day8/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input(src: &str) -> Grid<char> {
    let vs = src
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Grid::new(vs)
}

fn distance_signed(a1: (i32, i32), a2: (i32, i32)) -> (i32, i32) {
    ((a1.0 - a2.0), (a1.1 - a2.1))
}

fn add_points(a1: (i32, i32), a2: (i32, i32)) -> (i32, i32) {
    (a1.0 + a2.0, a1.1 + a2.1)
}

fn part1(src: &str) -> Result<i64, Error> {
    let grid = parse_input(src);

    //find antennas
    let mut ant_map: FxHashMap<char, Vec<(i32, i32)>> = FxHashMap::default();
    for (x, y) in grid.iter_points() {
        if grid.at(x, y) != Some(&'.') {
            let a = grid.at(x, y).unwrap();
            ant_map.entry(*a).or_default().push((x, y));
        }
    }

    let mut antinodes: FxHashSet<(i32, i32)> = FxHashSet::default();

    for (_, ats) in &ant_map {
        for cs in ats.iter().combinations(2) {
            if let [a1, a2] = cs[..] {
                let (dx, dy) = distance_signed(*a1, *a2);

                let ant1 = add_points(*a1, (dx, dy));
                if grid.in_bounds(ant1.0, ant1.1) {
                    antinodes.insert(ant1);
                }

                let ant1 = add_points(*a2, (-dx, -dy));
                if grid.in_bounds(ant1.0, ant1.1) {
                    antinodes.insert(ant1);
                }
            }
        }
    }
    Ok(antinodes.len() as i64)
}

fn part2(src: &str) -> Result<i64, Error> {
    let grid = parse_input(src);

    //find antennas
    let mut ant_map: FxHashMap<char, Vec<(i32, i32)>> = FxHashMap::default();
    let mut ant_count: usize = 0;

    for (x, y) in grid.iter_points() {
        if grid.at(x, y) != Some(&'.') {
            let a = grid.at(x, y).unwrap();
            ant_count += 1;
            ant_map.entry(*a).or_default().push((x, y));
        }
    }

    let mut antinodes: FxHashSet<(i32, i32)> = FxHashSet::default();

    for (_, ats) in &ant_map {
        for cs in ats.iter().combinations(2) {
            if let [a1, a2] = cs[..] {
                for pt in grid.line(*a1, *a2) {
                    if pt != *a1 && pt != *a2 && grid.at(pt.0, pt.1) == Some(&'.') {
                        antinodes.insert(pt);
                    }
                }
            }
        }
    }

    Ok((ant_count + antinodes.len()) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let src2 = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

        assert_eq!(14, part1(src).unwrap());
        assert_eq!(2, part1(src2).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

        let src2 = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        assert_eq!(9, part2(src).unwrap());
        assert_eq!(34, part2(src2).unwrap());
    }
}
