use std::{collections::HashMap, usize};
mod refactor;

fn day2_part1(
    input: String,
    ammount_red: usize,
    ammount_blue: usize,
    ammount_green: usize,
) -> usize {
    let mut game: usize = 0;
    input
        .lines()
        .map(|line| {
            game += 1;
            let sets: Vec<&str> = line
                .strip_prefix(format!("Game {game}: ").as_str())
                .expect("Wrong line format")
                .split(";")
                .collect();
            for set in sets {
                let cubes: Vec<&str> = set.split(',').collect();
                for cube in cubes {
                    let (ammount, colour) = cube
                        .trim()
                        .split_once(' ')
                        .expect("wrong number - colour format");
                    let ammount = ammount.parse::<usize>().expect("wrong number of cubes");
                    match colour {
                        "red" => {
                            if ammount > ammount_red {
                                return 0;
                            }
                        }
                        "blue" => {
                            if ammount > ammount_blue {
                                return 0;
                            }
                        }
                        "green" => {
                            if ammount > ammount_green {
                                return 0;
                            }
                        }
                        _ => (),
                    }
                }
            }
            game
        })
        .sum()
}

fn day2_part2(input: String) -> usize {
    let mut game: usize = 0;
    input
        .lines()
        .map(|line| {
            game += 1;
            let sets: Vec<&str> = line
                .strip_prefix(format!("Game {game}: ").as_str())
                .expect("Wrong line format")
                .split(";")
                .collect();
            let mut min_red = 0;
            let mut min_blue = 0;
            let mut min_green = 0;
            for set in sets {
                let cubes: Vec<&str> = set.split(',').collect();
                for cube in cubes {
                    let (ammount, colour) = cube
                        .trim()
                        .split_once(' ')
                        .expect("wrong number - colour format");
                    let ammount = ammount.parse::<usize>().expect("wrong number of cubes");
                    match colour {
                        "red" => {
                            min_red = min_red.max(ammount);
                        }
                        "blue" => {
                            min_blue = min_blue.max(ammount);
                        }
                        "green" => {
                            min_green = min_green.max(ammount);
                        }
                        _ => (),
                    }
                }
            }
            min_green * min_blue * min_red
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day2_part1_small_test() {
        let test = String::from(
            r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(day2_part1(test.clone(), 12, 14, 13), 8);
        assert_eq!(refactor::day2_part1(test, 12, 14, 13), 8);
    }

    #[test]
    fn day2_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day2/input.txt")?;

        let r = refactor::day2_part1(input.clone(), 12, 14, 13);
        println!("{}", r);
        assert_eq!(r, 2551);
        assert_eq!(refactor::day2_part1(input, 12, 14, 13), 2551);
        Ok(())
    }

        #[test]
        fn day2_part2_small_test() {
            let test = String::from(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            );
            assert_eq!(refactor::day2_part2(test.clone()),2286);
            assert_eq!(day2_part2(test),2286);
        }

        #[test]
        fn day2_part2_test() -> Result<()> {
            let input = fs::read_to_string("./src/day2/input.txt")?;
            let r = refactor::day2_part2(input.clone());
            println!("{}", r);
            assert_eq!(r, 62811);
            assert_eq!(refactor::day2_part2(input), 62811);
            Ok(())
        }
}
