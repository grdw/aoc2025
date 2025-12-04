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
    //println!("p2: {}", part2(&paper_rolls));

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
                // check if there are fewer than 4
            }
        }
    }
    return total
}

fn count_surrounding_rolls(paper_rolls: &PaperRolls, x: isize, y: isize) -> u16 {
    let mut count = 0;

    for (gy, gx) in &GRID {
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

//fn part2(paper_rolls: &PaperRolls) -> u64 {
//    let mut counter = 0;
//    let powered_batteries: usize = 12;
//
//    for bank in paper_rolls {
//        let mut joltage: u64 = 0;
//        let mut idx = 0;
//        let mut picks = 0;
//
//        loop {
//            let midx = cmp::min(idx+(bank.len()+1-powered_batteries), bank.len());
//            let mut max = 0;
//            for j in idx..midx {
//                if bank[j] > max {
//                    max = bank[j];
//                    idx = j;
//                }
//            }
//
//            joltage = joltage*10 + (bank[idx] as u64);
//            idx += 1;
//            picks += 1;
//
//            println!("{:?} {} {}", joltage, picks, midx);
//            if picks == powered_batteries {
//                break;
//            }
//
//            if idx == bank.len() {
//                break;
//            }
//        }
//        counter += joltage;
//    }
//    return counter
//}
//
//#[test]
//fn test_part2() {
//    assert_eq!(
//        part2(
//            &vec![
//                vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1],
//                vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9],
//                vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8],
//                vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]
//            ]
//        ),
//        3121910778619
//    );
//}
