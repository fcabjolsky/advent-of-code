use itertools::Itertools;
fn day9_part1(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            let current_values = line
                .split(" ")
                .filter_map(|values| values.parse::<i64>().ok())
                .collect::<Vec<i64>>();

            let mut sequences = vec![current_values];
            loop {
                let current_history = sequences.last().expect("Invalid sequence");
                let mut differences = vec![];
                
                for (current, next) in current_history.iter().tuple_windows() {
                    differences.push(*next - *current);
                }
                if differences.iter().all(|d| *d == 0) {
                    break;
                }
                sequences.push(differences);
            }
            sequences.reverse();
            sequences.iter().fold(0, |acc, sequence| {
                acc + sequence.last().expect("Invalid sequence")
            })
        })
        .sum()
}

fn day9_part2(input: String) -> i64 {
    input
        .lines()
        .map(|line| {
            let current_values = line
                .split(" ")
                .filter_map(|values| values.parse::<i64>().ok())
                .collect::<Vec<i64>>();

            let mut sequences = vec![current_values];
            loop {
                let current_history = sequences.last().expect("Invalid sequence");
                let mut differences = vec![];
                for (current, next) in current_history.iter().tuple_windows() {
                    differences.push(*next - *current);
                }
                if differences.iter().all(|d| *d == 0) {
                    break;
                }
                sequences.push(differences);
            }
            let starting = sequences
                .pop()
                .expect("Invalid input")
                .first()
                .expect("Invalid sequence")
                .to_owned();

            sequences.reverse();
            sequences.iter().fold(starting, |acc, sequence| {
                sequence.first().expect("Invalid sequence") - acc
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day9_part1_small_test() {
        let test = String::from(
            r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(day9_part1(test), 114);
    }

    #[test]
    fn day9_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day9/input.txt")?;

        let r = day9_part1(input);
        println!("{}", r);
        assert_eq!(r, 1684566095);
        Ok(())
    }
    #[test]
    fn day9_part2_small_test() {
        let test = String::from(
            r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(day9_part2(test), 2);
    }
    #[test]
    fn day9_part2_test() -> Result<()> {
        let input = fs::read_to_string("./src/day9/input.txt")?;
        let r = day9_part2(input);
        println!("{}", r);
        assert_eq!(r, 1136);
        Ok(())
    }
}
