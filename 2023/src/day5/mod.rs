use std::{collections::BTreeMap, ops::Range};

fn day5_part1(input: String) -> u64 {
    let mut lines = input.lines();
    let mut maps: Vec<BTreeMap<u64, Range<u64>>> = vec![];
    let mut locations = lines
        .next()
        .expect("Wrong input")
        .split(" ")
        .filter_map(|n| n.parse::<u64>().ok())
        .collect::<Vec<u64>>();
    let mut current_map = BTreeMap::new();
    lines.for_each(|line| {
        if line.contains("map:") {
            maps.push(current_map.clone());
            current_map = BTreeMap::new();
        } else if !line.is_empty() {
            let line_nums = line
                .split(" ")
                .filter_map(|n| n.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            let destination = line_nums.get(0).expect("Wrong line format");
            let start_range = line_nums.get(1).expect("Wrong line format");
            let end_range = start_range + line_nums.get(2).expect("Wrong line format");
            current_map.insert(
                *destination,
                Range {
                    start: *start_range,
                    end: end_range,
                },
            );
        }
    });
    maps.push(current_map);
    for map in maps {
        locations = move_seeds(&locations, &map);
    }
    *locations.iter().min().expect("Wrong mapping")
}

fn day5_part2(input: String) -> u64 {
    let mut lines = input.lines();
    let mut maps: Vec<BTreeMap<u64, Range<u64>>> = vec![];
    let mut locations = lines
        .next()
        .expect("Wrong input")
        .split(" ")
        .filter_map(|n| n.parse::<u64>().ok())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect::<Vec<Range<u64>>>();
    let mut current_map = BTreeMap::new();
    lines.for_each(|line| {
        if line.contains("map:") {
            maps.push(current_map.clone());
            current_map = BTreeMap::new();
        } else if !line.is_empty() {
            let line_nums = line
                .split(" ")
                .filter_map(|n| n.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            let destination = line_nums.get(0).expect("Wrong line format");
            let start_range = line_nums.get(1).expect("Wrong line format");
            let end_range = start_range + line_nums.get(2).expect("Wrong line format");
            current_map.insert(
                *destination,
                Range {
                    start: *start_range,
                    end: end_range,
                },
            );
        }
    });
    maps.push(current_map);

    for map in maps {
        let mut new_ranges = vec![];
        while locations.len() > 0 {
            let range = locations.pop().expect("Wrong input");
            let mut matched = false;
            for (dest, map_range) in &map {
                let overlap_start = range.start.max(map_range.start);
                let overlap_end = range.end.min(map_range.end);
                if overlap_start < overlap_end {
                    new_ranges.push(Range {
                        start: (overlap_start - map_range.start + dest),
                        end: overlap_end - map_range.start + dest,
                    });
                    if overlap_start > range.start {
                        locations.push(Range {
                            start: range.start,
                            end: overlap_start,
                        });
                    }
                    if range.end > overlap_end {
                        locations.push(Range {
                            start: overlap_end,
                            end: range.end,
                        });
                    }
                    matched = true;
                    break;
                }
            }
            if !matched {
                new_ranges.push(range);
            }
        }
        locations = new_ranges;
    }
    locations
        .iter()
        .map(|loc| loc.start)
        .min()
        .expect("Should have a min location")
}

fn move_seeds(mut seeds: &Vec<u64>, map: &BTreeMap<u64, Range<u64>>) -> Vec<u64> {
    let mut moved = vec![];
    for seed in seeds.iter() {
        let mut included = false;
        for (dest, range) in map {
            if range.contains(seed) {
                moved.push(dest + (seed - range.start));
                included = true;
                break;
            }
        }
        if !included {
            moved.push(*seed);
        }
    }
    moved
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use std::fs;
    #[test]
    fn day5_part1_small_test() {
        let test = String::from(
            r"seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4",
        );
        assert_eq!(day5_part1(test), 35);
    }

    #[test]
    fn day5_part1_test() -> Result<()> {
        let input = fs::read_to_string("./src/day5/input.txt")?;

        let r = day5_part1(input);
        println!("{}", r);
        assert_eq!(r, 346433842);
        Ok(())
    }
    #[test]
    fn day5_part2_small_test() {
        let test = String::from(
            r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
        );
        assert_eq!(day5_part2(test), 46);
    }

    #[test]
    fn day5_part2_test() -> Result<()> {
        let input = fs::read_to_string("./src/day5/input.txt")?;
        let r = day5_part2(input);
        println!("{}", r);
        assert_eq!(r, 60294664);
        Ok(())
    }
}
