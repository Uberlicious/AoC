fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let time = lines[0].split(":").collect::<Vec<_>>()[1]
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = lines[1].split(":").collect::<Vec<_>>()[1]
        .split_ascii_whitespace()
        .into_iter()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut wins = 0;
    for t in 0..time {
        let d = t * (time - t);

        if d > distance {
            wins += 1;
        }
    }

    wins
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
        assert_eq!(result, 71503);
    }
}
