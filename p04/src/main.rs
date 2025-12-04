use std::fs;
use std::io;

type PaperRolls = Vec<Vec<char>>;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    let paper_rolls = input
        .trim()
        .split("\n")
        .map(|row| row.chars().collect())
        .collect::<PaperRolls>();

    println!("p1: {}", part1(&paper_rolls));
    println!("p2: {}", part2(&paper_rolls));

    Ok(())
}

const MAX_SIZE: u16 = 4;
const GRID: [(isize, isize);8] = [
    (-1,-1),(-1,0),(-1,1),
    (0,-1),        (0,1),
    (1,-1), (1,0), (1,1),
];

fn part1(paper_rolls: &PaperRolls) -> usize {
    let row_len = paper_rolls[0].len();

    (0..paper_rolls.len()).map(|y| {
        (0..row_len).filter(|&x| grabbable(paper_rolls, x, y)).count()
    }).sum()
}

fn grabbable(paper_rolls: &PaperRolls, x: usize, y: usize) -> bool {
    if paper_rolls[y][x] == '.' {
        return false
    }

    let mut count = 0;
    let iy = y as isize;
    let ix = x as isize;

    for (gy, gx) in &GRID {
        // Note: this is pure evil math hackery. If you end up with an isize
        // of -1 f.e. and casting that to a usize, it will return the usize::MAX
        // which can't be found on the grid.
        let dy = (gy + iy) as usize;
        let dx = (gx + ix) as usize;

        if let Some(row) = paper_rolls.get(dy) && let Some(roll) = row.get(dx) {
            if *roll == '@' {
                count += 1;
            }
        }
    }

    return count < MAX_SIZE
}

fn part2(paper_rolls: &PaperRolls) -> u16 {
    let mut total = 0;
    let mut altered_rolls = paper_rolls.clone();

    loop {
        let removed = remove_rolls(&mut altered_rolls);

        if removed == 0 {
            break;
        }

        total += removed;
    }

    return total
}

fn remove_rolls(paper_rolls: &mut PaperRolls) -> u16 {
    let mut total = 0;
    let row_len = paper_rolls[0].len();

    for y in 0..paper_rolls.len() {
        for x in 0..row_len {
            if !grabbable(paper_rolls, x, y) {
                continue
            }

            paper_rolls[y][x] = '.';
            total += 1;
        }
    }

    return total
}
