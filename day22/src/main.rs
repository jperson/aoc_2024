use anyhow::Error;
use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!("../../input/day22/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Vec<i64> {
    src.lines().map(|l| l.parse().unwrap()).collect()
}

fn mix(secret: i64, n: i64) -> i64 {
    secret ^ n
}

fn prune(secret: i64) -> i64 {
    secret % 16777216
}

fn evolve(secret: i64) -> i64 {
    let mut ev = secret * 64;
    ev = mix(secret, ev);
    ev = prune(ev);
    ev = mix(ev, ev / 32);
    ev = prune(ev);
    ev = mix(ev, ev * 2048);
    prune(ev)
}

fn price(secret: i64) -> i64 {
    secret % 10
}

fn diff(mut secret: i64, n: i64, ms: &mut FxHashMap<(i64, i64, i64, i64), i64>) {
    let mut prev = price(secret);
    let mut ds: Vec<i64> = Vec::new();
    let mut cs: FxHashMap<(i64, i64, i64, i64), i64> = FxHashMap::default();

    for i in 0..n as usize {
        secret = evolve(secret);
        let cur = price(secret);
        ds.push(cur - prev);

        if i > 4 {
            let key: (i64, i64, i64, i64) = (ds[i - 3], ds[i - 2], ds[i - 1], ds[i]);
            if !cs.contains_key(&key) {
                *cs.entry(key).or_default() = cur;
            }
        }
        prev = cur;
    }

    for (k, v) in cs.into_iter() {
        *ms.entry(k).or_default() += v;
    }
}

fn part1(src: &str) -> Result<i64, Error> {
    let secrets = parse_input(src);
    let mut total: i64 = 0;

    for s in secrets {
        let mut m = s;
        for _ in 0..2000 {
            m = evolve(m);
        }
        total += m;
    }

    Ok(total)
}

fn part2(src: &str) -> Result<i64, Error> {
    let secrets = parse_input(src);
    let mut ms: FxHashMap<(i64, i64, i64, i64), i64> = FxHashMap::default();

    for s in secrets {
        diff(s, 2000, &mut ms);
    }
    let (_, v) = ms.into_iter().max_by_key(|entry| entry.1).unwrap();

    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "1
10
100
2024";
        assert_eq!(37327623, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "1
2
3
2024";
        assert_eq!(23, part2(src).unwrap());
    }

    #[test]
    fn test_mix() {
        assert_eq!(37, mix(42, 15));
    }

    #[test]
    fn test_prune() {
        assert_eq!(16113920, prune(100000000))
    }

    #[test]
    fn test_evolve() {
        let mut start = 123;
        for _ in 0..10 {
            start = evolve(start);
        }
        assert_eq!(5908254, start);
    }

    #[test]
    fn test_price() {
        let mut start = 123;
        for _ in 0..10 {
            start = evolve(start);
        }
        assert_eq!(4, price(start));
    }
}
