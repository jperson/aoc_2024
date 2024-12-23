use anyhow::Error;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    let input = include_str!("../../input/day23/input.txt");

    println!("part 1:   {}", part1(input).unwrap());
    println!("part 2_1: {}", part2_1(input).unwrap());
    println!("part 2_2: {}", part2_2(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> Vec<(&str, &str)> {
    src.lines()
        .map(|l| {
            let (a, b) = l.split_once("-").unwrap();
            (a, b)
        })
        .collect()
}

fn connected<'a>(graph: &'a FxHashMap<&'a str, FxHashSet<&'a str>>) -> FxHashSet<Vec<&'a str>> {
    let mut groups: FxHashSet<Vec<&'a str>> = FxHashSet::default();

    for k in graph.keys() {
        if let Some(neighbors) = graph.get(k) {
            for n in neighbors {
                if let Some(n2) = graph.get(n) {
                    for ni in neighbors.intersection(n2) {
                        if k.starts_with("t") || n.starts_with("t") || ni.starts_with("t") {
                            let mut key = vec![*k, *n, *ni];
                            key.sort();
                            groups.insert(key);
                        }
                    }
                }
            }
        }
    }
    groups
}

fn part1(src: &str) -> Result<i64, Error> {
    let input = parse_input(src);
    let mut network: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();

    for (a, b) in &input {
        network.entry(a).or_default().insert(b);
        network.entry(b).or_default().insert(a);
    }
    let groups = connected(&network);

    Ok(groups.len() as i64)
}

fn max_clique<'a>(graph: &'a FxHashMap<&'a str, FxHashSet<&'a str>>) -> Vec<&'a &'a str> {
    let mut ggs: FxHashMap<Vec<&'a &'a str>, i64> = FxHashMap::default();

    for c in graph.keys() {
        if let Some(ns) = graph.get(c) {
            let fs = ns
                .iter()
                .map(|n| {
                    let nn = graph.get(n).unwrap();
                    let mut vs = ns.intersection(nn).collect::<Vec<_>>();
                    vs.sort();
                    vs
                })
                .collect::<Vec<_>>();

            for f in fs.iter() {
                *ggs.entry(f.to_owned()).or_default() += 1;
            }
        }
    }
    let (rs, _) = ggs.into_iter().max_by_key(|e| e.1).unwrap();
    rs
}

fn bron_kerbosch<'a>(
    r: FxHashSet<&'a str>,
    mut p: FxHashSet<&'a str>,
    mut x: FxHashSet<&'a str>,
    graph: &mut FxHashMap<&'a str, FxHashSet<&'a str>>,
    cliques: &mut Vec<FxHashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    for v in p.clone() {
        bron_kerbosch(
            &r | &FxHashSet::<&'a str>::from_iter([v]),
            &p & graph.entry(v).or_default(),
            &x & graph.entry(v).or_default(),
            graph,
            cliques,
        );
        p.remove(v);
        x.insert(v);
    }
}

fn part2_1(src: &str) -> Result<String, Error> {
    let input = parse_input(src);
    let mut network: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();

    for (a, b) in &input {
        network.entry(a).or_default().insert(b);
        network.entry(b).or_default().insert(a);
        network.entry(a).or_default().insert(a);
        network.entry(b).or_default().insert(b);
    }
    let lan = max_clique(&network);
    let s = lan
        .iter()
        .map(|s| s.chars().collect::<String>())
        .collect::<Vec<String>>()
        .join(",");
    Ok(s)
}

fn part2_2(src: &str) -> Result<String, Error> {
    let input = parse_input(src);
    let mut network: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();

    for (a, b) in &input {
        network.entry(a).or_default().insert(b);
        network.entry(b).or_default().insert(a);
    }

    let mut cliques: Vec<FxHashSet<&str>> = Vec::new();
    bron_kerbosch(
        FxHashSet::default(),
        FxHashSet::from_iter(network.clone().into_keys()),
        FxHashSet::default(),
        &mut network,
        &mut cliques,
    );

    cliques.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut max_clique = cliques[0].iter().collect::<Vec<_>>();
    max_clique.sort();

    let s = max_clique
        .iter()
        .map(|s| s.chars().collect::<String>())
        .collect::<Vec<String>>()
        .join(",");

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let src = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!(7, part1(src).unwrap());
        assert_eq!("co,de,ka,ta".to_string(), part2_1(src).unwrap());
        assert_eq!("co,de,ka,ta".to_string(), part2_2(src).unwrap());
    }
}

//2472 too high
