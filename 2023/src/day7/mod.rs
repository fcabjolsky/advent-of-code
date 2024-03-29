use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Debug, PartialEq, Clone, Copy)]
enum HandType {
    /// Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind = 7,
    /// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind = 6,
    /// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse = 5,
    /// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind = 4,
    /// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair = 3,
    /// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair = 2,
    /// High card, where all cards' labels are distinct: 23456
    HighCard = 1,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn new(input: &str, part2: bool) -> Hand {
        let (cards, bid) = input.split_once(" ").expect("Wrong line format");
        let mut cards_count: BTreeMap<char, usize> =
            cards
                .chars()
                .fold(BTreeMap::new(), |mut cards_count_acc, card| {
                    cards_count_acc
                        .entry(card)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    cards_count_acc
                });

        let mut j_count = 0;
        if part2 {
            j_count = cards_count.remove(&'J').unwrap_or_default();
        }
        let mut max_count = *cards_count.values().max().unwrap_or(&0usize);
        max_count += j_count;

        let hand_type = match (max_count, cards_count.len()) {
            (1, _) => HandType::HighCard,
            (2, 4) => HandType::OnePair,
            (2, _) => HandType::TwoPair,
            (3, 3) => HandType::ThreeOfAKind,
            (3, _) => HandType::FullHouse,
            (4, _) => HandType::FourOfAKind,
            (5, _) => HandType::FiveOfAKind,
            _ => unreachable!(),
        };

        Hand {
            cards: cards.chars().collect(),
            bid: bid.parse().expect("Wrong bid format"),
            hand_type,
        }
    }
}

fn run(input: String, cards_strenght: &BTreeMap<char, i32>, part2: bool) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::new(line, part2)).collect();
    hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            for (a_char, b_char) in a.cards.iter().zip(b.cards.iter()) {
                if a_char == b_char {
                    continue;
                }

                let a_str = cards_strenght.get(a_char).expect("Wrong card");
                let b_str = cards_strenght.get(b_char).expect("Wrong card");
                return a_str
                    .partial_cmp(b_str)
                    .expect("Strenghts to be comparable");
            }
        }
        (a.hand_type as u8)
            .partial_cmp(&(b.hand_type as u8))
            .expect("Invalid rank")
    });

    let mut starting_rank = 0;
    hands
        .iter()
        .map(|hand| {
            starting_rank += 1;
            hand.bid * starting_rank
        })
        .sum()
}

fn day7_part1(input: String) -> usize {
    let cards_strenght = BTreeMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    run(input, &cards_strenght, false)
}

fn day7_part2(input: String) -> usize {
    let cards_strenght = BTreeMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    run(input, &cards_strenght, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day7_part1_small_test() {
        let test = String::from(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(day7_part1(test), 6440);
    }

    #[test]
    fn day7_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day7/input.txt")?;

        let r = day7_part1(input);
        println!("{}", r);
        assert_eq!(r, 253910319);
        Ok(())
    }
    #[test]
    fn day7_part2_small_test() {
        let test = String::from(
            r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(day7_part2(test), 5905);
    }

    #[test]
    fn day7_part2_test() -> Result<()> {
        let input = fs::read_to_string("./src/day7/input.txt")?;
        let r = day7_part2(input);
        println!("{}", r);
        assert_eq!(r, 254083736);
        Ok(())
    }
}
