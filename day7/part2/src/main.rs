use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Kind {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

/* impl TryFrom<usize> for Kind {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            6  => Ok(Kind::Five),
            5  => Ok(Kind::Four),
            4  => Ok(Kind::Full),
            3  => Ok(Kind::Three),
            2 => Ok(Kind::Two),
            1 => Ok(Kind::One),
            0  => Ok(Kind::High),
            _ => Err(()),
        }
    }
} */
#[derive(Debug)]
struct Hand {
    label: String,
    bid: i32,
    kind: Kind,
}

fn is_five(label: &str) -> bool {
    let mut set = HashMap::new();
    for x in label.chars() {
        set.insert(x, if let Some(y) = set.get(&x) { y + 1 } else { 1 });
    }
    let ve: Vec<i32> = set.values().copied().collect();
    ve.contains(&5) && ve.len() == 1
}

fn is_four(label: &str) -> bool {
    let mut set = HashMap::new();
    for x in label.chars() {
        set.insert(x, if let Some(y) = set.get(&x) { y + 1 } else { 1 });
    }
    let ve: Vec<i32> = set.values().copied().collect();
    ve.contains(&4) && ve.contains(&1) && ve.len() == 2
}

fn is_full(label: &str) -> bool {
    let mut set = HashMap::new();
    for x in label.chars() {
        set.insert(x, if let Some(y) = set.get(&x) { y + 1 } else { 1 });
    }
    let ve: Vec<i32> = set.values().copied().collect();
    ve.contains(&3) && ve.contains(&2) && ve.len() == 2
}

fn is_three(label: &str) -> bool {
    let mut set = HashMap::new();
    for x in label.chars() {
        set.insert(x, if let Some(y) = set.get(&x) { y + 1 } else { 1 });
    }
    let ve: Vec<i32> = set.values().copied().collect();
    ve.contains(&3) && ve.contains(&1) && ve.len() == 3
}

fn is_two(label: &str) -> bool {
    let mut set = HashMap::new();
    for x in label.chars() {
        set.insert(x, if let Some(y) = set.get(&x) { y + 1 } else { 1 });
    }
    let ve: Vec<i32> = set.values().copied().collect();
    ve.contains(&2) && ve.contains(&1) && ve.len() == 3
}

fn is_one(label: &str) -> bool {
    let mut set = HashMap::new();
    for x in label.chars() {
        set.insert(x, if let Some(y) = set.get(&x) { y + 1 } else { 1 });
    }
    let ve: Vec<i32> = set.values().copied().collect();
    ve.contains(&2) && ve.len() == 4
}
fn is_high(label: &str) -> bool {
    let mut set = HashMap::new();
    for x in label.chars() {
        set.insert(x, if let Some(y) = set.get(&x) { y + 1 } else { 1 });
    }
    let ve: Vec<i32> = set.values().copied().collect();
    ve.contains(&1) && ve.len() == 5
}
impl Hand {
    pub fn new(label: String, bid: i32) -> Hand {
        let mut kind: Kind;

        if is_five(&label) {
            kind = Kind::Five;
        } else if is_high(&label) {
            kind = Kind::High;
        } else if is_full(&label) {
            kind = Kind::Full;
        } else if is_four(&label) {
            kind = Kind::Four;
        } else if is_three(&label) {
            kind = Kind::Three;
        } else if is_one(&label) {
            kind = Kind::One;
        } else if is_two(&label) {
            kind = Kind::Two;
        } else {
            panic!("Unexepcted Kind for {}", label);
        }

        let count = label.chars().filter(|x| *x == 'J').count();
        match kind {
            Kind::Four => {
                if count == 1 || count == 4 {
                    kind = Kind::Five;
                }
            }
            Kind::Full => {
                if count == 2 || count == 3 {
                    kind = Kind::Five;
                }
            }
            Kind::Three => {
                if count == 1 || count == 3 {
                    kind = Kind::Four;
                }
            }
            Kind::Two => {
                if count == 2 {
                    kind = Kind::Four;
                }
                if count == 1 {
                    kind = Kind::Full;
                }
            }
            Kind::One => {
                if count == 2 || count == 1 {
                    kind = Kind::Three;
                }
            }
            Kind::High => {
                if count == 1 {
                    kind = Kind::One;
                }
            }
            _ => {}
        }

        Hand { label, bid, kind }
    }
}
fn translate(text: &str) -> String {
    let mut output = String::new();
    let table = HashMap::from([
        ('A', 'A'),
        ('K', 'B'),
        ('Q', 'C'),
        ('T', 'E'),
        ('9', 'F'),
        ('8', 'G'),
        ('7', 'H'),
        ('6', 'I'),
        ('5', 'J'),
        ('4', 'K'),
        ('3', 'L'),
        ('2', 'M'),
        ('J', 'N'),
    ]);
    for x in text.chars() {
        output.push(table[&x]);
    }
    output
}
fn parse(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for x in input.lines() {
        let line: Vec<&str> = x.split(' ').collect();
        let string = line[0].to_string();
        hands.push(Hand::new(string, line[1].parse().unwrap()));
    }
    hands
}
fn main() {
    let input = include_str!("input");
    let mut hands = parse(input);
    hands.sort_by(|a, b| {
        a.kind
            .cmp(&b.kind)
            .then(translate(&a.label).cmp(&translate(&b.label)))
    });
    hands.reverse();

    let mut output = 0;
    for (idx, hand) in hands.iter().enumerate() {
        output += (idx + 1) as i32 * hand.bid;
    }

    println!("Output: {}", output);
}
