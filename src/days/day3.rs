use regex::Regex;

pub fn solve(inp: &str) -> (usize, usize) {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    let parse_mul = |caps: regex::Captures| {
        caps[1].parse::<usize>().unwrap() * caps[2].parse::<usize>().unwrap()
    };
    let part1: usize = mul_regex.captures_iter(inp).map(parse_mul).sum();
    let part2: usize = mul_regex
        .split(inp)
        .zip(mul_regex.captures_iter(inp).map(parse_mul))
        .scan(true, |incl, (hay, x)| {
            *incl = match hay {
                x if x.contains("do()") => true,
                x if x.contains("don't()") => false,
                _ => *incl
            };
            Some(if *incl { x } else { 0 })
        })
        .sum();
    (part1, part2)
}
