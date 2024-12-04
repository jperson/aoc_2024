use anyhow::Error;

fn main() {
    let input = include_str!("../../input/day4/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

const DIRECTION: [(i32, i32); 8] = [
    (1, 1),
    (-1, -1),
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (-1, 1),
    (1, -1),
];

fn grid_walk(grid: &Vec<Vec<char>>, dir: (i32, i32), x: i32, y: i32, word: &[char]) -> bool {
    if word.len() == 0 {
        return true;
    }

    if x < 0 || x as usize >= grid[0].len() || y < 0 || y as usize >= grid.len() {
        return false;
    }

    if grid[y as usize][x as usize] == word[0] {
        grid_walk(grid, dir, x + dir.0, y + dir.1, &word[1..])
    } else {
        false
    }
}

fn part1(src: &str) -> Result<i32, Error> {
    let grid: Vec<Vec<char>> = src.lines().map(|l| l.chars().collect()).collect();
    let word = &"XMAS".chars().collect::<Vec<char>>();
    let mut total = 0;

    for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            total += DIRECTION
                .map(|d| grid_walk(&grid, d, x, y, &word))
                .iter()
                .filter(|p| **p)
                .count();
        }
    }

    Ok(total as i32)
}

fn part2(src: &str) -> Result<i32, Error> {
    let grid: Vec<Vec<char>> = src.lines().map(|l| l.chars().collect()).collect();
    let mut total = 0;

    for y in 1..(grid.len() - 1) {
        for x in 1..(grid[0].len() - 1) {
            if grid[y][x] == 'A' {
                let vs: Vec<char> = vec![
                    grid[y - 1][x - 1],
                    grid[y - 1][x + 1],
                    grid[y + 1][x - 1],
                    grid[y + 1][x + 1],
                ];
                let ws: String = vs.into_iter().collect();

                if ws == "MMSS" || ws == "SSMM" || ws == "MSMS" || ws == "SMSM" {
                    total += 1;
                }
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = part1(src).unwrap();
        assert_eq!(18, result);
    }

    #[test]
    fn test_part_2() {
        let src = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = part2(src).unwrap();
        assert_eq!(9, result);
    }
}
