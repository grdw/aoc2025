use std::fs;
use std::io;

#[derive(Debug)]
enum CephalopodPart {
    Number(u64),
    Addition,
    Multiplication
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
                    "*" => CephalopodPart::Multiplication,
                    "+" => CephalopodPart::Addition,
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
    //println!("p2: {}", part2(&cephalod_problems));

    Ok(())
}

fn part1(cephalopod_problems: &CephalopodProblems) -> u64 {
    let mut total = 0;

    for problem in cephalopod_problems {
        let op = &problem[problem.len() - 1];

        let mut solution = match op {
            CephalopodPart::Multiplication => 1,
            CephalopodPart::Addition => 0,
            _ => panic!("Invalid operation")
        };

        for n in &problem[0..problem.len() - 1] {
            if let CephalopodPart::Number(x) = n {
                match op {
                    CephalopodPart::Multiplication => solution *= x,
                    CephalopodPart::Addition => solution += x,
                    _ => panic!("Invalid operation")
                };
            }
        }

        total += solution;
    }

    return total
}

//fn part2(cephalod_problems: &BatteryBanks) -> u64 {
//    let mut counter = 0;
//    let powered_batteries: usize = 12;
//
//    for bank in cephalod_problems {
//        let mut joltage: u64 = 0;
//        let mut remove = bank.len() - powered_batteries;
//        let mut stack = vec![];
//
//        for d in bank {
//            while remove > 0 && !stack.is_empty() && stack.last().unwrap() < &d {
//                stack.pop();
//                remove -= 1;
//            }
//            stack.push(d);
//        }
//
//        stack.truncate(powered_batteries);
//
//        for d in stack {
//            joltage = 10 * joltage + (*d as u64);
//        }
//
//        counter += joltage;
//    }
//    return counter
//}
//
//#[test]
//fn test_part2() {
//    assert_eq!(
//        part2(
//            &vec![
//                vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1],
//                vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9],
//                vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8],
//                vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]
//            ]
//        ),
//        3121910778619
//    );
//}
