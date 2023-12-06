fn run(puzzle: &Vec<Vec<usize>>) -> usize {
    let times = puzzle.get(0).expect("Wrong input format");
    let distances = puzzle.get(1).expect("Wrong input format").as_slice();
    let mut total_ways: Vec<usize> = vec![];
    for (i, time) in times.iter().enumerate() {
        let distance = distances[i];
        let mut ways = 0;
        for button_hold in 1..*time {
            let current_distance = button_hold * (time - button_hold);
            if current_distance > distance {
                ways += 1;
            }
            if ways > 0 && current_distance < distance {
                break;
            }
        }
        total_ways.push(ways);
    }
    total_ways.iter().product()

}
fn day6_part1(input: String) -> usize {
    let puzzle = input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    run(&puzzle)
}

fn day6_part2(input: String) -> usize {
    let puzzle = input
        .lines()
        .map(|line| {
            line.replace(" ", "")
                .split(":")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
        run(&puzzle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day6_part1_small_test() {
        let test = String::from(
            r"Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(day6_part1(test), 288);
    }

    #[test]
    fn day6_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day6/input.txt")?;

        let r = day6_part1(input);
        println!("{}", r);
        assert_eq!(r, 861300);
        Ok(())
    }
    #[test]
    fn day6_part2_small_test() {
        let test = String::from(
            r"Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(day6_part2(test), 71503);
    }

        #[test]
        fn day6_part2_test() -> Result<()> {
            let input = fs::read_to_string("./src/day6/input.txt")?;
            let r = day6_part2(input);
            println!("{}", r);
            assert_eq!(r, 28101347);
            Ok(())
        }
}
