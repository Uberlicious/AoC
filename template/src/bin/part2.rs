fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    todo!().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let result = process(include_str!("./input2.txt"));
        assert_eq!(result, 142.to_string());
    }
}