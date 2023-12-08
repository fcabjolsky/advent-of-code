use std::collections::BTreeMap;

use gcd::Gcd;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parse_input<'a>(input: &'a String) -> (Vec<&'a str>, BTreeMap<&'a str, (&'a str, &'a str)>) {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .expect("Wrong input format")
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    lines.next();
    let mapping = lines.fold(BTreeMap::<&str, (&str, &str)>::new(), |mut acc, line| {
        let (from, mut next) = line.split_once(" = ").expect("Wrong input format");
        next = next.trim_matches(|c| c == '(' || c == ')');
        let next = next.split_once(", ").expect("Wrong input format");
        acc.insert(from, next);
        acc
    });
    (directions, mapping)
}

fn day8_part1(input: String) -> usize {
    let (directions, mapping) = parse_input(&input);
    let mut steps = 0;
    let mut next = "AAA";
    loop {
        for direction in &directions {
            steps += 1;
            let current = mapping.get(next).expect("To have the next in the mapping");
            next = if *direction == "R" {
                current.1
            } else {
                current.0
            };
            if next == "ZZZ" {
                return steps;
            }
        }
    }
}

fn day8_part2(input: String) -> usize {
    let (directions, mapping) = parse_input(&input);
    let nexts = mapping
        .keys()
        .filter(|k| k.ends_with("A"))
        .cloned()
        .collect::<Vec<&str>>();
    let mut cycles = nexts
        .into_par_iter()
        .map(|start| {
            let mut next = start;
            let mut steps = 0;
            loop {
                for direction in &directions {
                    steps += 1;
                    let current = mapping.get(next).expect("To have the next in the mapping");
                    next = if *direction == "R" {
                        current.1
                    } else {
                        current.0
                    };
                    if next.ends_with("Z") {
                        return steps;
                    }
                }
            }
        })
        .collect::<Vec<usize>>();

    let lcm = cycles.pop().expect("To have a cycle");
    cycles
        .into_iter()
        .fold(lcm, |acc, cycle| (acc * (cycle)) / acc.gcd(cycle))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day8_part1_small_test() {
        let test = String::from(
            r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(day8_part1(test), 2);
    }

    #[test]
    fn day8_part1_small_test_2() {
        let test = String::from(
            r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(day8_part1(test), 6);
    }

    #[test]
    fn day8_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day8/input.txt")?;

        let r = day8_part1(input);
        println!("{}", r);
        assert_eq!(r, 14893);
        Ok(())
    }
    #[test]
    fn day8_part2_small_test() {
        let test = String::from(
            r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(day8_part2(test), 6);
    }

    #[test]
    fn day8_part2_test() -> Result<()> {
        let input = fs::read_to_string("./src/day8/input.txt")?;
        let r = day8_part2(input);
        println!("{}", r);
        assert_eq!(r, 10241191004509);
        Ok(())
    }
}
