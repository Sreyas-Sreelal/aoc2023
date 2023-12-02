#[derive(Debug, PartialEq, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
    Unkown,
}
#[derive(Debug)]
struct Cube {
    count: i32,
    color: Color,
}
#[derive(Debug)]
struct Game {
    _id: usize,
    sets: Vec<Vec<Cube>>,
}

impl Game {
    fn get_max_power(&self) -> i32 {
        let mut max_values = [0, 0, 0];
        for set in &self.sets {
            for cube in set {
                if cube.count > max_values[cube.color as usize] {
                    max_values[cube.color as usize] = cube.count;
                }
            }
        }
        max_values[0] * max_values[1] * max_values[2]
    }
}

fn get_color_from_string(data: &str) -> Color {
    match data {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => Color::Unkown,
    }
}
fn parse_game(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        let semi_pos = line.chars().position(|x| x == ':').unwrap();
        let mut sets = Vec::new();
        for token in line[semi_pos + 2..].split("; ") {
            sets.push(Vec::new());
            let set = sets.iter_mut().last().unwrap();
            for x in token.split(", ") {
                let p: Vec<&str> = x.split(' ').collect();
                set.push(Cube {
                    count: p[0].parse().unwrap(),
                    color: get_color_from_string(p[1]),
                });
            }
        }
        games.push(Game { _id: idx + 1, sets });
    }
    games
}
fn main() {
    let games = parse_game(include_str!("input"));
    let mut sum = 0;
    for game in games {
        sum += game.get_max_power();
    }
    println!("{}", sum);
}
