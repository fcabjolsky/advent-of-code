use anyhow::{Ok, Result};
use std::collections::HashMap;

fn get_calibration(input: String, numbers: HashMap<&str, u32>) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();
    return lines
        .iter()
        .map(|line| {
            let mut first_idx = line.len();
            let mut first_digit: &str = "";
            let mut last_idx = 0;
            let mut last_digit: &str = "";
            numbers.keys().for_each(|n| {
                let result = line.find(n).unwrap_or(line.len());
                if result < first_idx {
                    first_idx = result;
                    first_digit = &n;
                }
                let result = line.rfind(n).unwrap_or(0);
                if result > last_idx {
                    last_idx = result;
                    last_digit = &n;
                }
            });

            if last_idx == 0 {
                last_digit = first_digit;
            }

            let zero = 0;
            return format!(
                "{}{}",
                numbers.get(first_digit).unwrap_or(&zero),
                numbers.get(last_digit).unwrap_or(&zero)
            )
            .parse::<u32>()
            .expect("wrong format");
        })
        .sum();
}

fn day1_part1(input: String) -> u32 {
    let numbers: HashMap<&str, u32> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    return get_calibration(input, numbers);
}

fn day1_part2(input: String) -> u32 {
    let numbers: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    return get_calibration(input, numbers);
}
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn day1_part1_small_test() {
        let test = String::from(
            r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(day1_part1(test), 142);
    }

    #[test]
    fn day1_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day1/input.txt")?;
        let r = day1_part1(input);
        println!("{}", r);
        assert_eq!(r, 54331);
        Ok(())
    }

    #[test]
    fn day1_part2_small_test() {
        let test = String::from(
            r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(day1_part2(test), 281);
    }

    #[test]
    fn day1_part2_test() -> Result<()> {
        let input = fs::read_to_string("./src/day1/input.txt")?;
        let r = day1_part2(input);
        println!("{}", r);
        assert_eq!(r, 54518);
        Ok(())
    }
}
