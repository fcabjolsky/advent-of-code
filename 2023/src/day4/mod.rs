use std::{collections::HashMap, str};

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space0, space1},
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Card<'a> {
    id: &'a str,
    elf_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl<'a> Card<'a> {
    fn get_win_count(&self) -> usize {
        self.elf_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(&number))
            .count()
    }
}

fn nums(input: &str) -> IResult<&str, Vec<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(nums, tuple((tag("|"), space1)), nums)
        .map(|(winning_numbers, elf_numbers)| Card {
            id,
            elf_numbers,
            winning_numbers,
        })
        .parse(input)
}

fn parse(input: &str) -> Vec<Card> {
    let (_, games) = separated_list1(line_ending, card)(input).expect("should parse");
    games
}

fn day4_part1(input: String) -> usize {
    let cards = parse(input.as_str());
    cards
        .iter()
        .map(|card| {
            let count = card.get_win_count() as u32;
            if count > 0 {
                2usize.pow(count - 1)
            } else {
                0
            }
        })
        .sum()
}

fn day4_part2(input: String) -> usize {
    let cards = parse(input.as_str());
    let mut wining_copies: HashMap<usize, usize> = (0..cards.len()).map(|i| (i, 1)).collect();
    cards.iter().for_each(|card| {
        let id = card.id.parse::<usize>().expect("wrong id");
        let count = card.get_win_count();
        let current_instances = wining_copies.get(&id).unwrap_or(&1).clone();
        for n in 1..(count + 1) {
            wining_copies
                .entry(n + id)
                .and_modify(|e| *e += current_instances)
                .or_insert(current_instances);
        }
    });
    wining_copies.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day4_part1_small_test() {
        let test = String::from(
            r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(day4_part1(test), 13);
    }

    #[test]
    fn day4_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day4/input.txt")?;

        let r = day4_part1(input);
        println!("{}", r);
        assert_eq!(r, 21088);
        Ok(())
    }

    #[test]
    fn day4_part2_small_test() {
        let test = String::from(
            r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(day4_part2(test), 30);
    }

    #[test]
    fn day4_part2_test() -> Result<()> {
        let input = fs::read_to_string("./src/day4/input.txt")?;
        let r = day4_part2(input);
        println!("{}", r);
        assert_eq!(r, 6874754);
        Ok(())
    }
}
