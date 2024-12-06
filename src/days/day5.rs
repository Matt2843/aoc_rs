use std::collections::HashMap;
use itertools::Itertools;

fn check_page(rules: &HashMap<usize, Vec<usize>>, page: &[usize]) -> bool {
    page.iter()
        .enumerate()
        .all(|(i, n)|{
            let rule = rules.get(n).unwrap();
            if let Some(idx) = page.iter().position(|p| rule.contains(p)) {
                idx >= i
            } else {
                true
            }
        })
}

pub fn solve(input: &str) -> (usize, usize) {
    let (rules, pages) = input.trim().split_once("\n\n").unwrap();
    let rules = rules.lines()
        .map(|l| l.trim())
        .fold(HashMap::<usize, Vec<usize>>::new(), |mut acc, l| {
            let (lhs, rhs) = l.trim().split_once('|').unwrap();
            let lhs = lhs.parse().unwrap();
            let rhs = rhs.parse().unwrap();
            acc.entry(lhs).or_default().push(rhs);
            acc.entry(rhs).or_default(); // so we can unwrap any n
            acc
        });

    let (good, mut bad): (Vec<_>, Vec<_>) = pages.lines()
        .map(|l| l.trim().split(',').map(|s| s.parse::<usize>().unwrap()).collect_vec())
        .partition(|page| check_page(&rules, page));
    let part1 = good.iter().map(|p| p[p.len()/2]).sum();
    let part2 = bad.iter_mut()
        .map(|v| {v.sort_by(|a,b|
            match rules.get(a).unwrap().contains(b) {
                true => std::cmp::Ordering::Less,
                false => std::cmp::Ordering::Greater
            }
        ); v[v.len()/2]})
        .sum();


    (part1, part2)
}
