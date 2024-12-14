use anyhow::Error;
use nalgebra::*;

fn main() {
    let input = include_str!("../../input/day13/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    p: (i64, i64),
}

fn parse_input<'a>(src: &'a str) -> Vec<Machine> {
    let ms = src.split("\n\n").collect::<Vec<_>>();
    let mut machines: Vec<Machine> = Vec::new();
    let ms = ms
        .into_iter()
        .map(|m| m.split("\n").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for m in ms {
        let a = m[0].split_once(",").unwrap();
        let a = (
            a.0[12..].parse().expect("parse"),
            a.1[3..].parse().expect("parse"),
        );

        let b = m[1].split_once(",").unwrap();
        let b = (
            b.0[12..].parse().expect("parse"),
            b.1[3..].parse().expect("parse"),
        );

        let p = m[2].split_once(",").unwrap();
        let p = (
            p.0[9..].parse().expect("parse"),
            p.1[3..].parse().expect("parse"),
        );
        machines.push(Machine { a, b, p });
    }

    machines
}

fn combo(a: (i64, i64), b: (i64, i64), p: (i64, i64)) -> Option<(i64, i64)> {
    let m = Matrix2::new(a.0 as f64, b.0 as f64, a.1 as f64, b.1 as f64);
    let q = Vector2::new(p.0 as f64, p.1 as f64);

    let decomp = m.lu();
    let x = decomp.solve(&q).expect("failedl to solve");
    let (na, nb) = (x[0].round() as i64, x[1].round() as i64);

    if (na * a.0 + nb * b.0, na * a.1 + nb * b.1) == p {
        Some((na, nb))
    } else {
        None
    }
}

fn part1(src: &str) -> Result<i64, Error> {
    let machines = parse_input(src);
    let mut total: i64 = 0;

    for m in machines {
        if let Some((a, b)) = combo(m.a, m.b, m.p) {
            total += 3 * a + b;
        } else {
            continue;
        }
    }

    Ok(total)
}

fn part2(src: &str) -> Result<i64, Error> {
    let machines = parse_input(src);
    let machines = machines
        .iter()
        .map(|m| Machine {
            a: m.a,
            b: m.b,
            p: (10000000000000 + m.p.0, 10000000000000 + m.p.1),
        })
        .collect::<Vec<_>>();

    let mut total: i64 = 0;
    for m in machines {
        if let Some((a, b)) = combo(m.a, m.b, m.p) {
            total += (3 * a + b) as i64;
        } else {
            continue;
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!(480, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(875318608908, part2(src).unwrap());
    }
}
