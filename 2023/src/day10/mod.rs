use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

fn get_value(map: &Vec<Vec<char>>, x: i64, y: i64) -> &char {
    if x < 0 || y < 0 {
        return &'.';
    }
    if let Some(row) = map.get(x as usize) {
        return row.get(y as usize).unwrap_or(&'.');
    }
    return &'.';
}

fn walk(map: &Vec<Vec<char>>, current: (i64, i64), path: &mut BTreeSet<(i64, i64)>) {
    let directions: BTreeMap<char, [(i64, i64); 2]> = BTreeMap::from([
        ('|', [(-1, 0), (1, 0)]),
        ('-', [(0, -1), (0, 1)]),
        ('7', [(0, -1), (1, 0)]),
        ('J', [(-1, 0), (0, -1)]),
        ('L', [(0, 1), (-1, 0)]),
        ('F', [(0, 1), (1, 0)]),
    ]);
    let mut next = current;
    loop {
        let current_value = get_value(map, next.0, next.1);
        if path.contains(&next) || *current_value == '.' {
            return;
        }
        let current_direction = directions.get(current_value).unwrap();
        path.insert(next);
        if let Some(next_dir) = current_direction
            .iter()
            .find(|(x, y)| !path.contains(&(next.0 + x, next.1 + y)))
        {
            next =  (next.0 + next_dir.0, next.1 + next_dir.1);
        }
    }
}

fn day10_part1(input: String) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut path1: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut path2: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut path3: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut path4: BTreeSet<(i64, i64)> = BTreeSet::new();

    let mut start = (0, 0);
    'main: for (i, row) in map.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 'S' {
                start = (i as i64, j as i64);
                break 'main;
            }
        }
    }
    path1.insert(start);
    path2.insert(start);
    path3.insert(start);
    path4.insert(start);
    walk(&map, (start.0 + 1, start.1), &mut path1);
    walk(&map, (start.0, start.1 + 1), &mut path2);
    walk(&map, (start.0, start.1 - 1), &mut path3);
    walk(&map, (start.0 - 1, start.1), &mut path4);
    let paths = [path1, path2, path3, path4];
    paths.iter().map(|p| p.len()).sorted().last().unwrap() / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day10_part1_small_test() {
        let test = String::from(
            r".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(day10_part1(test), 4);
    }
    #[test]
    fn day10_part1_small_test_2() {
        let test = String::from(
            r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(day10_part1(test), 8);
    }

    #[test]
    fn day10_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day10/input.txt")?;

        let r = day10_part1(input);
        println!("{}", r);
        assert_eq!(r, 6820);
        Ok(())
    }
    //     #[test]
    //     fn day10_part2_small_test() {
    //         let test = String::from(
    //             r"0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45",
    //         );
    //         assert_eq!(day10_part2(test), 2);
    //     }
    //     #[test]
    //     fn day10_part2_test() -> Result<()> {
    //         let input = fs::read_to_string("./src/day10/input.txt")?;
    //         let r = day10_part2(input);
    //         println!("{}", r);
    //         assert_eq!(r, 1136);
    //         Ok(())
    //     }
}
