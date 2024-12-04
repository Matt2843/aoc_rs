const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn inbounds(rows: usize, cols: usize, x: isize, y: isize) -> bool {
    (0..rows as isize).contains(&x) && (0..cols as isize).contains(&y)
}

fn part1(grid: &[Vec<char>]) -> usize {
    let xmas = ['X', 'M', 'A', 'S'];
    (0..grid.len())
        .map(|x| {
            (0..grid[x].len())
                .map(|y| {
                    DIRECTIONS
                        .iter()
                        .filter(|(dx, dy)| {
                            (0..xmas.len()).all(|i| {
                                let nx = x as isize + dx * i as isize;
                                let ny = y as isize + dy * i as isize;
                                inbounds(grid.len(), grid[0].len(), nx, ny)
                                    && grid[nx as usize][ny as usize] == xmas[i]
                            })
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}

fn xmas_cross(tlc: char, trc: char, blc: char, brc: char) -> bool {
    (tlc == 'M' && brc == 'S' && trc == 'M' && blc == 'S')
        || (tlc == 'M' && brc == 'S' && trc == 'S' && blc == 'M')
        || (tlc == 'S' && brc == 'M' && trc == 'M' && blc == 'S')
        || (tlc == 'S' && brc == 'M' && trc == 'S' && blc == 'M')
}

fn part2(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    (1..rows - 1)
        .flat_map(|x| {
            (1..cols - 1).filter(move |&y| match grid[x][y] {
                'A' => xmas_cross(
                    grid[x - 1][y - 1],
                    grid[x - 1][y + 1],
                    grid[x + 1][y - 1],
                    grid[x + 1][y + 1],
                ),
                _ => false,
            })
        })
        .count()
}

pub fn solve(inp: &str) -> (usize, usize) {
    let grid: Vec<Vec<char>> = inp.lines().map(|l| l.trim().chars().collect()).collect();
    (part1(&grid), part2(&grid))
}
