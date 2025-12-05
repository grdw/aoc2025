use std::fs;
use std::io;
use std::collections::HashSet;

type IngredientRanges = Vec<(u64,u64)>;
type Ingredients = Vec<u64>;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input")?;
    let (raw_ranges, raw_ingredients) = input.split_once("\n\n").unwrap();
    let mut ranges = raw_ranges
        .trim()
        .split("\n")
        .map(|row| {
            let (low_s, high_s) = row.split_once("-").unwrap();
            let low = low_s.parse::<u64>().unwrap();
            let high = high_s.parse::<u64>().unwrap();
            (low, high)
        })
        .collect::<IngredientRanges>();

    ranges.sort();

    let ingredients = raw_ingredients
        .trim()
        .split("\n")
        .map(|row| row.parse::<u64>().unwrap())
        .collect::<Ingredients>();

    println!("p1: {}", part1(&ranges, &ingredients));
    println!("p2: {}", part2(&ranges));

    Ok(())
}

fn part1(ranges: &IngredientRanges, ingredients: &Ingredients) -> usize {
    ingredients
        .iter()
        .filter(|&id|
            ranges
                .iter()
                .any(|(start, end)| id >= start && id <= end)
        )
        .count()

    //return count
}

fn part2(ranges: &IngredientRanges) -> u64 {
    let mut total = 0;
    let mut set = HashSet::new();

    'parent: for (start, end) in ranges {
        let mut dstart = *start;

        for (xstart, xend) in ranges {
            if xstart == start && xend == end {
                break
            }

            // fully enveloped means we can just skip counting
            if start >= xstart && end <= xend {
                continue 'parent;
            } else if start >= xstart && start <= xend {
                dstart = xend + 1;
            }
        }

        set.insert((dstart, end));
    }

    for (start, end) in set {
        total += (end - start) + 1;
    }

    return total
}

#[test]
fn test_part2_example() {
    let mut ranges = vec![
        (3, 5),
        (10, 14),
        (16, 20),
        (12, 18)
    ];
    ranges.sort();
    assert_eq!(part2(&ranges), 14);
}

#[test]
fn test_part2_poor_start_rewrite() {
    let mut ranges = vec![
        (1, 10),
        (1, 12),
        (3, 14),
    ];
    ranges.sort();
    assert_eq!(part2(&ranges), 14); // 3,4,5,6
}

#[test]
fn test_part2_ignore_once() {
    let mut ranges = vec![
        (3, 5),
        (5, 6),
    ];
    ranges.sort();
    assert_eq!(part2(&ranges), 4); // 3,4,5,6
}

#[test]
fn test_part2_more_edge_case() {
    let mut ranges = vec![
        (3, 5),
        (3, 6),
        (3, 7),
        (3, 8),
    ];
    ranges.sort();
    assert_eq!(part2(&ranges), 6); // 3,4,5,6,7,8
}

#[test]
fn test_part2_triple_overlap() {
    let mut ranges = vec![
        (3, 10),
        (4, 11),
        (5, 12),
        (6, 13),
    ];
    ranges.sort();
    assert_eq!(part2(&ranges), 11); // 3,4,5,6,7,8,9,10,11,12,13
}

#[test]
fn test_part2_really_close() {
    let mut ranges = vec![
        (1, 2),
        (2, 3),
        (3, 4),
        (5, 6),
    ];
    ranges.sort();
    assert_eq!(part2(&ranges), 6); // 1,2,3,4,5,6
}

#[test]
fn test_part2_wrappers_all_the_way_down() {
    let mut ranges = vec![
        (1, 10),
        (2, 9),
        (3, 8),
        (4, 7),
    ];
    ranges.sort();
    assert_eq!(part2(&ranges), 10); // 1,2,3,4,5,6,7,8,9,10
}

#[test]
fn test_part2_wrappers_with_overlap() {
    let mut ranges = vec![
        (2, 10),
        (11, 15),
        (1, 16),
    ];
    ranges.sort();
    assert_eq!(part2(&ranges), 16);
}

#[test]
fn test_part2_fully_wrapped() {
    let mut ranges = vec![
        (3, 5),
        (5, 6),
        (3, 7),
    ];
    ranges.sort();
    assert_eq!(part2(&ranges), 5);
}
