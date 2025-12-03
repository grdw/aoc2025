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
    println!("p2: {}", part2(&battery_banks));

    Ok(())
}

fn part1(battery_banks: &BatteryBanks) -> u16 {
    let mut total = 0;

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

fn part2(battery_banks: &BatteryBanks) -> u64 {
    let mut counter = 0;
    let max_switches = 12;

    for bank in battery_banks {
        let mut switches = vec![];
        let mut count = 0;
        let l = bank.len();

        'parent: for i in ('1'..='9').rev() {
            for j in (0..l).rev() {
                if bank.chars().nth(j) == Some(i) {
                    switches.push(j);
                    count += 1;

                    if count == max_switches {
                        break 'parent;
                    }
                }
            }
        }

        let mut total = String::new();
        switches.sort();

        for i in 0..switches.len() {
            total.push(bank.chars().nth(switches[i]).unwrap());
        }
        println!("{:?}", total);
        counter += total.parse::<u64>().unwrap();
    }
    return counter
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            &vec![
                String::from("987654321111111"),
                String::from("811111111111119"),
                String::from("234234234234278"),
                String::from("818181911112111")
            ]
        ),
        3121910778619
    );
}
