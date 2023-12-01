fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

const NUMERALS: [(&str, u32); 20] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
]; 

fn process(input: &str) -> u32 {    
    input.lines().map(|line| {
        let mut numbers = Vec::new();
        for curr in 0..line.len() {
            for (numeral, value) in NUMERALS.iter() {
                if line[curr..].starts_with(numeral) {
                    numbers.push(value);
                    break;
                }
         
            }
        }
        let first = *numbers.first().unwrap();
        let last = *numbers.last().unwrap();
        return (first * 10) + last
    }).sum() 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
);
        assert_eq!(result, 281);
    }
}