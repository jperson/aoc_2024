use anyhow::Error;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    let input = include_str!("../../input/day24/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    AND,
    XOR,
    OR,
    UNKNOWN,
}

type Gate<'a> = (&'a str, Op, &'a str);

fn parse_input<'a>(src: &'a str) -> (FxHashMap<&str, i64>, FxHashMap<&str, Gate>) {
    let (a, b) = src.split_once("\n\n").unwrap();

    let inputs: FxHashMap<&str, i64> = FxHashMap::from_iter(a.lines().map(|l| {
        let (a, b) = l.split_once(": ").unwrap();
        (a, b.parse::<i64>().unwrap())
    }));

    let mut ops: FxHashMap<&str, (&str, Op, &str)> = FxHashMap::default();
    for l in b.lines() {
        let (a, b) = l.split_once(" -> ").unwrap();
        let vs = a.split(" ").collect::<Vec<&str>>();
        if let [x, op, y, ..] = vs[..] {
            let op = match op {
                "AND" => Op::AND,
                "XOR" => Op::XOR,
                "OR" => Op::OR,
                _ => Op::UNKNOWN,
            };
            ops.insert(b, (x, op, y));
        }
    }
    (inputs, ops)
}

fn topo_visit<'a>(
    n: &'a str,
    graph: &FxHashMap<&'a str, Vec<&'a str>>,
    visited: &mut FxHashSet<&'a str>,
    vs: &mut Vec<&'a str>,
) {
    visited.insert(n);

    if let Some(adj) = graph.get(n) {
        for a in adj {
            if !visited.contains(a) {
                topo_visit(a, graph, visited, vs);
            }
        }
    }
    vs.push(n);
}

fn topo_sort<'a>(graph: &FxHashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    let mut visited: FxHashSet<&str> = FxHashSet::default();
    let mut vs: Vec<&'a str> = Vec::new();

    for n in graph.keys() {
        if !visited.contains(n) {
            topo_visit(n, &graph, &mut visited, &mut vs);
        }
    }
    vs
}

fn part1(src: &str) -> Result<i64, Error> {
    let (mut outputs, ops) = parse_input(src);
    let mut graph: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
    let mut topo_vs: FxHashMap<&str, (&str, &Op, &str)> = FxHashMap::default();

    for (z, (x, op, y)) in ops.iter() {
        graph.entry(z).or_default().push(x);
        graph.entry(z).or_default().push(y);
        topo_vs.insert(z, (x, op, y));
    }

    let vord = topo_sort(&graph);

    for v in vord {
        if !outputs.contains_key(v) {
            let (x, op, y) = topo_vs.get(v).unwrap();
            let x = outputs.get(x).unwrap();
            let y = outputs.get(y).unwrap();

            *outputs.entry(v).or_default() = match op {
                Op::AND => x & y,
                Op::XOR => x ^ y,
                Op::OR => x | y,
                _ => 0,
            }
        }
    }

    let mut vs = outputs
        .iter()
        .filter(|p| p.0.starts_with("z"))
        .collect::<Vec<_>>();
    vs.sort_by(|a, b| a.0.cmp(&b.0));

    let mut total: i64 = 0;
    for (i, (_, v)) in vs.iter().enumerate() {
        total += *v << i;
    }

    Ok(total)
}

fn part2(src: &str) -> Result<String, Error> {
    let (_, ops) = parse_input(src);

    let mut faulty: FxHashSet<&str> = FxHashSet::default();
    for (out, (x, op, y)) in ops.clone() {
        if out.starts_with("z") && op != Op::XOR && out != "z45" {
            faulty.insert(out);
            continue;
        }
        if !(x.starts_with("x") || x.starts_with("y"))
            && !(y.starts_with("x") || y.starts_with("y"))
            && !out.starts_with("z")
            && op == Op::XOR
        {
            faulty.insert(out);
            continue;
        }
    }

    for (out, (x, op, y)) in ops.clone() {
        if op == Op::XOR
            && !out.starts_with("z")
            && (x != "x00" || x != "y00")
            && (y != "y00" || y != "x00")
        {
            let mut found: bool = false;
            for (_, (xx, op2, yy)) in ops.clone().into_iter() {
                if op2 == Op::XOR
                    && (xx == out || yy == out)
                    && (xx != "x00" || xx != "y00")
                    && (yy != "y00" || yy != "x00")
                {
                    found = true;
                    break;
                }
            }
            if !found {
                faulty.insert(out);
            }
        }
        if op == Op::AND && !((x == "x00" || x == "y00") || (y == "y00" || y == "x00")) {
            let mut found: bool = false;
            for (_, (xx, op2, yy)) in ops.clone().into_iter() {
                if op2 == Op::OR
                    && ((xx == out || yy == out)
                        || (xx == "x00" || xx == "y00")
                        || (yy == "y00" || yy == "x00"))
                {
                    found = true;
                    break;
                }
            }
            if !found {
                faulty.insert(out);
            }
        }
    }
    let mut faulty: Vec<&str> = faulty.into_iter().collect::<Vec<_>>();
    faulty.sort();
    Ok(faulty
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

        assert_eq!(4, part1(src).unwrap());
    }

    #[test]
    fn test_part_x() {
        let src = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

        assert_eq!(2024, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

        assert_eq!(0, part2(src).unwrap());
    }
}
