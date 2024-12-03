fn check_report(report: &[i64]) -> bool {
    if !report.is_sorted() && !report.iter().rev().is_sorted() {
        return false;
    }
    report
        .iter()
        .zip(report[1..].iter())
        .map(|(f, s)| (f - s).abs())
        .all(|d| (1..=3).contains(&d))
}

pub fn solve(inp: &str) -> (usize, usize) {
    let reports: Vec<Vec<i64>> = inp
        .lines()
        .map(|l| l.split(' ').flat_map(|n| n.parse()).collect())
        .collect();

    let part1 = reports.iter().filter(|r| check_report(r)).count();

    let part2 = reports
        .iter()
        .filter(|r| match check_report(r) {
            true => true,
            false => (0..r.len()).any(|i| {
                let mut cr = (*r).clone();
                cr.remove(i);
                check_report(&cr)
            }),
        })
        .count();
    (part1, part2)
}
