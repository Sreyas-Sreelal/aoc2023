use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Card {
    id: usize,
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
            id: idx + 1,
            winning_numbers,
            player_numbers,
        });
    }
    cards
}
fn main() {
    let cards = parse_game(include_str!("input"));
    let mut card_copy = cards.clone();
    for card in &cards {
        let common = card
            .winning_numbers
            .intersection(&card.player_numbers)
            .count();
        let count = card_copy.iter().filter(|x| x.id == card.id).count();
        for x in card.id + 1..=card.id + common {
            for _ in 0..count {
                card_copy.push(cards[x - 1].clone());
            }
        }
    }
    println!("{}", card_copy.len());
}
