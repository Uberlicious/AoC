use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input, 1000000);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

fn get_shortest_path(
    map: &Vec<Vec<char>>,
    rows: Vec<usize>,
    cols: Vec<usize>,
    mut multiplyer: usize,
) -> i64 {
    let mut galaxies = vec![];

    map.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, col)| {
            if *col == '#' {
                galaxies.push((row_idx, col_idx));
            }
        })
    });

    let mut all_pairs = vec![];

    if multiplyer != 1 {
        multiplyer = multiplyer - 1;
    }

    for i in 0..galaxies.len() {
        let mut new = galaxies[i].clone();
        let current = &mut galaxies[i];

        for r in rows.iter() {
            if current.0 > *r {
                new.0 += multiplyer;
            }
        }

        for c in cols.iter() {
            if current.1 > *c {
                new.1 += multiplyer;
            }
        }

        current.0 = new.0;
        current.1 = new.1;
    }

    for i in 0..galaxies.len() {
        let current = galaxies[i];

        for g in i + 1..galaxies.len() {
            let check = galaxies[g];

            let x = (check.0 as i64 - current.0 as i64).abs();
            let y = (check.1 as i64 - current.1 as i64).abs();
            let path_len = x + y;

            all_pairs.push(path_len);
        }
    }

    all_pairs.iter().sum()
}

fn expand(map: &Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<usize>, Vec<usize>) {
    let mut expanded: Vec<Vec<char>> = vec![];
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut boundary_rows = vec![];
    let mut boundary_cols = vec![];

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
                if !boundary_cols.contains(&col) {
                    boundary_cols.push(col);
                }
            }

            new_row.push(map[row][col]);
        }

        if !row_galaxy {
            boundary_rows.push(row);
        }

        expanded.push(new_row.clone());
    }

    (expanded, boundary_rows, boundary_cols)
}

fn process(input: &str, multiplyer: usize) -> i64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let map = lines
        .iter()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (map, rows, cols) = expand(&map);

    get_shortest_path(&map, rows, cols, multiplyer) as i64
}

#[allow(dead_code)]
const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(process(INPUT, 1), 374);
    }

    #[test]
    fn test2() {
        assert_eq!(process(INPUT, 10), 1030);
    }

    #[test]
    fn test3() {
        assert_eq!(process(INPUT, 100), 8410);
    }
}
