use std::collections::HashSet;
use itertools::Itertools;

fn guard_route(grid: &[Vec<char>], guard: (isize, isize), obstacle: Option<(isize,isize)>) -> Option<HashSet<(isize, isize)>> {
    let mut guard_pos = guard;
    let mut dirs = [(-1,0),(0,1),(1,0),(0,-1)].iter().cycle();
    let mut dir = dirs.next().unwrap();
    let mut visited = HashSet::new();
    loop {
        if visited.contains(&(guard_pos, dir)) {
            return None
        }
        visited.insert((guard_pos, dir));
        let nx = guard_pos.0 + dir.0;
        let ny = guard_pos.1 + dir.1;
        if !(0..grid.len() as isize).contains(&nx) || !(0..grid[0].len() as isize).contains(&ny) {
            break;
        }
        if let Some(o) = obstacle {
            if nx == o.0 && ny == o.1 {
                dir = dirs.next().unwrap();
                continue;
            }
        }
        if grid[nx as usize][ny as usize] == '#' {
            dir = dirs.next().unwrap();
            continue;
        }
        guard_pos = (nx, ny);

    }
    Some(visited.iter().map(|v| v.0).unique().collect())
}

pub fn solve(input: &str) -> (usize, usize) {
    let grid = input.trim()
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();

    let start = grid.iter()
        .enumerate()
        .find_map(|(i,p)| p.iter().position(|x| *x == '^').map(|j| (i as isize, j as isize)))
        .unwrap();

    let route = guard_route(&grid, start, None).unwrap();
    let part2 = route.iter()
        .filter(|p| guard_route(&grid, start, Some(**p)).is_none())
        .count();

    (route.len(),part2)
}
