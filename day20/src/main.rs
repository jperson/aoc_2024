use anyhow::Error;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use utils::grid::Grid;

fn main() {
    let input = include_str!("../../input/day20/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Grid<char> {
    let grid: Vec<Vec<char>> = src
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let grid: Grid<char> = Grid::new(grid);
    grid
}

#[inline]
fn dist(a: (i32, i32), b: (i32, i32)) -> i64 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as i64
}

fn bfs(g: &mut Grid<char>, start: (i32, i32), goal: (i32, i32)) -> Vec<((i32, i32), i64)> {
    const ADJ: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut q: VecDeque<((i32, i32), i64)> = VecDeque::from_iter(vec![(start, 0)]);
    let mut visited: FxHashSet<(i32, i32)> = FxHashSet::default();
    let mut path: Vec<((i32, i32), i64)> = Vec::new();

    while let Some((p, steps)) = q.pop_front() {
        if p == goal {
            path.push((p, steps));
            return path;
        }

        if visited.contains(&p) {
            continue;
        }
        visited.insert(p);
        path.push((p, steps));

        for a in ADJ {
            let np = (a.0 + p.0, a.1 + p.1);
            if g.in_bounds(np.0, np.1) && g[(np.0 as usize, np.1 as usize)] != '#' {
                q.push_back((np, steps + 1));
            }
        }
    }

    path
}

fn part1(src: &str) -> Result<i64, Error> {
    let mut grid = parse_input(src);
    let mut total = 0;

    let start = grid.find(&'S').unwrap();
    let start: (i32, i32) = (start.0 as i32, start.1 as i32);

    let goal = grid.find(&'E').unwrap();
    let goal: (i32, i32) = (goal.0 as i32, goal.1 as i32);

    let visited = bfs(&mut grid, start, goal);

    for (i, (sp, sd)) in visited.iter().enumerate() {
        if i + 100 < visited.len() {
            for (ep, ed) in &visited[i + 100..] {
                if dist(*sp, *ep) == 2 && ed - sd - 2 >= 100 {
                    total += 1;
                }
            }
        }
    }
    Ok(total)
}

fn part2(src: &str) -> Result<i64, Error> {
    let mut grid = parse_input(src);
    let mut total = 0;

    let start = grid.find(&'S').unwrap();
    let start: (i32, i32) = (start.0 as i32, start.1 as i32);

    let goal = grid.find(&'E').unwrap();
    let goal: (i32, i32) = (goal.0 as i32, goal.1 as i32);

    let visited = bfs(&mut grid, start, goal);

    for (i, (sp, sd)) in visited.iter().enumerate() {
        if i + 100 < visited.len() {
            for (ep, ed) in &visited[i + 100..] {
                let nd = dist(*sp, *ep);
                if nd <= 20 && ed - sd - nd >= 100 {
                    total += 1;
                }
            }
        }
    }

    Ok(total)
}
