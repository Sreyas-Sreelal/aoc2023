fn main() {
    let times = vec![56, 97, 78, 75];
    let distances = vec![546, 1927, 1131, 1139];
    let mut output = 1;
    for (time, high_score) in times.iter().zip(distances.iter()) {
        let mut count = 0;
        for hold_time in 1..*time {
            let distance_traveled = (time - hold_time) * hold_time;
            if distance_traveled > *high_score {
                count += 1;
            }
        }
        output *= count;
    }
    println!("{}", output);
}
