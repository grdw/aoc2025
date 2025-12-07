use std::fs;
use std::io;
use std::collections::{HashSet, VecDeque};

type Grid = Vec<Vec<char>>;

fn main() -> Result<(), io::Error> {
    let grid = parse("input")?;

    println!("part1: {}", part1(&grid));

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

fn part1(grid: &Grid) -> usize {
    let mut count = 0;
    let mut drawy = VecDeque::new();
    let mut set = HashSet::new();

    let sx = grid[0]
        .iter()
        .position(|&x| x == 'S')
        .unwrap();

    drawy.push_front((1, sx));

    while let Some((y, x)) = drawy.pop_front() {
        if set.contains(&(y, x)) || y + 1 >= grid.len() {
            continue
        }

        match grid[y + 1][x] {
            '^' => {
                count += 1;
                drawy.push_back((y + 1, x - 1));
                drawy.push_back((y + 1, x + 1));
            },
            '.' => {
                drawy.push_back((y + 1, x));
                drawy.push_back((y + 1, x));
            },
            _ => panic!("Invalid character")
        }

        set.insert((y, x));
    }

    return count
}

#[test]
fn test_part1() {
    let grid = parse("example_input").unwrap();

    assert_eq!(part1(&grid), 21);

}
