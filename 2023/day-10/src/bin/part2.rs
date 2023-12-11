use std::time::{SystemTime, UNIX_EPOCH};

#[allow(unused_variables, dead_code)]
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

#[derive(Debug, PartialEq)]
enum HoleDir {
    Top,
    Bottom,
    Left,
    Right,
}

fn search_vert_hole(
    map: &Vec<Vec<char>>,
    top: (usize, usize),
    offset: HoleDir,
) -> Option<Vec<(usize, usize)>> {
    let mut holder: Vec<(usize, usize)> = vec![];

    let vert_left_wall = ['|', '7', 'J'];
    let vert_right_wall = ['|', 'F', 'L'];

    let left_pos = (top.0, top.1 - 1);
    let right_pos = (top.0, top.1 + 1);

    let mut top_left_res: Option<(bool, usize, usize)> = None;
    let mut top_right_res: Option<(bool, usize, usize)> = None;
    let mut top = top;

    let top_left_char = map[left_pos.0][left_pos.1];
    let top_right_char = map[right_pos.0][right_pos.1];
    let top_char = map[top.0][top.1];

    let mut left = false;
    let mut right = false;

    if vert_left_wall.contains(&top_left_char) && vert_right_wall.contains(&top_char) {
        left = true;
    }

    if vert_left_wall.contains(&top_char) && vert_right_wall.contains(&top_right_char) {
        right = true;
    }

    if !right && !left {
        return None;
    }

    println!("right: {right} left: {left}");
    let mut idx = 0;

    loop {
        if idx >= 2 {
            break;
        }
        let top_left_pos;
        let top_right_pos;

        if left {
            top_left_pos = (top.0, top.1 - 1);
            top_right_pos = (top.0, top.1);
        } else {
            top_left_pos = (top.0, top.1);
            top_right_pos = (top.0, top.1 + 1);
        }

        idx += 1;
        if top_left_res == None && top_right_res == None {
            let top_left_char = map[top_left_pos.0][top_left_pos.1];
            let top_right_char = map[top_right_pos.0][top_right_pos.1];
            println!("tlp: {top_left_pos:?} tlc: {top_left_char:?} trp: {top_right_pos:?} trc: {top_right_char:?}");

            if !vert_left_wall.contains(&top_left_char)
                || !vert_right_wall.contains(&top_right_char)
            {
                let mut tl_valid = false;
                if top_left_char == '.' {
                    tl_valid = true;
                }

                top_left_res = Some((tl_valid, left_pos.0, left_pos.1));

                let mut tr_valid = false;
                if top_left_char == '.' {
                    tr_valid = true;
                }

                top_right_res = Some((tr_valid, top.0, top.1));
            }
        }

        println!("tlr: {top_left_res:?} trr: {top_right_res:?}");

        if top_left_res != None && top_right_res == None {
            break;
        }

        if offset == HoleDir::Top && top.0 > 0 {
            top = (top.0 - 1, top.1);
        } else if top.0 < map[0].len() - 1 {
            top = (top.0 + 1, top.1);
        } else {
            break;
        }
    }

    if let Some(tl) = top_left_res {
        if tl.0 {
            holder.push((top_left_res.unwrap().1, top_left_res.unwrap().2));
        }
    }

    if let Some(tr) = top_right_res {
        if tr.0 {
            holder.push((top_right_res.unwrap().1, top_right_res.unwrap().2));
        }
    }

    if holder.len() > 0 {
        return Some(holder);
    }

    None
}

fn fan_out(
    map: &Vec<Vec<char>>,
    pipes: &Vec<(usize, usize)>,
    searching: Vec<(usize, usize)>,
    searched: &mut Vec<Vec<(bool, bool)>>,
    mut chain: Vec<(usize, usize)>,
    mut chain_state: bool,
) {
    let vert_left_wall = ['|', '7', 'J'];
    let vert_right_wall = ['|', 'F', 'L'];

    let horiz_top_wall = ['L', 'J', '-'];
    let horiz_bottom_wall = ['7', 'F', '-'];

    let mut next_search: Vec<(usize, usize)> = vec![];

    for s in searching.iter() {
        let top = s.0 > 0;
        let bottom = s.0 < map.len() - 1;
        let left = s.1 > 0;
        let right = s.1 < map[0].len() - 1;

        if top {
            let top = (s.0 - 1, s.1);
            let top_char = map[top.0][top.1];
            if top_char == '.' && !searched[top.0][top.1].0 {
                next_search.push(top)
            }

            if pipes.contains(&top)
                && (vert_left_wall.contains(&top_char) || vert_right_wall.contains(&top_char))
            {
                if let Some(top_hole) = search_vert_hole(map, top, HoleDir::Top) {
                    println!("top_hole: {top_hole:?}");
                }
            }
        } else {
            chain_state = false;
        }

        if bottom {
            let bottom = (s.0 + 1, s.1);
            let bottom_char = map[bottom.0][bottom.1];
            if bottom_char == '.' && !searched[bottom.0][bottom.1].0 {
                next_search.push(bottom)
            }
        } else {
            chain_state = false;
        }

        if left {
            let left = (s.0, s.1 - 1);
            let left_char = map[left.0][left.1];
            if left_char == '.' && !searched[left.0][left.1].0 {
                next_search.push(left)
            }
        } else {
            chain_state = false;
        }

        if right {
            let right = (s.0, s.1 + 1);
            let right_char = map[right.0][right.1];
            if right_char == '.' && !searched[right.0][right.1].0 {
                next_search.push(right)
            }
        } else {
            chain_state = false;
        }

        searched[s.0][s.1].0 = true;
        chain.push((s.0, s.1));
    }

    if next_search.len() > 0 {
        fan_out(
            map,
            pipes,
            next_search,
            searched,
            chain.clone(),
            chain_state,
        );
    }

    chain
        .iter()
        .for_each(|x| searched[x.0][x.1].1 = chain_state);
}

fn find_nests(
    map: &Vec<Vec<char>>,
    pipes: &Vec<(usize, usize)>,
    searched: &mut Vec<Vec<(bool, bool)>>,
) {
    for (row, vec) in map.iter().enumerate() {
        for (col, char) in vec.iter().enumerate() {
            if *char != '.' {
                continue;
            }

            if searched[row][col].0 {
                continue;
            }

            fan_out(&map, &pipes, vec![(row, col)], searched, vec![], true);
        }
    }
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
    pipes.push(start);
    pipes.push(first.unwrap());
    pipes.push(second.unwrap());

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

    let mut searched: Vec<Vec<(bool, bool)>> = vec![vec![(false, false); map[0].len()]; map.len()];
    find_nests(&map, &pipes, &mut searched);

    searched.iter().enumerate().for_each(|(row_idx, row)| {
        let row = row
            .iter()
            .enumerate()
            .map(|(col_idx, col)| {
                if pipes.contains(&(row_idx, col_idx)) || !col.0 {
                    return map[row_idx][col_idx];
                }

                if col.1 {
                    return 'I';
                } else {
                    return 'O';
                }
            })
            .collect::<String>();
        println!("{row:?}");
    });

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

    // #[test]
    // fn test_part_2() {
    //     let result = process(
    //         ".F----7F7F7F7F-7....
    //         .|F--7||||||||FJ....
    //         .||.FJ||||||||L7....
    //         FJL7L7LJLJ||LJ.L-7..
    //         L--J.L7...LJS7F-7L7.
    //         ....F-J..F7FJ|L7L7L7
    //         ....L7.F7||L7|.L7L7|
    //         .....|FJLJ|FJ|F7|.LJ
    //         ....FJL-7.||.||||...
    //         ....L---J.LJ.LJLJ...",
    //     );
    //     assert_eq!(result, 8);
    // }

    // #[test]
    // fn test_part_3() {
    //     let result = process(
    //         "FF7FSF7F7F7F7F7F---7
    //         L|LJ||||||||||||F--J
    //         FL-7LJLJ||||||LJL-77
    //         F--JF--7||LJLJ7F7FJ-
    //         L---JF-JLJ.||-FJLJJ7
    //         |F|F-JF---7F7-L7L|7|
    //         |FFJF7L7F-JF7|JL---7
    //         7-L-JL7||F7|L7F-7F7|
    //         L.L7LFJ|||||FJL7||LJ
    //         L7JLJL-JLJLJL--JLJ.L",
    //     );
    //     assert_eq!(result, 10);
    // }
}
