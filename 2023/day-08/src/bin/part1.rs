use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

fn recurse(
    guide: Vec<char>,
    guide_map: HashMap<String, (String, String)>,
    mut search: String,
    mut iters: i32,
) -> i32 {
    for idx in 0..guide.len() {
        println!("dir: {}", guide[idx]);
        iters += 1;
        if guide[idx] == 'L' {
            search = guide_map[&search].0.clone();
        } else {
            search = guide_map[&search].1.clone();
        }

        if search == "ZZZ".to_string() {
            return iters;
        }
    }

    recurse(guide, guide_map, search, iters)
}

fn process(input: &str) -> i32 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let guide = lines[0].chars().collect::<Vec<char>>();

    let mut guide_map: HashMap<String, (String, String)> = HashMap::new();
    lines.iter().skip(2).for_each(|x| {
        let l = x.split("=").collect::<Vec<&str>>();

        let key = l[0].trim();

        let paths = l[1].trim().split(",").collect::<Vec<&str>>();
        let l = paths[0].trim().trim_start_matches("(");
        let r = paths[1].trim().trim_end_matches(")");

        guide_map.insert(key.to_string(), (l.to_string(), r.to_string()));
    });

    let iters = recurse(guide, guide_map, String::from("AAA"), 0);

    iters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 6);
    }
}
