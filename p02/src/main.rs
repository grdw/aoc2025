use std::fs;
use std::io;

type ProductIds = Vec<(u64, u64)>;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    let product_ids = input
        .trim()
        .split(",")
        .map(|range| {
            let (left, right) = range.split_once("-").unwrap();
            let start = left.parse::<u64>().unwrap();
            let end = right.parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<ProductIds>();

    println!("p1: {}", part1(&product_ids));
    //println!("p2: {}", part2(&product_ids));

    Ok(())
}

fn part1(product_ids: &ProductIds) -> u64 {
    let mut total = 0;

    for (start, end) in product_ids {
        for x in *start..=*end {
            let digit_length = ((x as f64).log10().floor() + 1.0) as u32;
            let tens = 10_u64.pow(digit_length / 2);
            let m = x % tens;
            let d = x / tens;

            if m == d {
                total += x;
            }
        }
    }

    total
}

//fn part2(product_ids: &ProductIds) -> u64 {
//    let mut counter = 0;
//    let mut start = 50;
//
//    for (dir, rotations) in product_ids {
//        for _ in 0..*rotations {
//            match dir {
//                'L' => start -= 1,
//                'R' => start += 1,
//                _   => panic!("invalid orientation")
//            };
//
//            start %= 100;
//
//            if start == 0 {
//                counter += 1;
//            }
//        }
//    }
//
//    return counter
//}
//
//#[test]
//fn test_parts() {
//    let test_rotations = vec![
//        ('L', 68),
//        ('L', 30),
//        ('R', 48),
//        ('L', 5),
//        ('R', 60),
//        ('L', 55),
//        ('L', 1),
//        ('L', 99),
//        ('R', 14),
//        ('L', 82),
//    ];
//    assert_eq!(part1(&test_rotations), 3);
//    assert_eq!(part2(&test_rotations), 6);
//}
