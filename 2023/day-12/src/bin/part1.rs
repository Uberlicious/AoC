use std::time::{SystemTime, UNIX_EPOCH};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

fn check_match(order: &Vec<usize>, parts: &Vec<char>) -> bool {
    let str = parts.iter().collect::<String>();

    let counts = str
        .split(".")
        .filter_map(|s| {
            // if s.is_empty() {
            //     return None;
            // }
            if s.contains('#') {
                return Some(s.chars().count());
            }
            None
        })
        .collect::<Vec<usize>>();

    *order == counts
}

fn recurse(
    order: &Vec<usize>,
    parts: &Vec<char>,
    mut idx: usize,
    matches: &mut Vec<Vec<char>>,
) -> Option<bool> {
    let mut new_parts = parts.clone();

    // let b_match = check_match(order, parts);
    // println!("matches: {b_match} parts: {new_parts:?}");

    if !parts.contains(&'?') && !matches.contains(&parts) && check_match(order, parts) {
        matches.push(parts.clone());
    }

    let chars = vec!['.', '#'];

    for i in idx..parts.len() {
        if parts[i] == '?' {
            chars.iter().for_each(|c| {
                new_parts[idx] = *c;
                recurse(order, &new_parts, idx, matches);
            })
        }
        idx += 1;
    }

    None
}

fn check(order: &Vec<usize>, parts: &Vec<char>) -> i64 {
    let mut matches = vec![];

    recurse(order, parts, 0, &mut matches);
    // println!("matches: {}", matches.len());
    matches.len() as i64
}

fn process(input: &str) -> i64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    lines
        .par_iter()
        .map(|l| {
            let split = l.split_ascii_whitespace().collect::<Vec<&str>>();
            let parts = split[0].chars().collect::<Vec<char>>();
            let order = split[1]
                .split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            check(&order, &parts)
        })
        .collect::<Vec<i64>>()
        .iter()
        .sum()

    // let l = lines[0];
    // for (_idx, l) in lines.iter().enumerate() {
    //     let split = l.split_ascii_whitespace().collect::<Vec<&str>>();
    //     let parts = split[0].chars().collect::<Vec<char>>();
    //     let order = split[1]
    //         .split(',')
    //         .filter_map(|x| x.parse::<usize>().ok())
    //         .collect::<Vec<usize>>();

    //     match_counter += check(&order, &parts);
    // }

    // let working = parts
    //     .split(".")
    //     .filter_map(|s| {
    //         if s.is_empty() {
    //             return None;
    //         }
    //         return Some(s.chars().count());
    //     })
    //     .collect::<Vec<usize>>();
    // match_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "#.#.### 1,1,3
            .#...#....###. 1,1,3
            .#.###.#.###### 1,3,1,6
            ####.#...#... 4,1,1
            #....######..#####. 1,6,5
            .###.##....# 3,2,1",
        );
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_1_2() {
        let result = process(
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1",
        );
        assert_eq!(result, 21);
    }
}
