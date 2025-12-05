use std::fs;
use std::io;

type IngredientRanges = Vec<(u64,u64)>;
type Ingredients = Vec<u64>;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    let (raw_ranges, raw_ingredients) = input.split_once("\n\n").unwrap();
    let ranges = raw_ranges
        .trim()
        .split("\n")
        .map(|row| {
            let (low_s, high_s) = row.split_once("-").unwrap();
            let low = low_s.parse::<u64>().unwrap();
            let high = high_s.parse::<u64>().unwrap();
            (low, high)
        })
        .collect::<IngredientRanges>();

    let ingredients = raw_ingredients
        .trim()
        .split("\n")
        .map(|row| row.parse::<u64>().unwrap())
        .collect::<Ingredients>();

    println!("p1: {}", part1(&ranges, &ingredients));
    //println!("p2: {}", part2(&paper_rolls));

    Ok(())
}

fn part1(ranges: &IngredientRanges, ingredients: &Ingredients) -> usize {
    let mut count = 0;
    for id in ingredients {
        for (start, end) in ranges {
            if id >= start && id <= end {
                count += 1;
                break;
            }
        }
    }
    return count
}
