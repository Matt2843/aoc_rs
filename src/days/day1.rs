use std::collections::HashMap;

pub fn solve(input: &str) -> (usize, usize) {
    let (mut lv, mut rv) = input
        .lines()
        .flat_map(|l| l.split_once("   "))
        .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
        .collect::<(Vec<_>, Vec<_>)>();

    lv.sort();
    rv.sort();

    let part1 = lv
        .iter()
        .zip(rv.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i64>() as usize;

    let counter = rv.iter().fold(HashMap::new(), |mut acc, x| {
        acc.entry(x).and_modify(|e| *e += 1).or_insert(1);
        acc
    });

    let part2 = lv
        .iter()
        .map(|l| counter.get(l).unwrap_or(&0) * l)
        .sum::<i64>() as usize;

    (part1, part2)
}
