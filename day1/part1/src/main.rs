fn main() {
    let mut output = 0;
    for x in include_str!("../input").lines() {
        let number: String = x.chars().filter(|x| x.is_ascii_digit()).collect();
        output += (number.chars().next().unwrap() as u32 - '0' as u32) * 10
                + (number.chars().last().unwrap() as u32 - '0' as u32);
    }
    println!("{}", output);
}
