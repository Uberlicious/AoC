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

struct Nest {
    pos: (usize, usize),
    top_char: Option<char>,
    bottom_char: Option<char>,
    left_char: Option<char>,
    right_char: Option<char>,
}

fn search_top_hole(
    map: &Vec<Vec<char>>,
    left: (usize, usize),
    top: (usize, usize),
    right: (usize, usize),
    searched: &mut Vec<(usize, usize, bool)>,
) -> bool {
    let vert_left_wall = ['|', '7', 'J'];
    let vert_right_wall = ['|', 'F', 'L'];

    if vert_left_wall.contains(&map[top.0][top.1])
        && vert_right_wall.contains(&map[right.0][right.1])
    {
        if top.0 == 0 {
            return true;
        }

        return search_top_hole(
            map,
            left,
            (top.0 - 1, top.1),
            (right.0 - 1, right.1),
            searched,
        );
    }

    if vert_right_wall.contains(&map[top.0][top.1]) && vert_left_wall.contains(&map[left.0][left.1])
    {
        if top.0 == 0 {
            return true;
        }

        return search_top_hole(
            map,
            (left.0 - 1, left.1),
            (top.0 - 1, top.1),
            right,
            searched,
        );
    }

    return true;
}

fn find_nests(
    map: &Vec<Vec<char>>,
    pipes: &Vec<(usize, usize)>,
    current: (usize, usize),
    mut searched: Vec<Vec<bool>>,
) -> bool {
    let walls = ['|', '-', 'L', 'J', '7', 'F', 'S'];
    let vert_left_wall = ['|', '7', 'J'];
    let vert_right_wall = ['|', 'F', 'L'];

    let horiz_top_wall = ['L', 'J', '-'];
    let horiz_bottom_wall = ['7', 'F', '-'];

    // if against outer wall return false
    if current.0 == 0 {
        searched[current.0][current.1] = true;
        return false;
    }

    if current.0 == map.len() - 1 {
        searched[current.0][current.1] = true;
        return false;
    }

    if current.1 == 0 {
        searched[current.0][current.1] = true;
        return false;
    }

    if current.1 == map.len() - 1 {
        searched[current.0][current.1] = true;
        return false;
    }

    let top = (current.0 - 1, current.1);
    if pipes.contains(&top) {
        let top_char = map[top.0][top.1];
        if vert_left_wall.contains(&top_char) && vert_right_wall.contains(&map[top.0][top.1 + 1]) {
            search_top_hole(
                map,
                (top.0, top.1 - 1),
                top,
                (top.0, top.1 + 1),
                &mut searched,
            );
        }
    }

    false
}

fn process(input: &str) -> i64 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").map(|l| l.trim()).collect::<Vec<_>>();
    let mut trapped = 0;

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
    let mut pipes: Vec<(usize, usize)> = vec![];

    loop {
        let first_new = check_surrounding(&map, first.unwrap(), prev_first);
        prev_first = first.unwrap();
        first = first_new;
        pipes.push(first.unwrap());

        let second_new = check_surrounding(&map, second.unwrap(), prev_second);
        prev_second = second.unwrap();
        second = second_new;

        if first == second {
            break;
        }

        pipes.push(second.unwrap());
    }

    find_nests(&map, &pipes, (0, 0), vec![]);

    println!("pipes: {pipes:?}");

    trapped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_1_2() {
        let result = process(
            "..........
            .S------7.
            .|F----7|.
            .||....||.
            .||....||.
            .|L-7F-J|.
            .|..||..|.
            .L--JL--J.
            ..........",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_2() {
        let result = process(
            ".F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_3() {
        let result = process(
            "FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, 10);
    }
}
