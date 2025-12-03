use std::fs;
use std::io;
use std::cmp;

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
    println!("p2: {}", part2(&product_ids));

    Ok(())
}

fn part1(product_ids: &ProductIds) -> u64 {
    product_ids
        .iter()
        .map(|(start, end)| {
            (*start..=*end)
                .filter(|x| {
                    let digit_length = ((*x as f64).log10().floor() + 1.0) as u32;
                    let tens = 10_u64.pow(digit_length / 2);
                    let m = x % tens;
                    let d = x / tens;

                    m == d
                })
                .sum::<u64>()
        })
        .sum()
}

fn part2(product_ids: &ProductIds) -> u64 {
    let mut total = 0;

    for (start, end) in product_ids {
        for x in *start..=*end {
            let s = x.to_string();
            let i = s.len();

            for group_size in 1..=(i / 2) {
                let mut groups = vec![];
                let mut j = 0;

                while j < i {
                    let u = j;
                    let v = cmp::min(j+group_size, i);

                    groups.push(&s[u..v]);
                    j += group_size
                }

                if groups.iter().all(|&x| x == groups[0]) {
                    total += x;
                    break;
                }
            }
        }
    }

    total
}

