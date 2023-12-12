use std::{collections::HashMap, usize};

fn generate_combinations(
    pattern: &Vec<char>,
    index: usize,
    valids: Vec<usize>,
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    if index >= pattern.len() {
        return if valids.is_empty() { 1 } else { 0 };
    }
    if valids.is_empty() {
        return if pattern[index..].contains(&'#') {
            0
        } else {
            1
        };
    }
    if let Some(cached) = cache.get(&(pattern[index..].iter().collect::<String>(), valids.clone()))
    {
        return *cached;
    }

    let mut sum = 0;
    if pattern[index] == '?' || pattern[index] == '.' {
        sum += generate_combinations(pattern, index + 1, valids.clone(), cache);
    }

    if pattern[index] == '#' || pattern[index] == '?' {
        if (valids[0] <= (pattern.len() - index))
            && !pattern[index..index + valids[0]].contains(&'.')
            && (valids[0] == (pattern.len() - index) || pattern[valids[0] + index] != '#')
        {
            sum +=
                generate_combinations(pattern, index + 1 + valids[0], valids[1..].to_vec(), cache);
        }
    }
    cache.insert(
        (pattern[index..].iter().collect::<String>(), valids.clone()),
        sum,
    );
    sum
}

fn day12_part1(input: String) -> usize {
    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    input
        .lines()
        .map(|line| {
            let (row, valids) = line.split_once(" ").expect("Wrong input");
            let valids = valids
                .split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            let row = row.chars().collect::<Vec<char>>();
            generate_combinations(&row, 0, valids, &mut cache)
        })
        .sum()
}

fn day12_part2(input: String) -> usize {
    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    input
        .lines()
        .map(|line| {
            let (row, valids) = line.split_once(" ").expect("Wrong input");
            let mut final_row = vec![];
            let mut final_valid = vec![];
            for _ in 0..5 {
                let mut valids = valids
                    .split(",")
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<usize>>();
                final_valid.append(&mut valids);
                let row = row.chars().collect::<Vec<char>>();
                final_row.push(row);
            }
            generate_combinations(&mut (final_row.join(&'?')), 0, final_valid, &mut cache)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;

    #[test]
    fn day12_part1_small_test() {
        let test = String::from(
            r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(day12_part1(test), 21);
    }

    #[test]
    fn day12_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day12/input.txt")?;

        let r = day12_part1(input);
        println!("{}", r);
        assert_eq!(r, 7705);
        Ok(())
    }

    #[test]
    fn day12_part2_small_test() {
        let test = String::from(
            r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        );
        assert_eq!(day12_part2(test), 525152);
    }
    #[test]
    fn day12_part2_test() -> Result<()> {
        let input = fs::read_to_string("./src/day12/input.txt")?;
        let r = day12_part2(input);
        println!("{}", r);
        assert_eq!(r, 50338344809230);
        Ok(())
    }
}
