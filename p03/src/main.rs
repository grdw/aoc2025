use std::fs;
use std::io;

type BatteryBanks = Vec<Vec<u8>>;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    let battery_banks = input
        .trim()
        .split("\n")
        .map(|b|
            b
            .chars()
            .map(|n| n.to_digit(10).unwrap() as u8)
            .collect()
        )
        .collect::<BatteryBanks>();

    println!("p1: {}", part1(&battery_banks));
    println!("p2: {}", part2(&battery_banks));

    Ok(())
}

fn part1(battery_banks: &BatteryBanks) -> u16 {
    let mut total = 0;

    for bank in battery_banks {
        let mut i = 0;
        let mut j = 1;
        let mut powered_batteries = 0;

        let l = bank.len();

        while i < l {
            let c = bank[i];

            while j < l {
                let d = bank[j];
                j += 1;

                let n = format!("{}{}", c, d);
                let m = n.parse::<u16>().unwrap();
                if m > powered_batteries {
                    powered_batteries = m;
                }
            }

            i += 1;
            j = i + 1;
        }
        total += powered_batteries;
    }
    return total
}

fn part2(battery_banks: &BatteryBanks) -> u64 {
    let mut counter = 0;
    let powered_batteries: usize = 12;
    let size = battery_banks[0].len();

    for bank in battery_banks {
        let mut joltage = vec![];
        for _ in 0..powered_batteries {
        }
        counter += joltage;
    }
    return counter
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            &vec![
                vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1],
                vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9],
                vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8],
                vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]
            ]
        ),
        3121910778619
    );
}
