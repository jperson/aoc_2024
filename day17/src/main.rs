use anyhow::{anyhow, Error};

#[derive(Debug, Default, Clone)]
enum Ins {
    ADV(u8),
    BXL(u8),
    BST(u8),
    JNZ(u8),
    BXC(u8),
    OUT(u8),
    BDV(u8),
    CDV(u8),
    #[default]
    EMPTY,
}

#[derive(Debug, Default, Clone)]
struct Program {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    ip: usize,
    input: Vec<Ins>,
    p: Vec<i64>,
}

fn main() {
    let input = include_str!("../../input/day17/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Program {
    let mut pgm = Program::default();

    if let Some((reg, ins)) = src.split_once("\n\n") {
        let [a, b, c, ..] = reg.split("\n").collect::<Vec<_>>()[..] else {
            panic!("failed to parse regs")
        };

        if let Some((_, a)) = a.split_once(": ") {
            pgm.reg_a = a.parse().expect("failed to parse reg a");
        }
        if let Some((_, b)) = b.split_once(": ") {
            pgm.reg_b = b.parse().expect("failed to parse reg a");
        }
        if let Some((_, c)) = c.split_once(": ") {
            pgm.reg_c = c.parse().expect("failed to parse reg a");
        }

        if let Some((_, ins)) = ins.split_once(" ") {
            let ins = ins
                .trim()
                .split(",")
                .map(|s| s.parse::<u8>().expect("failed to parse"))
                .collect::<Vec<_>>();

            pgm.p = ins.clone().into_iter().map(|v| v as i64).collect();

            let mut ins: &[u8] = &ins[..];
            while let [a, b, rest @ ..] = &ins[..] {
                pgm.input.push(match a {
                    0 => Ins::ADV(*b),
                    1 => Ins::BXL(*b),
                    2 => Ins::BST(*b),
                    3 => Ins::JNZ(*b),
                    4 => Ins::BXC(*b),
                    5 => Ins::OUT(*b),
                    6 => Ins::BDV(*b),
                    7 => Ins::CDV(*b),
                    _ => Ins::EMPTY,
                });
                ins = rest;
            }
        }
    }
    pgm
}

fn combo(p: &Program, op: u8) -> i64 {
    match op {
        0 | 1 | 2 | 3 => op as i64,
        4 => p.reg_a,
        5 => p.reg_b,
        6 => p.reg_c,
        _ => panic!("invalid operand"),
    }
}

fn run(p: &mut Program) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();
    loop {
        if p.ip > p.input.len() - 1 {
            break;
        }
        //count += 1;
        match p.input[p.ip] {
            Ins::ADV(v) => {
                let n = p.reg_a;
                let d = 2i64.pow(combo(p, v) as u32);
                p.reg_a = (n / d) as i64;
            }
            Ins::BXL(v) => p.reg_b ^= (v as i64),
            Ins::BST(v) => p.reg_b = combo(p, v) % 8,
            Ins::JNZ(v) => {
                if p.reg_a != 0 {
                    p.ip = v as usize;
                    continue;
                }
            }
            Ins::BXC(_) => p.reg_b ^= p.reg_c,
            Ins::OUT(v) => {
                output.push(combo(p, v) % 8);
            }
            Ins::BDV(v) => {
                let n = p.reg_a;
                let d = 2i64.pow(combo(p, v) as u32);
                p.reg_b = (n / d) as i64;
            }
            Ins::CDV(v) => {
                let n = p.reg_a;
                let d = 2i64.pow(combo(p, v) as u32);
                p.reg_c = (n / d) as i64;
            }
            _ => panic!("invalid instruction"),
        }
        p.ip += 1;
    }
    output
}

fn part1(src: &str) -> Result<String, Error> {
    let mut p = parse_input(src);
    let output = run(&mut p);
    let output = output
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    Ok(output)
}

fn search(p: &Program, a: i64, i: usize, ex: &[i64]) -> Option<i64> {
    if i == ex.len() {
        return None;
    }

    for x in 0..8 {
        let mut pc = p.clone();
        pc.reg_a = a + x;
        if run(&mut pc)[0] == ex[i] {
            if i < ex.len() - 1 {
                if let Some(n) = search(p, 8 * (a + x), i + 1, &ex) {
                    return Some(n);
                }
            } else {
                let mut pc = p.clone();
                pc.reg_a = a + x;
                let rx: Vec<i64> = ex.to_vec().into_iter().rev().collect();
                if run(&mut pc) == rx {
                    return Some(a + x);
                }
            }
        }
    }
    None
}

fn part2(src: &str) -> Result<i64, Error> {
    let p = parse_input(src);

    let mut expected = vec![2, 4, 1, 5, 7, 5, 1, 6, 4, 3, 5, 5, 0, 3, 3, 0];
    expected.reverse();

    if let Some(a) = search(&p, 1, 0, &expected) {
        Ok(a)
    } else {
        Err(anyhow!("Program not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(src).unwrap());

        println!("#################################\n\n");
        let src = "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!("0,3,5,4,3,0", part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(117440, part2(src).unwrap());
    }
}
