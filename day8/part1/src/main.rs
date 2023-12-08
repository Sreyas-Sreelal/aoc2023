use std::collections::BTreeMap;
#[derive(Debug)]
struct Map {
    instructions: Vec<char>,
    nodes: BTreeMap<String,(String,String)>
}

fn parse(input:&str) -> Map {
    let mut lines = input.lines();
    let mut nodes = BTreeMap::new();
    let line = lines.next().unwrap();
    let instructions = line.chars().collect();

    // empty line
    lines.next();
    for x in lines {
        let tokens:Vec<&str> = x.split(" = ").collect();
        let key = tokens[0].to_string();
        let pair = (tokens[1][1..4].to_string(),tokens[1][6..9].to_string());
        nodes.insert(key,pair);
    }
    Map {
        instructions,
        nodes,
    }

}

fn main() {
    let input = include_str!("input");
    let map = parse(input);
    println!("{:?}",map);
    let mut node = "AAA".to_string();
    let mut idx = 0;
    let mut output = 0;
    while node!= "ZZZ" {
        let direction = map.instructions[idx];
        if direction == 'R' {
            node = map.nodes[&node].1.clone();
        } else {
            node = map.nodes[&node].0.clone();
        }
        idx = (idx+1)%map.instructions.len();
        output+=1;
    }
    println!("Output: {}",output);
}

