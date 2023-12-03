use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<Vec<char>> = include_str!("input")
        .lines()
        .map(|x| {
            let mut x = x.to_string();
            x.insert(0, '.');
            x.insert(x.len(), '.');
            x.chars().collect()
        })
        .collect();

    input.insert(0, vec!['.'; input[0].len()]);
    input.insert(input.len(), vec!['.'; input[0].len()]);
    
    let mut output = 0;
    let mut temp = String::new();
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for x in 1..input.len() - 1 {
        let mut is_part_number = false;
        let mut gear_found_loc = (0, 0);
        for y in 1..input[x].len() - 1 {
            if input[x][y].is_ascii_digit() {
                temp.push(input[x][y]);

                if input[x + 1][y] != '.' && !input[x + 1][y].is_ascii_digit() {
                    is_part_number = true;
                    if input[x + 1][y] == '*' {
                        gear_found_loc = (x + 1, y);
                    }
                }
                if input[x - 1][y] != '.' && !input[x - 1][y].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x - 1, y);
                }

                if input[x][y + 1] != '.' && !input[x][y + 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x, y + 1);
                }
                if input[x][y - 1] != '.' && !input[x][y - 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x, y - 1);
                }

                if input[x + 1][y + 1] != '.' && !input[x + 1][y + 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x + 1, y + 1);
                }

                if input[x - 1][y - 1] != '.' && !input[x - 1][y - 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x - 1, y - 1);
                }
                if input[x + 1][y - 1] != '.' && !input[x + 1][y - 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x + 1, y - 1);
                }
                if input[x - 1][y + 1] != '.' && !input[x - 1][y + 1].is_ascii_digit() {
                    is_part_number = true;
                    gear_found_loc = (x - 1, y + 1);
                }
            } else {
                if is_part_number && gear_found_loc != (0, 0) {
                    gears.entry(gear_found_loc).or_insert_with(Vec::new);
                    gears
                        .get_mut(&gear_found_loc)
                        .unwrap()
                        .push(temp.parse::<i32>()?);
                    gear_found_loc = (0, 0);
                }
                is_part_number = false;
                temp.clear();
                continue;
            }
        }
        if is_part_number {
            if gear_found_loc != (0, 0) {
                gears.entry(gear_found_loc).or_insert_with(Vec::new);
                gears
                    .get_mut(&gear_found_loc)
                    .unwrap()
                    .push(temp.parse::<i32>()?);
            }
            temp.clear();
        }
    }

    for v in gears.values() {
        if v.len() == 2 {
            output += v[0] * v[1];
        }
    }
    println!("Output : {:?}", output);
    Ok(())
}
