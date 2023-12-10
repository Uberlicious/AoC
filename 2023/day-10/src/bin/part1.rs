use std::time::{SystemTime, UNIX_EPOCH};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

#[derive(Debug)]
enum Node {
    True,
    False,
    Start,
}

enum Dir {
    Above,
    Below,
    Left,
    Right,
}

#[derive(Debug)]
struct Tile {
    char: char,
    pos: (usize, usize),
    next: Option<(usize, usize)>,
    prev: Option<(usize, usize)>,
}

impl Tile {
    fn new(char: char, pos: (usize, usize)) -> Tile {
        Tile {
            char,
            pos,
            next: None,
            prev: None,
        }
    }

    fn set_next(&mut self, next: (usize, usize)) {
        self.next = Some(next);
    }

    fn set_prev(&mut self, prev: (usize, usize)) {
        self.prev = Some(prev);
    }

    fn next(&self) -> Option<(usize, usize)> {
        self.next
    }
}

struct PipeMap {
    start: (u32, u32),
}

fn check_surrounding(cur: &mut Vec<Vec<Tile>>, pos: (usize, usize)) -> bool {
    let row = pos.0;
    let col = pos.1;
    let above = row > 0;
    let below = row < map.len() - 1;
    let left = col > 0;
    let right = col < map[row].len() - 1;
    let valid_above = ['|', '7', 'F'];
    let valid_below = ['|', 'L', 'J'];
    let valid_left = ['-', 'L', 'F'];
    let valid_right = ['-', 'J', '7'];
    let c = map[row][col];

    match c {
        '|' => {
            if above && valid_above.contains(&map[row - 1][col]) {
                if cur.prev.unwrap().0 != row - 1 && cur.prev.unwrap().1 != col {
                    cur.set_next((row - 1, col));
                    if 
                }
            }

            if below && valid_above.contains(&map[row + 1][col]) {
                return (Node::False, None);
            }
            return Node::True;
        }
        '-' => {
            if right && !valid_right.contains(&map[row][col + 1]) {
                return (Node::False, None);
            }

            if left && !valid_left.contains(&map[row][col - 1]) {
                return (Node::False, None);
            }
            return Node::True;
        }
        'L' => {
            if above && !valid_above.contains(&map[row - 1][col]) {
                return (Node::False, None);
            }

            if right && !valid_right.contains(&map[row][col + 1]) {
                return (Node::False, None);
            }
            return Node::True;
        }
        'J' => {
            if above && !valid_above.contains(&map[row - 1][col]) {
                return (Node::False, None);
            }

            if left && !valid_left.contains(&map[row][col - 1]) {
                return (Node::False, None);
            }
            return Node::True;
        }
        '7' => {
            if left && !valid_left.contains(&map[row][col - 1]) {
                return (Node::False, None);
            }

            if below && !valid_above.contains(&map[row + 1][col]) {
                return (Node::False, None);
            }
            return Node::True;
        }
        'F' => {
            if right && !valid_right.contains(&map[row][col + 1]) {
                return (Node::False, None);
            }

            if below && !valid_above.contains(&map[row + 1][col]) {
                return (Node::False, None);
            }
            return Node::True;
        }
        '.' => return (Node::False, None),
        'S' => return Node::Start,

        _ => return (Node::False, None),
    };
}

fn process(input: &str) -> i64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let map = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // let mut tile_map: Vec<Vec<Tile>> = Vec::with;
    let mut start = (0, 0);

    let mut tile_map = map
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, col)| {
                    if *col == 'S' {
                        start = (row_idx, col_idx);
                    }
                    Tile::new(*col, (row_idx, col_idx))
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    println!("start: {:?}", start);
    check_surrounding(&mut tile_map, start);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 114);
    }
}
