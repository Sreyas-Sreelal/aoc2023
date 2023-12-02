#[derive(Debug, PartialEq)]
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
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in &self.sets {
            for cube in set {
                if cube.color == Color::Red && cube.count > max_red {
                    max_red = cube.count;
                } else if cube.color == Color::Green && cube.count > max_green {
                    max_green = cube.count;
                } else if cube.color == Color::Blue && cube.count > max_blue {
                    max_blue = cube.count;
                }
            }
        }
        max_red * max_green * max_blue
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
                let p: Vec<&str> = x.split(" ").collect();
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
