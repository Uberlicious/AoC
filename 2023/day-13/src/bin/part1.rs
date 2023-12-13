use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

#[derive(Debug, Clone)]
struct Map {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Map {
    fn new(map: &str) -> Self {
        let width = map.lines().next().unwrap().len();
        let height = map.lines().count();

        let mut rows = vec![0; height];
        let mut cols = vec![0; width];
        let mut bytes = map.bytes();

        let (mut x, mut y) = (0, 0);
        while let Some(byte) = bytes.next() {
            match byte {
                b'.' => x += 1,
                b'#' => {
                    rows[y] |= 0b1 << x;
                    cols[x] |= 0b1 << y;
                    x += 1;
                }
                b'\n' => {
                    x = 0;
                    y += 1;
                }
                _ => {}
            }
        }

        Map { rows, cols }
    }
}

fn is_mirror(left: &[u32], right: &[u32]) -> bool {
    let mut l = left.iter().rev();
    let mut r = right.iter();

    while let Some(a) = l.next() {
        if let Some(b) = r.next() {
            if a != b {
                return false;
            }
        } else {
            break;
        }
    }

    true
}

fn process(input: &str) -> usize {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n\n").collect::<Vec<_>>();

    let mut maps: Vec<Map> = vec![];
    maps.extend(lines.iter().map(|&m| Map::new(m)));

    let mut col_matches = 0;
    let mut row_matches = 0;
    for m in maps {
        let cols = m.cols.len();
        for i in 1..cols {
            if is_mirror(&m.cols[0..i], &m.cols[i..]) {
                col_matches += i;
            }
        }

        let rows = m.rows.len();
        for i in 1..rows {
            if is_mirror(&m.rows[0..i], &m.rows[i..]) {
                row_matches += i;
            }
        }
    }

    col_matches + (row_matches * 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, 405);
    }
}
