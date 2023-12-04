use std::{collections::HashMap, usize};

const POINT: char = '.';
const STAR: char = '*';

fn get_map_numbers(input: String) -> (Vec<Vec<char>>, HashMap<(usize, usize), String>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut numbers: HashMap<(usize, usize), String> = HashMap::new();
    let mut map: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    for (i, line) in lines.iter().enumerate() {
        let mut number = String::new();
        map.push(line.chars().collect());
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                number.push(c);
            } else if number.len() > 0 {
                numbers.insert((i, j - number.len()), number.clone());
                number = String::new();
            }
        }
        if number.len() > 0 {
            numbers.insert((i, line.len() - number.len()), number.clone());
        }
    }

    (map, numbers)
}

fn check_range(
    map: &Vec<Vec<char>>,
    value: usize,
    x: usize,
    start: usize,
    end: usize,
    gears: &mut HashMap<(usize, usize), Vec<usize>>,
) -> bool {
    let row = map.get(x).expect("wrong row");
    for (gy, c) in row
        .get(start..end)
        .expect("wrong columns")
        .iter()
        .enumerate()
    {
        if check_value(c, x, gy + start, value, gears) {
            return true;
        }
    }
    return false;
}

fn check_value(
    value: &char,
    x: usize,
    y: usize,
    num: usize,
    gears: &mut HashMap<(usize, usize), Vec<usize>>,
) -> bool {
    if !value.is_digit(10) && *value != POINT {
        if *value == STAR {
            gears
                .entry((x, y))
                .and_modify(|e| e.push(num))
                .or_insert(vec![num]);
        }
        return true;
    }
    false
}

fn day3(input: String) -> (usize, usize) {
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let (map, numbers) = get_map_numbers(input);
    let mut sum = 0;
    'main: for (key, value) in numbers.iter() {
        let num = value.parse::<usize>().expect("wrong number format");
        let row = map.get(key.0).expect("wrong row");

        // left same row
        if key.1 > 0 {
            let l_v = row.get(key.1 - 1).unwrap_or(&POINT);
            if check_value(l_v, key.0, key.1 - 1, num, &mut gears) {
                sum += num;
                continue;
            }
        }

        // right same row
        let r_v = row.get(key.1 + value.len()).unwrap_or(&POINT);
        if check_value(r_v, key.0, key.1 + value.len(), num, &mut gears) {
            sum += num;
            continue;
        }

        let start = key.1.saturating_sub(1);
        let mut end = start + value.len() + 2;
        end = if end < row.len() { end } else { row.len() };
        
        // up
        if key.0 > 0 {
            if check_range(&map, num, key.0 - 1, start, end, &mut gears) {
                sum += num;
                continue 'main;
            }
        }
        //down
        if key.0 < map.len() - 1 {
            if check_range(&map, num, key.0 + 1, start, end, &mut gears) {
                sum += num;
                continue 'main;
            }
        }
    }
    let gears_sum = gears
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(_, v)| v.iter().product::<usize>())
        .sum();
    (sum, gears_sum)
}

mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day3_part1_small_test() {
        let test = String::from(
            r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(day3(test).0, 4361);
    }

    #[test]
    fn day3_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day3/input.txt")?;

        let r = day3(input).0;
        println!("{}", r);
        assert_eq!(r, 529618);
        Ok(())
    }

    #[test]
    fn day3_part2_small_test() {
        let test = String::from(
            r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(day3(test).1, 467835);
    }
    //
    #[test]
    fn day3_part2_test() -> Result<()> {
        let input = fs::read_to_string("./src/day3/input.txt")?;
        let r = day3(input).1;
        println!("{}", r);
        assert_eq!(r, 77509019);
        Ok(())
    }
}
