fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let input = input.replace("\r\n", "\n");
    let lines = input.split("\n").collect::<Vec<_>>();
    
    let nums: Vec<_> = lines.iter().filter_map(|l| {
        let digits: Vec<_> = l.chars().filter(|c| c.is_ascii_digit()).collect();
        let mut num = "".to_string();
        if digits.len() == 0 { return None }
        num.push(digits[0]);
        num.push(digits[digits.len() - 1]);

        let digit = num.parse::<u32>().expect("number expected");
        return Some(digit);
    }).collect();

    let total: u32 = nums.iter().sum();

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, 142.to_string());
    }
}