use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Debug, PartialEq)]
enum HandType {
    // Five of a kind, where all five cards have the same label: AAAAA
    FiveOfAKind,
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind,
    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind,
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair,
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair,
    // High card, where all cards' labels are distinct: 23456
    HighCard,
}

#[derive(Debug)]
struct Rank {
    hand_type: HandType,
    idx: usize,
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: usize,
    rank: Rank,
}

impl<'a> Hand<'a> {
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

        let mut max_count = *cards_count.values().max().expect("To have one max count");

        if part2 {
            let j_count = cards_count.remove(&'J').unwrap_or_default();
            max_count = *cards_count.values().max().unwrap_or(&0usize);
            max_count += j_count;
        }

        let rank = match max_count {
            1 => Rank {
                hand_type: HandType::HighCard,
                idx: 1,
            },
            2 => {
                if cards_count.len() == 4 {
                    Rank {
                        hand_type: HandType::OnePair,
                        idx: 2,
                    }
                } else {
                    Rank {
                        hand_type: HandType::TwoPair,
                        idx: 3,
                    }
                }
            }
            3 => {
                if cards_count.len() == 3 {
                    Rank {
                        hand_type: HandType::ThreeOfAKind,
                        idx: 4,
                    }
                } else {
                    Rank {
                        hand_type: HandType::FullHouse,
                        idx: 5,
                    }
                }
            }
            4 => Rank {
                hand_type: HandType::FourOfAKind,
                idx: 6,
            },
            5 => Rank {
                hand_type: HandType::FiveOfAKind,
                idx: 7,
            },
            _ => unreachable!(),
        };

        Hand {
            cards,
            bid: bid.parse().expect("Wrong bid format"),
            rank,
        }
    }
}

fn run(input: String, cards_strenght: &BTreeMap<char, i32>, part2: bool) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::new(line, part2)).collect();
    hands.sort_by(|a, b| {
        if a.rank.hand_type == b.rank.hand_type {
            let a_chars = a.cards.chars().collect::<Vec<char>>();
            let b_chars = b.cards.chars().collect::<Vec<char>>();

            for i in 0..a.cards.len() {
                let a_char = a_chars.get(i).expect("Wrong card format");
                let b_char = b_chars.get(i).expect("Wrong card format");
                let a_str = cards_strenght.get(a_char).expect("Wrong card");
                let b_str = cards_strenght.get(b_char).expect("Wrong card");
                if a_str > b_str {
                    return Ordering::Greater;
                }
                if a_str < b_str {
                    return Ordering::Less;
                }
            }
        }
        a.rank.idx.partial_cmp(&b.rank.idx).expect("Invalid rank")
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
