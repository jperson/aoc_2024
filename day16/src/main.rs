use anyhow::Error;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use utils::grid::Grid;

//for minheap
#[derive(Debug)]
struct Node(i64, (usize, (i32, i32)));

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

fn main() {
    let input = include_str!("../../input/day16/input.txt");

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

fn dijkstra(grid: &Grid<char>, start: (i32, i32), goal: (i32, i32)) -> (i64, i64) {
    const N: usize = 0;
    const E: usize = 1;
    const S: usize = 2;
    const W: usize = 3;

    let mut h: BinaryHeap<Node> = BinaryHeap::new();
    let mut dist: FxHashMap<(usize, (i32, i32)), i64> = FxHashMap::default();

    const ADJ: [[(usize, i64, (i32, i32)); 3]; 4] = [
        [(W, 1001, (-1, 0)), (N, 1, (0, -1)), (E, 1001, (1, 0))], // N
        [(N, 1001, (0, -1)), (E, 1, (1, 0)), (S, 1001, (0, 1))],  // E
        [(E, 1001, (1, 0)), (S, 1, (0, 1)), (W, 1001, (-1, 0))],  // S
        [(N, 1001, (0, -1)), (W, 1, (-1, 0)), (S, 1001, (0, 1))], // W
    ];

    h.push(Node(0, (E, start)));
    while let Some(Node(score, (dir, p))) = h.pop() {
        if !grid.in_bounds(p.0, p.1) || *grid.at_unsafe(p.0, p.1) == '#' {
            continue;
        }

        for i in 1..=3 {
            let xd = (i + dir).rem_euclid(4) as usize;
            if 1000 + score < *dist.entry((xd, p)).or_insert(i64::MAX) {
                *dist.entry((xd, p)).or_default() = 1000 + score;
            }
        }

        for (nd, weight, pt) in ADJ[dir] {
            let np = (p.0 + pt.0, p.1 + pt.1);
            if score + weight < *dist.entry((nd, np)).or_insert(i64::MAX) {
                *dist.entry((nd, np)).or_insert(i64::MAX) = score + weight;
                h.push(Node(score + weight, (nd, np)));
            }
        }
    }

    let mut min_score: i64 = *dist.entry((N, goal)).or_insert(i64::MAX);
    for d in N..W {
        min_score = min_score.min(*dist.entry((d, goal)).or_insert(i64::MAX));
    }

    //part 2 DFS back through graph
    let mut seats: FxHashSet<(i32, i32)> = FxHashSet::default();
    let mut rq: VecDeque<((i32, i32), usize, i64)> = VecDeque::new();
    rq.push_back((goal, N, 0));

    while let Some((p, d, s)) = rq.pop_front() {
        if p == start {
            seats.insert(p);
        }

        if !seats.contains(&p) {
            for (nd, nw, np) in ADJ[d] {
                let np = (p.0 - np.0, p.1 - np.1);
                if grid.in_bounds(np.0, np.1) && *grid.at_unsafe(np.0, np.1) != '#' {
                    for i in 0..4 {
                        if *dist.entry((i, np)).or_default() + nw + s == min_score {
                            seats.insert(p);
                            rq.push_front((np, nd, s + nw));
                        }
                    }
                }
            }
        }
    }

    (min_score, seats.len() as i64)
}

fn part1(src: &str) -> Result<i64, Error> {
    let grid = parse_input(src);
    let mut start: (i32, i32) = (0, 0);
    let mut goal: (i32, i32) = (0, 0);

    for (x, y) in grid.iter_points() {
        if *grid.at_unsafe(x, y) == 'S' {
            start = (x, y);
        }
        if *grid.at_unsafe(x, y) == 'E' {
            goal = (x, y);
        }
    }

    let (score, _) = dijkstra(&grid, start, goal);
    Ok(score)
}

fn part2(src: &str) -> Result<i64, Error> {
    let grid = parse_input(src);
    let mut start: (i32, i32) = (0, 0);
    let mut goal: (i32, i32) = (0, 0);

    for (x, y) in grid.iter_points() {
        if *grid.at_unsafe(x, y) == 'S' {
            start = (x, y);
        }
        if *grid.at_unsafe(x, y) == 'E' {
            goal = (x, y);
        }
    }

    let (_, seats) = dijkstra(&grid, start, goal);

    Ok(seats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(7036, part1(src).unwrap());

        let src = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(11048, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(45, part2(src).unwrap());

        let src = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(64, part2(src).unwrap());
    }
}
