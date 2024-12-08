use itertools::Itertools;
use std::collections::HashMap;

fn inbounds(a: (isize, isize), dim: isize) -> bool {
    (0..dim).contains(&a.0) && (0..dim).contains(&a.1)
}

fn antinodes(
    mut a1: (isize, isize),
    mut a2: (isize, isize),
    dim: isize,
    repeat: bool,
) -> Vec<(isize, isize)> {
    let mut nodes = vec![];
    let diff = (a1.0 - a2.0, a1.1 - a2.1);
    loop {
        let mut updated = false;
        a1 = (a1.0 + diff.0, a1.1 + diff.1);
        a2 = (a2.0 - diff.0, a2.1 - diff.1);
        if inbounds(a1, dim) {
            updated = true;
            nodes.push(a1);
        }
        if inbounds(a2, dim) {
            updated = true;
            nodes.push(a2);
        }
        if !repeat || !updated {
            break nodes;
        }
    }
}

pub fn solve(inp: &str) -> (usize, usize) {
    let dim = inp.trim().lines().count() as isize;
    let antennas = inp
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.trim()
                .chars()
                .enumerate()
                .flat_map(move |(c, ch)| match ch {
                    '.' | '#' => None,
                    _ => Some((ch, r, c)),
                })
        })
        .fold(
            HashMap::<char, Vec<(isize, isize)>>::new(),
            |mut acc, (ch, r, c)| {
                acc.entry(ch)
                    .or_default()
                    .push((r.try_into().unwrap(), c.try_into().unwrap()));
                acc
            },
        );

    let part1 = antennas
        .iter()
        .flat_map(|(_, vs)| {
            vs.iter()
                .combinations_with_replacement(2)
                .filter(|v| v[0] != v[1])
                .flat_map(|v| antinodes(*v[0], *v[1], dim, false))
        })
        .unique()
        .count();

    let part2 = antennas
        .iter()
        .flat_map(|(_, vs)| {
            vs.iter()
                .combinations_with_replacement(2)
                .filter(|v| v[0] != v[1])
                .flat_map(|v| antinodes(*v[0], *v[1], dim, true))
        })
        .chain(
            antennas
                .iter()
                .filter(|a| a.1.len() > 1)
                .flat_map(|a| a.1.clone()),
        )
        .unique()
        .count();

    (part1, part2)
}
