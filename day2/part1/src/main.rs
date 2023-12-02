#[derive(Debug, PartialEq, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
    Unkown,
}
static MAX_VALUE: [i32; 3] = [12, 13, 14];
#[derive(Debug)]
struct Cube {
    count: i32,
    color: Color,
}
#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Vec<Cube>>,
}

impl Game {
    fn is_valid(&self) -> bool {
        for set in &self.sets {
            for cube in set {
                if cube.count > MAX_VALUE[cube.color as usize] {
                    return false;
                }
            }
        }
        true
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
        games.push(Game { id: idx + 1, sets });
    }
    games
}
fn main() {
    let games = parse_game(include_str!("input"));
    let mut sum = 0;
    for game in games {
        if game.is_valid() {
            sum += game.id;
        }
    }
    println!("{}", sum);
}
