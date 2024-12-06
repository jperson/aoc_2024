use anyhow::Error;
//use std::collections::HashSet;
use rustc_hash::FxHashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    x: i32,
    y: i32,
    d: Dir,
}

fn main() {
    let input = include_str!("../../input/day6/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn in_bounds(x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
    x >= 0 && y >= 0 && x < grid[0].len() as i32 && y < grid.len() as i32
}

fn next_dir(d: Dir) -> Dir {
    match d {
        Dir::UP => Dir::RIGHT,
        Dir::RIGHT => Dir::DOWN,
        Dir::DOWN => Dir::LEFT,
        Dir::LEFT => Dir::UP,
    }
}

fn peek(s: &State, grid: &Vec<Vec<char>>) -> Option<(State, char)> {
    let mut ns = s.clone();
    match s.d {
        Dir::UP if in_bounds(s.x, s.y - 1, &grid) => {
            ns.y -= 1;
            Some((ns, grid[ns.y as usize][ns.x as usize]))
        }
        Dir::RIGHT if in_bounds(s.x + 1, s.y, &grid) => {
            ns.x += 1;
            Some((ns, grid[ns.y as usize][ns.x as usize]))
        }
        Dir::DOWN if in_bounds(s.x, s.y + 1, &grid) => {
            ns.y += 1;
            Some((ns, grid[ns.y as usize][ns.x as usize]))
        }
        Dir::LEFT if in_bounds(s.x - 1, s.y, &grid) => {
            ns.x -= 1;
            Some((ns, grid[ns.y as usize][ns.x as usize]))
        }
        _ => None,
    }
}

fn has_cycle(mut s: State, grid: &Vec<Vec<char>>) -> bool {
    let mut vv: FxHashSet<State> = FxHashSet::default();
    vv.insert(s);

    while let Some((ns, c)) = peek(&s, &grid) {
        if vv.contains(&ns) {
            return true;
        }

        match c {
            '#' | 'O' => s.d = next_dir(ns.d),
            _ => s = ns,
        }
        vv.insert(s);
    }

    false
}

fn part1(src: &str) -> Result<i32, Error> {
    let mut grid: Vec<Vec<char>> = src.lines().map(|l| l.chars().collect()).collect();
    let mut total = 0;

    let mut state = State {
        x: 0,
        y: 0,
        d: Dir::UP,
    };

    'outer: for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            if grid[y as usize][x as usize] == '^' {
                grid[y as usize][x as usize] = 'X';
                state = State { x, y, d: Dir::UP };
                break 'outer;
            }
        }
    }

    let mut path: FxHashSet<(i32, i32)> = FxHashSet::default();

    while let Some((next, c)) = peek(&state, &grid) {
        if c == '#' {
            state.d = next_dir(state.d);
        } else {
            if c == '.' && !path.contains(&(next.x, next.y)) {
                path.insert((next.x, next.y));
                total += 1;
            }
            state = next;
        }
    }

    total += 1;

    Ok(total)
}

fn part2(src: &str) -> Result<i32, Error> {
    let mut grid: Vec<Vec<char>> = src.lines().map(|l| l.chars().collect()).collect();

    let mut state = State {
        x: 0,
        y: 0,
        d: Dir::UP,
    };

    'outer: for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            if grid[y as usize][x as usize] == '^' {
                state = State { x, y, d: Dir::UP };
                break 'outer;
            }
        }
    }

    let start = state.clone();
    let mut path: FxHashSet<(i32, i32)> = FxHashSet::default();

    //find guard path
    while let Some((next, c)) = peek(&state, &grid) {
        if c == '#' {
            state.d = next_dir(state.d);
        } else {
            if c == '.' && !path.contains(&(next.x, next.y)) {
                path.insert((next.x, next.y));
            }
            state = next;
        }
    }

    let mut total = 0;
    for (x, y) in path.iter() {
        grid[*y as usize][*x as usize] = 'O';
        if has_cycle(start, &grid) {
            total += 1;
        }
        grid[*y as usize][*x as usize] = '.';
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(41, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        assert_eq!(6, part2(src).unwrap());
    }
}
