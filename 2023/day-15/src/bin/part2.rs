use std::{
    collections::{hash_map::Entry::*, HashMap},
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

fn hash(s: &[u8]) -> u8 {
    let mut current = 0;
    s.iter().for_each(|&b| {
        current = ((current + b as u32) * 17) % 256;
    });
    current as u8
}

fn process(input: &str) -> usize {
    let input = input
        .split(',')
        .map(|s| s.as_bytes())
        .collect::<Vec<&[u8]>>();

    let mut boxes: Vec<Vec<(&[u8], u8)>> = vec![vec![]; 256];

    input.iter().for_each(|&b| {
        if b.contains(&b'=') {
            let label = &b[0..&b.len() - 2];
            let hash = hash(label);
            let lens = std::str::from_utf8(&b[b.len() - 1..])
                .expect("utf8")
                .parse::<u8>()
                .expect("not a number");

            match boxes[hash as usize].iter_mut().find(|x| x.0 == label) {
                Some(b) => {
                    b.1 = lens;
                }
                None => boxes[hash as usize].push((label, lens)),
            }
        } else {
            let label = &b[0..&b.len() - 1];
            let hash = hash(label);
            match boxes[hash as usize].iter().position(|x| x.0 == label) {
                Some(i) => {
                    boxes[hash as usize].remove(i);
                }
                None => {}
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|b| {
            b.1.iter()
                .enumerate()
                .map(|l| (b.0 + 1) * (l.0 + 1) * l.1 .1 as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 145);
    }
}
