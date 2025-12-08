use std::fs;
use std::io;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<char>>;

fn main() -> Result<(), io::Error> {
    let grid = parse("input")?;
    let (p1, p2) = resolve(&grid);

    println!("part1: {}", p1);
    println!("part2: {}", p2);

    Ok(())
}

fn parse(file: &'static str) -> Result<Grid, io::Error> {
    let input = fs::read_to_string(file)?;

    Ok(
        input
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect()
    )
}

fn resolve(grid: &Grid) -> (usize, usize) {
    let mut count = 0;
    let mut queue = VecDeque::new();
    let mut set = HashSet::new();
    let mut count_grid = vec![
        vec![0; grid[0].len()];
        grid.len()
    ];

    let sx = grid[0]
        .iter()
        .position(|&x| x == 'S')
        .unwrap();

	queue.push_front((1, sx));
    count_grid[1][sx] = 1;

    while let Some((y, x)) = queue.pop_front() {
        if set.contains(&(y, x)) || y + 1 >= grid.len() {
            continue
        }

        match grid[y + 1][x] {
            '^' => {
                count += 1;
                count_grid[y + 1][x - 1] += count_grid[y][x];
                count_grid[y + 1][x + 1] += count_grid[y][x];

                queue.push_back((y + 1, x - 1));
                queue.push_back((y + 1, x + 1));
            },
            '.' => {
                count_grid[y + 1][x] += count_grid[y][x];

                queue.push_back((y + 1, x));
            },
            _ => panic!("Invalid character")
        }

        set.insert((y, x));
    }

    return (count, count_grid[count_grid.len() - 1].iter().sum())
}

#[test]
fn test_resolve() {
    let grid = parse("example_input").unwrap();

    assert_eq!(resolve(&grid), (21, 40));

}
