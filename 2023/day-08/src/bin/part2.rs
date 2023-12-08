use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use num::Integer;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

fn find(
    guide: Vec<char>,
    guide_map: HashMap<String, (String, String)>,
    mut search: Vec<String>,
) -> i64 {
    let mut cycles = vec![];

    for s in 0..search.len() {
        let mut iters = 0;
        loop {
            let next = guide[iters as usize % guide.len()];
            iters += 1;
            if next == 'L' {
                search[s] = guide_map[&search[s]].0.clone();
            } else {
                search[s] = guide_map[&search[s]].1.clone();
            }

            if search[s].ends_with('Z') {
                break;
            }
        }
        cycles.push(iters);
    }

    cycles.iter().fold(1, |a, b| a.lcm(b))
}

fn process(input: &str) -> i64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let guide = lines[0].chars().collect::<Vec<char>>();

    let mut guide_map: HashMap<String, (String, String)> = HashMap::new();
    let mut starters: Vec<String> = vec![];

    lines.iter().skip(2).for_each(|x| {
        let l = x.split("=").collect::<Vec<&str>>();

        let key = l[0].trim();
        if key.ends_with('A') {
            starters.push(key.to_string());
        }

        let paths = l[1].trim().split(",").collect::<Vec<&str>>();
        let l = paths[0].trim().trim_start_matches("(");
        let r = paths[1].trim().trim_end_matches(")");

        guide_map.insert(key.to_string(), (l.to_string(), r.to_string()));
    });

    let iters = find(guide, guide_map, starters);

    iters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let result = process(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 10);
    }
}
