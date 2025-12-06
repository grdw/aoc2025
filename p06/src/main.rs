use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    println!("p1: {}", part1(&input));
    println!("p2: {}", part2(&input));

    Ok(())
}

fn part1(input: &String) -> u64 {
    let mut problems: Vec<Vec<&str>> = vec![];

    input
        .trim()
        .split("\n")
        .for_each(|row|
            for (i, n) in row.split_whitespace().enumerate() {
                match problems.get_mut(i) {
                    Some(vec) => vec.push(n),
                    None      => problems.push(vec![n])
                }
            }
        );


    problems
        .iter()
        .map(|problem| {
            let end = problem.len() - 1;
            let op = problem[end];

            match op {
                "*" => problem[0..end]
                    .iter()
                    .map(|s| s.parse::<u64>().unwrap())
                    .product::<u64>(),

                "+" => problem[0..end]
                    .iter()
                    .map(|s| s.parse::<u64>().unwrap())
                    .sum::<u64>(),

                _ => panic!("Invalid operation")
            }
        })
        .sum()
}

fn part2(input: &String) -> u64 {
    let rows: Vec<&str> = input
        .split_terminator("\n")
        .collect();

    let mut problems: Vec<Vec<&str>> = vec![];
    let mut spaces: Vec<usize> = vec![];

    // This just counts offsets from one +/* to the next +/*
    for (i, c) in rows.last().unwrap().chars().enumerate() {
        match c {
            '*' | '+' => spaces.push(i),
            ' '       => continue,
            _         => panic!("Invalid character {}", c)
        }
    }

    // ... and finally adds the full length in the end
    spaces.push(rows[0].len());

    // This makes sure all the correct string chunks are
    // added to the correct vectors
    for row in rows.iter() {
        for i in 0..spaces.len()-1 {
            let blurb = &row[spaces[i]..spaces[i + 1]];

            match problems.get_mut(i) {
                Some(vec) => vec.push(blurb),
                None      => problems.push(vec![blurb])
            }
        }
    }

    problems
        .iter()
        .map(|problem| {
            let mut numbers: Vec<u64> = vec![0; problem.len()];
            let op = problem.last().unwrap().trim();

            for y in 0..problem.len()-1 {
                for x in 0..problem[y].len() {
                    let d = problem[y].chars().nth(x).unwrap();

                    if let Some(e) = d.to_digit(10) {
                        numbers[x] = numbers[x] * 10 + e as u64;
                    }
                }
            };

            numbers.retain(|&x| x > 0);

            match op {
                "+" => numbers.iter().sum::<u64>(),
                "*" => numbers.iter().product::<u64>(),
                _   => panic!("Invalid operand")
            }
        })
        .sum()
}
