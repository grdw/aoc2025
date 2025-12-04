use std::fs;
use std::io;

type PaperRolls = Vec<Vec<char>>;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    let paper_rolls = input
        .trim()
        .split("\n")
        .map(|b|
            b
            .chars()
            .collect()
        )
        .collect::<PaperRolls>();

    println!("p1: {}", part1(&paper_rolls));
    println!("p2: {}", part2(&paper_rolls));

    Ok(())
}

const GRID: [(isize, isize);8] = [
    (-1,-1),(-1,0),(-1,1),
    (0,-1),        (0,1),
    (1,-1), (1,0), (1,1),
];

fn part1(paper_rolls: &PaperRolls) -> u16 {
    let mut total = 0;

    for y in 0..paper_rolls.len() {
        let row = &paper_rolls[y];

        for x in 0..row.len() {
            if row[x] == '@' {
                let c = count_surrounding_rolls(
                    paper_rolls,
                    x as isize,
                    y as isize
                );
                if c < 4 {
                    total += 1;
                }
            }
        }
    }
    return total
}

fn count_surrounding_rolls(paper_rolls: &PaperRolls, x: isize, y: isize) -> u16 {
    let mut count = 0;

    for (gy, gx) in &GRID {
        // Note: this is pure evil math hackery. If you end up with an isize
        // of -1 f.e. and casting that ot a usize, it will return the usize::MAX
        // which can't be found on the grid.
        let dy = (gy + y) as usize;
        let dx = (gx + x) as usize;

        if let Some(row) = paper_rolls.get(dy) && let Some(roll) = row.get(dx) {
            if *roll == '@' {
                count += 1;
            }
        }
    }

    return count
}

fn part2(paper_rolls: &PaperRolls) -> u64 {
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

fn remove_rolls(paper_rolls: &mut PaperRolls) -> u64 {
    let mut total = 0;
    let row_len = paper_rolls[0].len();

    for y in 0..paper_rolls.len() {
        for x in 0..row_len {
            if paper_rolls[y][x] == '@' {
                let c = count_surrounding_rolls(
                    paper_rolls,
                    x as isize,
                    y as isize
                );
                if c < 4 {
                    paper_rolls[y][x] = '.';
                    total += 1;
                }
            }
        }
    }
    return total
}
