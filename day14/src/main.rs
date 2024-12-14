use anyhow::Error;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use utils::grid::Grid;
use utils::nums;

fn main() {
    let input = include_str!("../../input/day14/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

fn parse_input<'a>(src: &'a str) -> Vec<Robot> {
    src.lines()
        .map(|l| {
            let (a, b) = l.split_once(" ").unwrap();
            let [x, y] = &nums(a)[..] else {
                panic!("failed to parse")
            };
            let [vx, vy] = &nums(b)[..] else {
                panic!("failed to parse")
            };
            Robot {
                x: *x,
                y: *y,
                vx: *vx,
                vy: *vy,
            }
        })
        .collect::<Vec<Robot>>()
}

fn part1(src: &str) -> Result<i64, Error> {
    // const WIDTH: i64 = 11;
    // const HEIGHT: i64 = 7;
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;

    let mut robots = parse_input(src);

    for r in robots.iter_mut() {
        r.x += if r.vx < 0 { WIDTH + r.vx } else { r.vx } * 100;
        r.x %= WIDTH;

        r.y += if r.vy < 0 { HEIGHT + r.vy } else { r.vy } * 100;
        r.y %= HEIGHT;
    }

    let mut quads: [i64; 4] = [0, 0, 0, 0];
    for r in robots {
        if r.x < (WIDTH / 2) && r.y < (HEIGHT / 2) {
            quads[0] += 1;
        } else if r.x > (WIDTH / 2) && r.y < (HEIGHT / 2) {
            quads[1] += 1;
        } else if r.x < (WIDTH / 2) && r.y > (HEIGHT / 2) {
            quads[2] += 1;
        } else if r.x > (WIDTH / 2) && r.y > (HEIGHT / 2) {
            quads[3] += 1;
        }
    }

    println!("{:?}", quads);

    Ok(quads.iter().product())
}

fn connected(g: &Grid<char>) -> bool {
    let mut seen: FxHashSet<(i32, i32)> = FxHashSet::default();
    let dirs: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut max_connect = 0;
    for y in 0..g.height {
        for x in 0..g.width {
            if !seen.contains(&(x, y)) && *g.at_unsafe(x, y) == '#' {
                let mut connected = 0;
                let mut q: VecDeque<(i32, i32)> = VecDeque::new();
                q.push_back((x, y));

                while let Some(p) = q.pop_front() {
                    seen.insert(p);
                    if *g.at_unsafe(p.0, p.1) != '#' {
                        if connected > max_connect {
                            max_connect = connected;
                            connected = 0;
                        }
                        continue;
                    }

                    connected += 1;
                    if connected > 100 {
                        return true;
                    }
                    for d in dirs {
                        if !seen.contains(&(p.0 + d.0, p.1 + d.1))
                            && g.in_bounds(p.0 + d.0, p.1 + d.1)
                        {
                            if *g.at_unsafe(p.0 + d.0, p.1 + d.1) == '#' {
                                q.push_back((p.0 + d.0, p.1 + d.1));
                            }
                        }
                    }
                }
            } else {
                seen.insert((x, y));
            }
        }
    }
    false
}

fn part2(src: &str) -> Result<i64, Error> {
    // const WIDTH: i64 = 11;
    // const HEIGHT: i64 = 7;
    const WIDTH: i64 = 101;
    const HEIGHT: i64 = 103;

    let mut robots = parse_input(src);

    let mut space: FxHashMap<(i64, i64), i64> = FxHashMap::default();

    let vs: Vec<char> = [' '; (WIDTH * HEIGHT) as usize].to_vec();
    let mut grid: Grid<char> = Grid::from_vec(&vs, WIDTH as i32, HEIGHT as i32);
    for n in 1..1000000 {
        for r in robots.iter_mut() {
            *space.entry((r.x, r.y)).or_default() -= 1;
            let v = grid.at_mut(r.x as i32, r.y as i32);
            if *space.entry((r.x, r.y)).or_default() <= 0 {
                *space.entry((r.x, r.y)).or_default() = 0;
                *v = ' ';
            }

            r.x += if r.vx < 0 { WIDTH + r.vx } else { r.vx };
            r.x %= WIDTH;

            r.y += if r.vy < 0 { HEIGHT + r.vy } else { r.vy };
            r.y %= HEIGHT;

            let v = grid.at_mut(r.x as i32, r.y as i32);
            *v = '#';
            *space.entry((r.x, r.y)).or_default() += 1;
        }
        if connected(&grid) {
            println!("{}", grid);
            return Ok(n);
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        //assert_eq!(12, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        //assert_eq!(0, part2(src).unwrap());
    }
}
