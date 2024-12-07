use itertools::Itertools;

fn eval(ops: &[&char], vals: &[usize]) -> usize {
    ops.iter()
        .enumerate()
        .fold(vals[0], |sum, (i, &op)| match op {
            '+' => sum + vals[i + 1],
            '*' => sum * vals[i + 1],
            '|' => sum * 10usize.pow(vals[i + 1].ilog10() + 1) + vals[i + 1],
            _ => unreachable!(),
        })
}

fn calculate_sum(parsed: &[(usize, Vec<usize>)], operators: &[char]) -> usize {
    parsed.iter()
        .flat_map(|(lhs, vals)| {
            (0..vals.len() - 1)
                .map(|_| operators)
                .multi_cartesian_product()
                .find(|ops| *lhs == eval(ops, vals))
                .map(|_| lhs)
        })
        .sum()
}

pub fn solve(inp: &str) -> (usize, usize) {
    let operators = ['+', '*', '|'];
    let parsed = inp
        .trim()
        .lines()
        .flat_map(|l| l.trim().split_once(": "))
        .map(|(lhs, vals)| {
            (
                lhs.parse::<usize>().unwrap(),
                vals.split(' ')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();
    (
        calculate_sum(&parsed, &operators[0..2]),
        calculate_sum(&parsed, &operators),
    )
}
