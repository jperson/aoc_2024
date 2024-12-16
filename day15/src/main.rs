use anyhow::Error;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::ops::Range;
use utils::grid::Grid;
use utils::nums;

fn main() {
    let input = include_str!("../../input/day15/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> (Grid<char>, Vec<char>) {
    let (grid, moves) = src.split_once("\n\n").unwrap();
    let grid: Vec<Vec<char>> = grid
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let grid: Grid<char> = Grid::new(grid);
    let moves: Vec<char> = moves
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    (grid, moves)
}

fn parse_input2<'a>(src: &'a str) -> (Grid<char>, Vec<char>) {
    let (grid, moves) = src.split_once("\n\n").unwrap();
    let grid: Vec<Vec<char>> = grid
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let grid: Grid<char> = Grid::new(grid);

    let mut new_grid: Vec<char> = Vec::new();
    for (x, y) in grid.iter_points() {
        if *grid.at_unsafe(x, y) == '.' || *grid.at_unsafe(x, y) == '#' {
            new_grid.push(*grid.at_unsafe(x, y));
            new_grid.push(*grid.at_unsafe(x, y));
        } else if *grid.at_unsafe(x, y) == 'O' {
            new_grid.push('[');
            new_grid.push(']');
        } else {
            new_grid.push(*grid.at_unsafe(x, y));
            new_grid.push('.');
        }
    }
    let grid: Grid<char> = Grid::from_vec(&new_grid, grid.width * 2, grid.height);

    //let moves: Vec<char> = moves.trim().chars().collect::<Vec<char>>();
    let moves: Vec<char> = moves
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    (grid, moves)
}

fn move_robot(grid: &mut Grid<char>, curx: i32, cury: i32, d: (i32, i32)) -> (i32, i32) {
    if grid.at_unsafe(curx + d.0, cury + d.1) == &'#' {
        return (curx, cury);
    }
    if grid.at_unsafe(curx + d.0, cury + d.1) == &'.' {
        *grid.at_mut(curx, cury) = '.';
        *grid.at_mut(curx + d.0, cury + d.1) = '@';
        return (curx + d.0, cury + d.1);
    }
    if grid.at_unsafe(curx + d.0, cury + d.1) == &'O' {
        let (mut nx, mut ny) = (curx + d.0, cury + d.1);
        while grid.in_bounds(nx, ny) {
            if grid.at_unsafe(nx, ny) == &'.' {
                while (nx, ny) != (curx, cury) {
                    *grid.at_mut(nx, ny) = *grid.at_unsafe(nx - d.0, ny - d.1);
                    (nx, ny) = (nx - d.0, ny - d.1);
                }
                *grid.at_mut(curx, cury) = '.';
                return (curx + d.0, cury + d.1);
            } else if grid.at_unsafe(nx, ny) == &'#' {
                return (curx, cury);
            }
            (nx, ny) = (nx + d.0, ny + d.1);
        }
    }
    return (curx, cury);
}

fn part1(src: &str) -> Result<i32, Error> {
    let (mut grid, moves) = parse_input(src);

    const UP: (i32, i32) = (0, -1);
    const RIGHT: (i32, i32) = (1, 0);
    const DOWN: (i32, i32) = (0, 1);
    const LEFT: (i32, i32) = (-1, 0);

    let mut curx: i32 = 0;
    let mut cury: i32 = 0;

    for (x, y) in grid.iter_points() {
        if grid.at(x, y) == Some(&'@') {
            (curx, cury) = (x, y);
            break;
        }
    }

    for m in moves.iter() {
        if *m == '^' {
            (curx, cury) = move_robot(&mut grid, curx, cury, UP);
        }
        if *m == '>' {
            (curx, cury) = move_robot(&mut grid, curx, cury, RIGHT);
        }
        if *m == 'v' {
            (curx, cury) = move_robot(&mut grid, curx, cury, DOWN);
        }
        if *m == '<' {
            (curx, cury) = move_robot(&mut grid, curx, cury, LEFT);
        }
    }

    Ok(grid.iter_points().fold(0, |acc, (x, y)| {
        if *grid.at_unsafe(x, y) == 'O' {
            acc + 100 * y + x
        } else {
            acc
        }
    }))
}

fn hmove(grid: &mut Grid<char>, mut x: i32, y: i32, dx: i32) -> bool {
    let mut nx = x;
    while *grid.at_unsafe(nx + dx, y) != '.' {
        if *grid.at_unsafe(nx + dx, y) == '#' {
            return false;
        }
        nx += dx;
    }

    nx += dx;
    while *grid.at_unsafe(nx, y) != '@' {
        *grid.at_mut(nx, y) = *grid.at_unsafe(nx - dx, y);
        nx -= dx;
    }
    true
}

fn vmove(grid: &mut Grid<char>, boxes: &FxHashSet<(i32, i32)>, y: i32, dy: i32) -> bool {
    let mut vbs: FxHashSet<(i32, i32)> = FxHashSet::default();
    for b in boxes {
        if *grid.at_unsafe(b.0, y + dy) == '#' || *grid.at_unsafe(b.1, y + dy) == '#' {
            return false;
        }

        if *grid.at_unsafe(b.0, y + dy) == '[' && *grid.at_unsafe(b.1, y + dy) == ']' {
            vbs.insert(*b);
            continue;
        }
        if *grid.at_unsafe(b.0, y + dy) == ']' && *grid.at_unsafe(b.1, y + dy) == '[' {
            vbs.insert((b.0 - 1, b.1 + 1));
        }
        if *grid.at_unsafe(b.0, y + dy) == ']' {
            vbs.insert((b.0 - 1, b.0));
        }
        if *grid.at_unsafe(b.1, y + dy) == '[' {
            vbs.insert((b.1, b.1 + 1));
        }
    }

    if vbs.len() > 0 && !vmove(grid, &vbs, y + dy, dy) {
        return false;
    } else {
        for b in vbs {
            for x in b.0..=b.1 {
                *grid.at_mut(x, y + dy) = '.';
            }
        }
        for b in boxes {
            for x in b.0..=b.1 {
                if *grid.at_unsafe(x, y + dy) == '.' {
                    *grid.at_mut(x, y + dy) = *grid.at_unsafe(x, y);
                }
            }
        }
        return true;
    }
}

fn part2(src: &str) -> Result<i32, Error> {
    let (mut grid, moves) = parse_input2(src);

    let mut curx: i32 = 0;
    let mut cury: i32 = 0;

    for (x, y) in grid.iter_points() {
        if grid.at(x, y) == Some(&'@') {
            (curx, cury) = (x, y);
            break;
        }
    }

    for m in moves.iter() {
        if *m == '>' {
            if hmove(&mut grid, curx, cury, 1) {
                *grid.at_mut(curx + 1, cury) = '@';
                *grid.at_mut(curx, cury) = '.';
                curx += 1;
            }
        }
        if *m == '<' {
            if hmove(&mut grid, curx, cury, -1) {
                *grid.at_mut(curx - 1, cury) = '@';
                *grid.at_mut(curx, cury) = '.';
                curx -= 1;
            }
        }
        if *m == '^' {
            if *grid.at_unsafe(curx, cury - 1) == '#' {
                continue;
            }
            if *grid.at_unsafe(curx, cury - 1) == '.' {
                *grid.at_mut(curx, cury - 1) = '@';
                *grid.at_mut(curx, cury) = '.';
                cury -= 1;
                continue;
            }

            let r = if *grid.at_unsafe(curx, cury - 1) == '[' {
                (curx, curx + 1)
            } else {
                (curx - 1, curx)
            };

            let mut vbs: FxHashSet<(i32, i32)> = FxHashSet::default();
            vbs.insert(r);

            if vmove(&mut grid, &vbs, cury - 1, -1) {
                for x in r.0..=r.1 {
                    *grid.at_mut(x as i32, cury - 1) = '.'; //*grid.at_unsafe(x as i32, cury);
                }
                *grid.at_mut(curx, cury) = '.';
                *grid.at_mut(curx, cury - 1) = '@';
                cury -= 1;
            }
        }
        if *m == 'v' {
            if *grid.at_unsafe(curx, cury + 1) == '#' {
                continue;
            }
            if *grid.at_unsafe(curx, cury + 1) == '.' {
                *grid.at_mut(curx, cury + 1) = '@';
                *grid.at_mut(curx, cury) = '.';
                cury += 1;
                continue;
            }

            let r = if *grid.at_unsafe(curx, cury + 1) == '[' {
                (curx, 1 + curx)
            } else {
                (curx - 1, curx)
            };

            let mut vbs: FxHashSet<(i32, i32)> = FxHashSet::default();
            vbs.insert(r);

            if vmove(&mut grid, &vbs, cury + 1, 1) {
                for x in r.0..=r.1 {
                    *grid.at_mut(x as i32, cury + 1) = '.'; //*grid.at_unsafe(x as i32, cury);
                }
                *grid.at_mut(curx, cury) = '.';
                *grid.at_mut(curx, cury + 1) = '@';
                cury += 1;
            }
        }
    }
    Ok(grid.iter_points().fold(0, |acc, (x, y)| {
        if *grid.at_unsafe(x, y) == '[' {
            acc + 100 * y + x
        } else {
            acc
        }
    }))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(2028, part1(src).unwrap());

        let src = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(10092, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!(9021, part2(src).unwrap());
    }
}
