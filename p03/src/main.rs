use std::fs;
use std::io;

type BatteryBanks = Vec<String>;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    let battery_banks = input
        .trim()
        .split("\n")
        .map(|b| b.to_string())
        .collect::<BatteryBanks>();

    println!("p1: {}", part1(&battery_banks));
    //println!("p2: {}", part2(&battery_banks));

    Ok(())
}

fn part1(battery_banks: &BatteryBanks) -> u16 {
    let mut total = 0;
    let range = ['9', '8', '7', '6', '5', '4', '3', '2', '1'];
    for bank in battery_banks {
        let mut i = 0;
        let mut j = 1;
        let mut max = 0;
        let l = bank.len();

        while i < l {
            let c = bank.chars().nth(i).unwrap();

            while j < l {
                let d = bank.chars().nth(j).unwrap();
                j += 1;

                let n = format!("{}{}", c, d);
                let m = n.parse::<u16>().unwrap();
                if m > max {
                    max = m;
                }
            }

            i += 1;
            j = i + 1;
        }
        total += max;
    }
    return total
}

//fn part2(battery_banks: &BatteryBanks) -> u16 {
//    let mut counter = 0;
//    let mut start = 50;
//
//    for (dir, rotations) in battery_banks {
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
