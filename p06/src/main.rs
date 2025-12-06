use std::fs;
use std::io;

#[derive(Debug)]
enum CephalopodPart {
    Number(u64),
    Add,
    Mul
}

type CephalopodProblems = Vec<Vec<CephalopodPart>>;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    let mut cephalopod_problems: CephalopodProblems = vec![];

    input
        .trim()
        .split("\n")
        .for_each(|row|
            for (i, n) in row.split_whitespace().enumerate() {
                let part = match n {
                    "*" => CephalopodPart::Mul,
                    "+" => CephalopodPart::Add,
                    _   => CephalopodPart::Number(
                        n.parse::<u64>().unwrap()
                    )
                };

                match cephalopod_problems.get_mut(i) {
                    Some(vec) => vec.push(part),
                    None => cephalopod_problems.push(vec![part])
                }
            }
        );

    println!("p1: {}", part1(&cephalopod_problems));
    println!("p2: {}", part2(&input));

    Ok(())
}

fn part1(cephalopod_problems: &CephalopodProblems) -> u64 {
    let mut total = 0;

    for problem in cephalopod_problems {
        let op = &problem[problem.len() - 1];

        let mut solution = match op {
            CephalopodPart::Mul => 1,
            CephalopodPart::Add => 0,
            _ => panic!("Invalid operation")
        };

        for n in &problem[0..problem.len() - 1] {
            if let CephalopodPart::Number(x) = n {
                match op {
                    CephalopodPart::Mul => solution *= x,
                    CephalopodPart::Add => solution += x,
                    _ => panic!("Invalid operation")
                };
            }
        }

        total += solution;
    }

    return total
}

fn part2(input: &String) -> u64 {
    let mut rows: Vec<&str> = input
        .split("\n")
        .collect();

    let _ = rows.pop();

    let mut counter = 0;
    let mut problems: Vec<Vec<&str>> = vec![];
    let mut spaces: Vec<usize> = vec![];
    let mut space_count = 0;

    // Parsing
    for (i, c) in rows.last().unwrap().chars().enumerate() {
        match c {
            '*' | '+' => spaces.push(i),
            ' '       => continue,
            _         => panic!("Invalid character {}", c)
        }
    }

    spaces.push(rows[0].len());

    for (j, row) in rows.iter().enumerate() {
        for i in 0..spaces.len()-1 {
            let blurb = &row[spaces[i]..spaces[i + 1]];

            match problems.get_mut(i) {
                Some(vec) => vec.push(blurb),
                None      => problems.push(vec![blurb])
            }
        }
    }

    for p in problems {
        let mut numbers: Vec<u64> = vec![0; p.len()];

        for y in 0..p.len()-1 {
            for x in 0..p[y].len() {
                let d = p[y].chars().nth(x).unwrap();

                if let Some(e) = d.to_digit(10) {
                    numbers[x] = numbers[x] * 10 + e as u64;
                }
            }
        };

        let op = p.last().unwrap().trim();
        numbers.retain(|&x| x > 0);

        let answer = match op {
            "+" => numbers.iter().sum::<u64>(),
            "*" => numbers.iter().product::<u64>(),
            _   => panic!("Invalid operand")
        };

        counter += answer;
    }

    return counter
}
