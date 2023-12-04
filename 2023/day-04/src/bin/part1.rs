fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");
    let lines = input.split("\n").collect::<Vec<_>>();

    let mut holder: Vec<u32> = vec![];

    for line in lines {
        let numbers = line.split(":").collect::<Vec<_>>()[1]
            .split("|")
            .collect::<Vec<_>>()
            .iter()
            .map(|n| {
                return n
                    .trim()
                    .split_ascii_whitespace()
                    .map(|n| n.trim().parse::<u32>().expect("num"))
                    .collect::<Vec<u32>>();
            })
            .collect::<Vec<Vec<u32>>>();

        let mut matches = 0;

        for n in &numbers[0] {
            if numbers[1].contains(&n) {
                matches += 1
            }
        }

        if matches == 0 {
            continue;
        }

        let pow = u32::pow(2, matches - 1);

        holder.push(pow);
    }

    holder.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }
}
