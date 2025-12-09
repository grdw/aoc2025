use std::fs;
use std::io;
use std::collections::{HashSet, VecDeque};

type RedTiles = Vec<(isize, isize)>;

fn main() -> Result<(), io::Error> {
    let mut red_tiles = parse("input")?;
    red_tiles.sort();

    let p1 = part1(&red_tiles);

    println!("part1: {}", p1);
    //println!("part2: {}", p2);

    Ok(())
}

fn parse(file: &'static str) -> Result<RedTiles, io::Error> {
    let input = fs::read_to_string(file)?;

    Ok(
        input
            .trim()
            .split("\n")
            .map(|s| {
                let (sx, sy) = s.split_once(",").unwrap();
                (
                    sy.parse::<isize>().unwrap(),
                    sx.parse::<isize>().unwrap()
                )
            })
            .collect()
    )
}

fn part1(red_tiles: &RedTiles) -> isize {
    let mut max_area = 0;

    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let (ax, ay) = red_tiles[i];
            let (bx, by) = red_tiles[j];

            let width = (bx - ax).abs() + 1;
            let height = (by - ay).abs() + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

#[test]
fn test_part1() {
    assert_eq!(part1(&vec![
        (7,1),
        (11,1),
        (11,7),
        (9,7),
        (9,5),
        (2,5),
        (2,3),
        (7,3),
    ]), 50)
}
