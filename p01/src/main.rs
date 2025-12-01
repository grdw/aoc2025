use std::fs;
use std::io;

type Rotations = Vec<(char, i16)>;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    let directions = input
        .trim()
        .split("\n")
        .map(|point| {
            let dir = point.chars().nth(0).unwrap();
            let steps = point[1..].parse::<i16>().unwrap();

            (dir, steps)
        })
        .collect::<Rotations>();

    println!("p1: {}", part1(&directions));
    println!("p2: {}", part2(&directions));

    Ok(())
}

fn part1(directions: &Rotations) -> u8 {
    let mut counter = 0;
    let mut start = 50;

    for (dir, rotations) in directions {
        match dir {
            'L' => start -= rotations,
            'R' => start += rotations,
            _   => panic!("invalid orientation")
        };

        start %= 100;

        if start == 0 {
            counter += 1;
        }
    }

    return counter
}

fn part2(directions: &Rotations) -> u8 {
    let mut counter = 0;
    let mut start = 50;

    for (dir, rotations) in directions {
        for _ in 0..*rotations {
            match dir {
                'L' => start -= 1,
                'R' => start += 1,
                _   => panic!("invalid orientation")
            };

            start %= 100;

            if start == 0 {
                counter += 1;
            }
        }
    }

    return counter
}

#[test]
fn test_parts() {
    let test_rotations = vec![
        ('L', 68),
        ('L', 30),
        ('R', 48),
        ('L', 5),
        ('R', 60),
        ('L', 55),
        ('L', 1),
        ('L', 99),
        ('R', 14),
        ('L', 82),
    ];
    assert_eq!(part1(&test_rotations), 3);
    assert_eq!(part2(&test_rotations), 6);
}
