use std::time::{SystemTime, UNIX_EPOCH};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

// 10  13  16  21  30  45  68
//    3   3   5   9  15  23
//      0   2   4   6   8
//        2   2   2   2
//          0   0   0

fn recurse(seq: Vec<i64>) -> i64 {
    let diffs = seq
        .iter()
        .zip(seq.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<i64>>();

    if diffs.iter().all(|x| *x == 0) {
        return seq.last().unwrap().clone();
    }

    recurse(diffs.clone()) + seq.last().unwrap()
}

fn process(input: &str) -> i64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let seqs = lines
        .par_iter()
        .map(|x| {
            x.split_ascii_whitespace()
                .into_iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    seqs.par_iter()
        .map(|x| recurse(x.clone()))
        .collect::<Vec<i64>>()
        .par_iter()
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 114);
    }
}
