fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| x.split(' ').map(|y| y.parse().unwrap()).collect())
        .collect()
}
fn check_all_zero(data: &Vec<i32>) -> bool {
    data.iter().filter(|x| **x == 0).count() == data.len()
}
fn main() {
    let input = include_str!("input");
    let data = parse(input);
    let mut output = 0;
    for history in data {
        let mut workspace = Vec::new();
        workspace.push(history);
        let mut idx = 0;
        while !check_all_zero(&workspace[idx]) {
            let mut temp = Vec::new();
            for x in 0..workspace[idx].len() - 1 {
                temp.push(workspace[idx][x + 1] - workspace[idx][x]);
            }
            idx += 1;
            workspace.push(temp);
        }
        let mut iter = workspace.iter_mut().rev();
        let x = iter.next().unwrap();
        x.push(0);
        let mut last = 0;
        for x in iter {
            last += x[x.len() - 1];
            x.push(last);
        }
        output += workspace[0][workspace[0].len() - 1];
    }
    println!("Output: {}", output);
}
