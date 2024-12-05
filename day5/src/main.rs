use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input/day5/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn part1(src: &str) -> Result<i32, Error> {
    let mut total = 0;
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let (rules, updates) = src.split_once("\n\n").ok_or(anyhow!("failed to parse"))?;

    for r in rules.lines() {
        let (p, b) = r.split_once("|").ok_or(anyhow!("failed to parse rule"))?;
        map.entry(p).or_insert(HashSet::new()).insert(b);
    }

    let updates: Vec<Vec<&str>> = updates.lines().map(|u| u.split(",").collect()).collect();

    for u in updates {
        if u.iter()
            .tuple_windows()
            .map(|(a, b)| map.get(a).map(|v| v.contains(b)) == Some(true))
            .all(|p| p)
        {
            total += u[u.len() / 2].parse::<i32>().expect("failed to parse");
        }
    }

    Ok(total)
}

fn fix_update(mut up: Vec<&str>, m: &HashMap<&str, HashSet<&str>>) -> i32 {
    let ul = HashSet::from_iter(up.iter().cloned());
    let empty = HashSet::new();

    up.sort_by(|a, b| {
        let al = m.get(a).unwrap_or(&empty);
        let bl = m.get(b).unwrap_or(&empty);

        let aa = &al.intersection(&ul).collect::<Vec<_>>().len();
        let bb = &bl.intersection(&ul).collect::<Vec<_>>().len();
        bb.cmp(aa)
    });

    return up[up.len() / 2].parse::<i32>().expect("failed to parse");
}

fn part2(src: &str) -> Result<i32, Error> {
    let mut total = 0;
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let (rules, updates) = src.split_once("\n\n").ok_or(anyhow!("failed to parse"))?;

    for r in rules.lines() {
        let (p, b) = r.split_once("|").ok_or(anyhow!("failed to parse rule"))?;
        map.entry(p).or_insert(HashSet::new()).insert(b);
    }

    let updates: Vec<Vec<&str>> = updates.lines().map(|u| u.split(",").collect()).collect();

    for u in updates {
        if !u
            .iter()
            .tuple_windows()
            .map(|(a, b)| map.get(a).map(|v| v.contains(b)) == Some(true))
            .all(|p| p)
        {
            total += fix_update(u, &map);
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(143, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(123, part2(src).unwrap());
    }
}
