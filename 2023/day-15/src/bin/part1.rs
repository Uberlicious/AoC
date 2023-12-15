use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

fn process(input: &str) -> usize {
    let input = input
        .split(',')
        .map(|s| s.as_bytes())
        .collect::<Vec<&[u8]>>();

    input
        .iter()
        .map(|b| {
            let mut current = 0;
            b.iter().for_each(|&b| {
                current = ((current + b as u32) * 17) % 256;
            });
            current
        })
        .sum::<u32>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process("HASH");
        assert_eq!(result, 52)
    }

    #[test]
    fn test_part_1_2() {
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 1320);
    }
}
