use anyhow::{anyhow, Error};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use utils::grid::Grid;
use utils::nums;

const W: usize = 71;
const H: usize = 71;
const STEPS: usize = 1024;
const START: (i32, i32) = (0, 0);
const END: (i32, i32) = ((W - 1) as i32, (H - 1) as i32);

fn main() {
    let input = include_str!("../../input/day18/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {:?}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Vec<Vec<i32>> {
    src.lines().map(|l| utils::nums(l)).collect::<Vec<_>>()
}

//for minheap
#[derive(Debug)]
struct Node(i64, (i32, i32), FxHashSet<(i32, i32)>);

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.0.cmp(&self.0)
    }
}

fn dijkstra(
    grid: &mut Grid<char>,
    start: (i32, i32),
    goal: (i32, i32),
) -> (i64, FxHashSet<(i32, i32)>) {
    let mut h: BinaryHeap<Node> = BinaryHeap::new();
    let mut dist: FxHashMap<(i32, i32), i64> = FxHashMap::default();
    let mut path_to: FxHashMap<(i32, i32), FxHashSet<(i32, i32)>> = FxHashMap::default();

    const ADJ: [(i64, (i32, i32)); 4] = [(1, (0, -1)), (1, (1, 0)), (1, (0, 1)), (1, (-1, 0))];

    let path: FxHashSet<(i32, i32)> = FxHashSet::default();
    h.push(Node(0, start, path));

    while let Some(Node(score, p, path)) = h.pop() {
        if !grid.in_bounds(p.0, p.1) || *grid.at_unsafe(p.0, p.1) == '#' {
            continue;
        }

        if p == goal {
            *path_to.entry(p).or_default() = path.clone();
        }

        if score < *dist.entry(p).or_insert(i64::MAX) {
            *dist.entry(p).or_default() = score;
            *path_to.entry(p).or_default() = path.clone();
        }

        for (cost, pt) in ADJ {
            let np = (p.0 + pt.0, p.1 + pt.1);
            if grid.in_bounds(np.0, np.1) && *grid.at_unsafe(np.0, np.1) != '#' {
                if score + cost < *dist.entry(np).or_insert(i64::MAX) {
                    *dist.entry(np).or_insert(i64::MAX) = cost + score;
                    let mut npath = path.clone();
                    npath.insert(np);
                    *path_to.entry(p).or_default() = npath.clone();
                    h.push(Node(score + cost, np, npath));
                }
            }
        }
    }

    let min_score: i64 = *dist.entry(goal).or_insert(i64::MAX);
    (min_score, path_to.entry(goal).or_default().clone())
}

fn part1(src: &str) -> Result<i64, Error> {
    let blocks = parse_input(src);

    let grid: &[char; W * H] = &['.'; W * H];
    let mut grid: Grid<char> = Grid::from_vec(&grid.to_vec(), W as i32, H as i32);

    for b in blocks.iter().take(STEPS) {
        if let &[x, y] = &b[..] {
            *grid.at_mut(x, y) = '#';
        }
    }
    let (score, path) = dijkstra(&mut grid, START, END);
    for p in path {
        *grid.at_mut(p.0, p.1) = 'O';
    }
    Ok(score)
}

fn part2(src: &str) -> Result<(i32, i32), Error> {
    let blocks = parse_input(src);

    let grid: &[char; W * H] = &['.'; W * H];
    let mut grid: Grid<char> = Grid::from_vec(&grid.to_vec(), W as i32, H as i32);
    let (_, mut path) = dijkstra(&mut grid, START, END);

    for b in blocks {
        if let &[x, y] = &b[..] {
            *grid.at_mut(x, y) = '#';

            if path.contains(&(x, y)) {
                let (score, p) = dijkstra(&mut grid, START, END);
                if score == i64::MAX {
                    return Ok((x, y));
                }
                path = p;
            }
        }
    }
    Err(anyhow!("NOT BLOCKED"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(22, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!((6, 1), part2(src).unwrap());
    }
}
