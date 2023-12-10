use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

fn check_above(
    map: &Vec<Vec<char>>,
    prev: (usize, usize),
    pos: (usize, usize),
) -> Option<(usize, usize)> {
    if pos.0 <= 0 {
        return None;
    }

    let valid_above = ['|', '7', 'F'];
    let next = (pos.0 - 1, pos.1);

    if valid_above.contains(&map[next.0][next.1]) {
        if prev != next {
            return Some(next);
        }
    }
    return None;
}

fn check_below(
    map: &Vec<Vec<char>>,
    prev: (usize, usize),
    pos: (usize, usize),
) -> Option<(usize, usize)> {
    if pos.0 >= map.len() - 1 {
        return None;
    }

    let valid_below = ['|', 'L', 'J'];
    let next = (pos.0 + 1, pos.1);

    if valid_below.contains(&map[next.0][next.1]) {
        if prev != next {
            return Some(next);
        }
    }
    return None;
}

fn check_left(
    map: &Vec<Vec<char>>,
    prev: (usize, usize),
    pos: (usize, usize),
) -> Option<(usize, usize)> {
    if pos.1 <= 0 {
        return None;
    }

    let valid_left = ['-', 'L', 'F'];
    let next = (pos.0, pos.1 - 1);

    if valid_left.contains(&map[next.0][next.1]) {
        if prev != next {
            return Some(next);
        }
    }
    return None;
}

fn check_right(
    map: &Vec<Vec<char>>,
    prev: (usize, usize),
    pos: (usize, usize),
) -> Option<(usize, usize)> {
    if pos.1 >= map[pos.0].len() - 1 {
        return None;
    }

    let valid_right = ['-', 'J', '7'];
    let next = (pos.0, pos.1 + 1);

    if valid_right.contains(&map[next.0][next.1]) {
        if prev != next {
            return Some(next);
        }
    }
    return None;
}

fn check_surrounding(
    map: &Vec<Vec<char>>,
    pos: (usize, usize),
    prev: (usize, usize),
) -> Option<(usize, usize)> {
    let c = map[pos.0][pos.1];

    match c {
        '|' => {
            if let Some(pos) = check_above(map, prev, pos) {
                return Some(pos);
            }

            if let Some(pos) = check_below(map, prev, pos) {
                return Some(pos);
            }
            return None;
        }
        '-' => {
            if let Some(pos) = check_right(map, prev, pos) {
                return Some(pos);
            }

            if let Some(pos) = check_left(map, prev, pos) {
                return Some(pos);
            }
            return None;
        }
        'L' => {
            if let Some(pos) = check_above(map, prev, pos) {
                return Some(pos);
            }

            if let Some(pos) = check_right(map, prev, pos) {
                return Some(pos);
            }
            return None;
        }
        'J' => {
            if let Some(pos) = check_above(map, prev, pos) {
                return Some(pos);
            }

            if let Some(pos) = check_left(map, prev, pos) {
                return Some(pos);
            }
            return None;
        }
        '7' => {
            if let Some(pos) = check_left(map, prev, pos) {
                return Some(pos);
            }

            if let Some(pos) = check_below(map, prev, pos) {
                return Some(pos);
            }
            return None;
        }
        'F' => {
            if let Some(pos) = check_right(map, prev, pos) {
                return Some(pos);
            }

            if let Some(pos) = check_below(map, prev, pos) {
                return Some(pos);
            }
            return None;
        }
        'S' => {
            if let Some(pos) = check_above(map, prev, pos) {
                return Some(pos);
            }
            if let Some(pos) = check_below(map, prev, pos) {
                return Some(pos);
            }
            if let Some(pos) = check_left(map, prev, pos) {
                return Some(pos);
            }
            if let Some(pos) = check_right(map, prev, pos) {
                return Some(pos);
            }
        }
        '.' => return None,

        _ => return None,
    };
    return None;
}

fn process(input: &str) -> i64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let mut start = (0, 0);

    let map = lines
        .iter()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, char)| {
                    if char == 'S' {
                        start = (row, col);
                    }
                    char
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut first = check_surrounding(&map, start, (0, 0));
    let mut prev_first = start;
    let mut second = check_surrounding(&map, start, first.unwrap());
    let mut prev_second = start;
    let mut iters = 1;

    while first != second {
        iters += 1;
        let first_new = check_surrounding(&map, first.unwrap(), prev_first);
        prev_first = first.unwrap();
        first = first_new;

        let second_new = check_surrounding(&map, second.unwrap(), prev_second);
        prev_second = second.unwrap();
        second = second_new;
    }

    iters
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
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_2() {
        let result = process(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, 8);
    }
}
