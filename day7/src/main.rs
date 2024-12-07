use anyhow::Error;

fn main() {
    let input = include_str!("../../input/day7/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input(src: &str) -> Vec<(i64, Vec<i64>)> {
    src.lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(v, ts)| (v, ts.trim().split(" ").collect::<Vec<_>>()))
        .map(|(v, ts)| {
            (
                v.parse::<i64>().unwrap(),
                ts.into_iter()
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

fn solve(t: i64, target: i64, vs: &[i64]) -> bool {
    if vs.is_empty() {
        return t == target;
    }

    solve(t + vs[0], target, &vs[1..]) || solve(t * vs[0], target, &vs[1..])
}

fn part1(src: &str) -> Result<i64, Error> {
    Ok(parse_input(src)
        .into_iter()
        .filter(|(t, vs)| solve(vs[0], *t, &vs[1..]))
        .map(|(t, _)| t)
        .sum())
}

fn solve2(t: i64, target: i64, vs: &[i64]) -> bool {
    if vs.is_empty() {
        return t == target;
    }

    let tt: i64 = format!("{}{}", t, vs[0]).parse().unwrap();
    solve2(t + vs[0], target, &vs[1..])
        || solve2(t * vs[0], target, &vs[1..])
        || solve2(tt, target, &vs[1..])
}

fn part2(src: &str) -> Result<i64, Error> {
    Ok(parse_input(src)
        .into_iter()
        .filter(|(t, vs)| solve2(vs[0], *t, &vs[1..]))
        .map(|(t, _)| t)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(3749, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        assert_eq!(11387, part2(src).unwrap());
    }
}
