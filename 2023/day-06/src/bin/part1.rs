fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let mut holder: Vec<u32> = vec![];

    let times = lines[0].split(":").collect::<Vec<_>>()[1]
        .split_ascii_whitespace()
        .into_iter()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let distances = lines[1].split(":").collect::<Vec<_>>()[1]
        .split_ascii_whitespace()
        .into_iter()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for (idx, time) in times.iter().enumerate() {
        let mut wins = 0;
        for t in 0..*time {
            let d = t * (time - t);

            if d > distances[idx] {
                wins += 1;
            }
        }
        holder.push(wins);
    }

    holder.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 288);
    }
}
