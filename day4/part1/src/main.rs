use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    _id: usize,
    winning_numbers: HashSet<i32>,
    player_numbers: HashSet<i32>,
}
fn parse_game(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        let colon_pos = line.find(':').unwrap();
        let raw_numbers: Vec<&str> = line[colon_pos + 2..].split(" | ").collect();
        let winning_numbers = raw_numbers[0]
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        let player_numbers = raw_numbers[1]
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();

        cards.push(Card {
            _id: idx + 1,
            winning_numbers,
            player_numbers,
        });
    }
    cards
}
fn main() {
    let cards = parse_game(include_str!("input"));

    let mut output = 0;
    for card in cards {
        let common = card
            .winning_numbers
            .intersection(&card.player_numbers)
            .count();
        if common > 0 {
            output += 2_i32.pow(common as u32 - 1);
        }
    }
    println!("{:?}", output);
}
