use anyhow::Error;
use memoize::memoize;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../input/day21/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Vec<Vec<char>> {
    src.lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
fn keymap(c: char) -> (i8, i8) {
    match c {
        'A' => (2, 3),
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        _ => panic!("not found"),
    }
}

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
fn dirmap(c: char) -> (i8, i8) {
    match c {
        'A' => (2, 0),
        '^' => (1, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("not found"),
    }
}

const ADJ: &[(char, (i8, i8)); 4] = &[('^', (0, -1)), ('>', (1, 0)), ('v', (0, 1)), ('<', (-1, 0))];

fn bfs(start: (i8, i8), end: (i8, i8), wh: (i8, i8), blank: (i8, i8)) -> Vec<Vec<char>> {
    let mut min_dist: i64 = i64::MAX;
    let mut paths: Vec<Vec<char>> = Vec::new();
    let mut visited: FxHashSet<(i8, i8)> = FxHashSet::default();

    let mut q: VecDeque<((i8, i8), i64, Vec<char>)> = VecDeque::new();
    q.push_back((start, 0, vec![]));

    while let Some((p, steps, path)) = q.pop_front() {
        if steps > min_dist {
            break;
        }

        if p == end {
            min_dist = steps;
            let mut p = path.clone();
            p.push('A');
            paths.push(p);
            continue;
        }

        for (c, a) in ADJ {
            let np = (p.0 + a.0, p.1 + a.1);
            if !visited.contains(&np)
                && np.0 >= 0
                && np.0 < wh.0
                && np.1 >= 0
                && np.1 < wh.1
                && np != blank
            {
                let mut npath = path.clone();
                npath.push(*c);
                q.push_back((np, steps + 1, npath));
            }
        }
        visited.insert(p);
    }

    paths
}

#[memoize]
fn search_dir_pad(code: Vec<char>, depth: i64) -> i64 {
    const DMAP_DIM: (i8, i8) = (3, 2);
    const DMAP_BLANK: (i8, i8) = (0, 0);

    let mut total: i64 = 0;
    let mut start: char = 'A';

    if depth == 0 {
        return code.len() as i64;
    }

    for c in &code {
        let paths = bfs(dirmap(start), dirmap(*c), DMAP_DIM, DMAP_BLANK);
        start = *c;
        total += paths.iter().fold(i64::MAX, |acc, p| {
            acc.min(search_dir_pad(p.to_vec(), depth - 1))
        });
    }

    total
}

fn path_complexity(codes: &Vec<Vec<char>>, depth: i64) -> i64 {
    const KEYMAP_DIM: (i8, i8) = (3, 4);
    const KEYMAP_BLANK: (i8, i8) = (0, 3);

    let mut score: i64 = 0;

    for code in codes {
        let mut start: char = 'A';

        let mut total = 0;
        for c in code {
            let paths = bfs(keymap(start), keymap(*c), KEYMAP_DIM, KEYMAP_BLANK);
            start = *c;
            total += paths.iter().fold(i64::MAX, |acc, p| {
                acc.min(search_dir_pad(p.to_vec(), depth))
            });
        }

        let n: i64 = code
            .iter()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();
        score += total * n;
    }

    score
}

fn part1(src: &str) -> Result<i64, Error> {
    let codes = parse_input(src);
    Ok(path_complexity(&codes, 2))
}

fn part2(src: &str) -> Result<i64, Error> {
    let codes = parse_input(src);
    Ok(path_complexity(&codes, 25))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "029A
980A
179A
456A
379A";
        assert_eq!(126384, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "";
        assert_eq!(0, part2(src).unwrap());
    }
}
