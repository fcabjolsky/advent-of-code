use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
fn day11(input: String, factor: usize) -> usize {
    let mut map = vec![];
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];
    input.lines().enumerate().for_each(|(i, line)| {
        if !line.contains("#") {
            empty_rows.push(i);
        }
        map.push(line.chars().collect::<Vec<char>>());
    });
    'main: for (j, char) in map
        .first()
        .expect("Wrong map input")
        .clone()
        .iter()
        .enumerate()
    {
        if char == &'#' {
            continue;
        }
        for line in map.iter() {
            if line.get(j).expect("Wrong map input") == &'#' {
                continue 'main;
            }
        }
        empty_cols.push(j);
    }
    let mut nums = vec![];
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, ch)| {
            if ch == &'#' {
                nums.push((i, j));
            }
        });
    });
    let pairs = nums
        .into_iter()
        .combinations(2)
        .map(|c| {
            let start = *c.first().unwrap();
            let end = *c.last().unwrap();
            (start, end)
        })
        .collect::<Vec<((usize, usize), (usize, usize))>>();

    pairs
        .into_par_iter()
        .fold(
            || 0,
            |acc, (start, end)| {
                manhattan_distance(
                    (start.0, start.1),
                    (end.0, end.1),
                    &empty_rows,
                    &empty_cols,
                    factor,
                ) + acc
            },
        )
        .reduce(|| 0, usize::wrapping_add)
}

fn manhattan_distance(
    start: (usize, usize),
    end: (usize, usize),
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    grow: usize,
) -> usize {
    let mut path_size = 0;
    for i in start.0.min(end.0)..start.0.max(end.0) {
        path_size += if empty_rows.contains(&i) { grow } else { 1 };
    }
    for j in start.1.min(end.1)..start.1.max(end.1) {
        path_size += if empty_cols.contains(&j) { grow } else { 1 };
    }
    path_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day11_part1_small_test() {
        let test = String::from(
            r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
",
        );
        assert_eq!(day11(test, 2), 374);
    }

    #[test]
    fn day11_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day11/input.txt")?;

        let r = day11(input, 2);
        println!("{}", r);
        assert_eq!(r, 9734203);
        Ok(())
    }

    #[test]
    fn day11_part2_small_test() {
        let test = String::from(
            r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
",
        );
        assert_eq!(day11(test, 100), 8410);
    }
    #[test]
    fn day11_part2_test() -> Result<()> {
        let input = fs::read_to_string("./src/day11/input.txt")?;
        let r = day11(input, 1000000);
        println!("{}", r);
        assert_eq!(r, 568914596391);
        Ok(())
    }
}
