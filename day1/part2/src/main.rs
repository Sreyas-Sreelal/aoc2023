use std::collections::HashMap;

fn main() {
    let translations = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut output = 0;
    let mut checker: Vec<Vec<i32>> = Vec::new();
    for x in include_str!("../input").lines() {
        checker.push(Vec::new());
        let mut temp = String::new();
        for y in x.chars() {
            temp.push(y);
            let data = checker.iter_mut().last().unwrap();
            if y.is_ascii_digit() {
                data.push(y as i32 - '0' as i32);
                continue;
            }
            for key in translations.keys() {
                if temp.contains(key) {
                    data.push(translations[key]);
                    let last = temp.chars().last().unwrap();
                    temp = String::new();
                    temp.push(last);
                }
            }
        }
    }
    let checker = checker.iter().filter(|x| !x.is_empty());
    for number in checker {
        output = *number.iter().next().unwrap() * 10 + *number.iter().last().unwrap();
    }
    println!("{:?}", output);
}
