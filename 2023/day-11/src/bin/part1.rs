use std::time::{SystemTime, UNIX_EPOCH};

use num::{integer::Roots, traits::AsPrimitive};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

fn get_shortest_path(map: &Vec<Vec<char>>) -> i32 {
    let mut galaxies = vec![];

    map.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, col)| {
            if *col == '#' {
                galaxies.push((row_idx, col_idx));
            }
        })
    });

    let mut shortest_path = vec![i32::MAX; galaxies.len() - 1];
    let mut all_pairs = vec![];

    for i in 0..galaxies.len() {
        let current = galaxies[i];

        for g in i + 1..galaxies.len() {
            let check = galaxies[g];

            let x = (check.0 as i32 - current.0 as i32).abs();
            let y = (check.1 as i32 - current.1 as i32).abs();
            let path_len = x + y;

            if path_len < shortest_path[i].abs() {
                shortest_path[i] = path_len;
            }

            all_pairs.push(path_len);
        }
    }

    all_pairs.iter().sum()
}

fn expand(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded: Vec<Vec<char>> = vec![];
    let mut galaxies: Vec<(usize, usize)> = vec![];

    map.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, col)| {
            if *col == '#' {
                galaxies.push((row_idx, col_idx));
            }
        })
    });

    for row in 0..map.len() {
        let mut new_row: Vec<char> = vec![];
        let row_galaxy = galaxies.iter().any(|x| x.0 == row);
        for col in 0..map[row].len() {
            let col_galaxy = galaxies.iter().any(|y| y.1 == col);
            if !col_galaxy {
                new_row.push(map[row][col]);
            }

            new_row.push(map[row][col]);
        }

        if !row_galaxy {
            expanded.push(new_row.clone());
        }

        expanded.push(new_row.clone());
    }

    expanded
}

fn process(input: &str) -> i64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let map = lines
        .iter()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let map = expand(&map);

    get_shortest_path(&map) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....",
        );
        assert_eq!(result, 374);
    }
}
