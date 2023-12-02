fn get_caliberation(iterator: impl Iterator<Item = char> + DoubleEndedIterator, last: bool) -> i32 {
    let translations = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut val = 0;
    let mut temp = String::new();
    'outer: for y in iterator {
        if last {
            temp.insert(0, y);
        } else {
            temp.insert(temp.len(), y);
        }

        if y.is_ascii_digit() {
            val = y as i32 - '0' as i32;
            break;
        }
        for (idx, key) in translations.iter().enumerate() {
            if temp.contains(key) {
                val = idx as i32;
                break 'outer;
            }
        }
    }
    if last {
        val
    } else {
        val * 10
    }
}
fn main() {
    let mut output = 0;
    for x in include_str!("../input").lines() {
        output += get_caliberation(x.chars(), false);
        output += get_caliberation(x.chars().rev(), true);
    }
    println!("{:?}", output);
}
