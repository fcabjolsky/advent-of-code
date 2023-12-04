use std::collections::HashMap;

#[derive(Debug)]
struct Cube<'a> {
    colour: &'a str,
    ammount: usize,
}

impl<'a> Cube<'a> {
    fn parse(input: &'a str) -> Cube {
        let (ammount, colour) = input
            .trim()
            .split_once(' ')
            .expect("wrong number - colour format");
        Cube {
            colour,
            ammount: ammount.parse().expect("wrong ammount number"),
        }
    }
}

#[derive(Debug)]
struct Sets<'a> {
    cubes: Vec<Cube<'a>>,
}
impl<'a> Sets<'a> {
    fn parse(input: &'a str) -> Sets {
        Sets {
            cubes: input.split(',').map(Cube::parse).collect(),
        }
    }
}

#[derive(Debug)]
struct Game<'a> {
    id: usize,
    sets: Vec<Sets<'a>>,
}

impl<'a> Game<'a> {
    fn parse(input: &'a str, id: usize) -> Game {
        Game {
            id,
            sets: input
                .strip_prefix(format!("Game {id}: ").as_str())
                .expect("Wrong line format")
                .split(';')
                .map(Sets::parse)
                .collect(),
        }
    }

    fn is_valid(&self, valid_values: &HashMap<&str, usize>) -> Option<usize> {
        self.sets
            .iter()
            .all(|set| {
                set.cubes.iter().all(|cube| {
                    *valid_values.get(cube.colour).expect("Not a valid colour") >= cube.ammount
                })
            })
            .then_some(self.id)
    }
    fn get_power_set(&self) -> usize {
        let mut min_values: HashMap<&str, usize> = HashMap::new();
        self.sets.iter().for_each(|set| {
            set.cubes.iter().for_each(|cube| {
                min_values
                    .entry(cube.colour)
                    .and_modify(|v| *v = (*v).max(cube.ammount))
                    .or_insert(cube.ammount);
            });
        });
        min_values.values().product()
    }
}

fn parse<'a>(input: &'a String) -> Vec<Game<'a>> {
    let mut game: usize = 0;
    input
        .lines()
        .map(|line| {
            game += 1;
            Game::parse(line, game)
        })
        .collect()
}

pub fn day2_part1(input: String, reds: usize, blues: usize, greens: usize) -> usize {
    let valid_values = HashMap::from([("red", reds), ("green", greens), ("blue", blues)]);

    let games = parse(&input);
    games
        .iter()
        .filter_map(|game| game.is_valid(&valid_values))
        .sum()
}

pub fn day2_part2(input: String) -> usize {
    let games = parse(&input);
    games
        .iter()
        .map(|game| game.get_power_set())
        .sum()
}
