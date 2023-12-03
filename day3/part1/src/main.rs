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
    
    for x in 1..input.len() - 1 {
        let mut is_part_number = false;
        for y in 1..input[x].len() - 1 {
            if input[x][y].is_ascii_digit() {
                temp.push(input[x][y]);

                if (input[x + 1][y] != '.' && !input[x + 1][y].is_ascii_digit())
                    || (input[x - 1][y] != '.' && !input[x - 1][y].is_ascii_digit())
                {
                    is_part_number = true;
                }

                if (input[x][y + 1] != '.' && !input[x][y + 1].is_ascii_digit())
                    || (input[x][y - 1] != '.' && !input[x][y - 1].is_ascii_digit())
                {
                    is_part_number = true;
                }

                if (input[x + 1][y + 1] != '.' && !input[x + 1][y + 1].is_ascii_digit())
                    || (input[x - 1][y - 1] != '.' && !input[x - 1][y - 1].is_ascii_digit())
                    || (input[x + 1][y - 1] != '.' && !input[x + 1][y - 1].is_ascii_digit())
                    || (input[x - 1][y + 1] != '.' && !input[x - 1][y + 1].is_ascii_digit())
                {
                    is_part_number = true;
                }
            } else {
                if is_part_number {
                    output += temp.parse::<i32>()?;
                }
                is_part_number = false;
                temp.clear();
                continue;
            }
        }
        if is_part_number {
            output += temp.parse::<i32>()?;
            temp.clear();
        }
    }
    println!("Output : {}", output);
    Ok(())
}
