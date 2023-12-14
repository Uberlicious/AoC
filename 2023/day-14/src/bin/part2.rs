use std::{
    collections::{hash_map::Entry::*, HashMap},
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

#[derive(Debug, PartialEq)]
enum Tilt {
    North,
    South,
    East,
    West,
}

fn tilt(cols: &mut Vec<Vec<char>>, tilt: Tilt) -> Vec<usize> {
    let mut weight: Vec<usize> = vec![0; cols[0].len()];
    let mut offset: i32 = 1;

    if tilt == Tilt::North || tilt == Tilt::West {
        offset = -1;
    }

    if tilt == Tilt::North || tilt == Tilt::West {
        for i in 0..cols.len() {
            for char in 0..cols[i].len() {
                if cols[i][char] == 'O' {
                    move_rock(&tilt, char, i, cols, offset);
                }
            }
        }
    }

    if tilt == Tilt::East {
        for i in (0..cols.len()).rev() {
            for char in 0..cols[i].len() {
                if cols[i][char] == 'O' {
                    move_rock(&tilt, char, i, cols, offset);
                }
            }
        }
    }

    if tilt == Tilt::South {
        for i in 0..cols.len() {
            for char in (0..cols[0].len()).rev() {
                if cols[i][char] == 'O' {
                    move_rock(&tilt, char, i, cols, offset);
                }
            }
        }
    }

    weight
}

fn move_rock(tilt: &Tilt, char: usize, i: usize, cols: &mut Vec<Vec<char>>, offset: i32) {
    let mut current;
    if *tilt == Tilt::North || *tilt == Tilt::South {
        current = char;
    } else {
        current = i;
    }

    loop {
        if *tilt == Tilt::North
            && (current <= 0 || cols[i][(current as i32 + offset) as usize] != '.')
        {
            break;
        }

        if *tilt == Tilt::South
            && (current >= cols[i].len() - 1 || cols[i][(current as i32 + offset) as usize] != '.')
        {
            break;
        }

        if *tilt == Tilt::East
            && (current >= cols.len() - 1 || cols[(current as i32 + offset) as usize][char] != '.')
        {
            break;
        }

        if *tilt == Tilt::West
            && (current <= 0 || cols[(current as i32 + offset) as usize][char] != '.')
        {
            break;
        }

        if *tilt == Tilt::North || *tilt == Tilt::South {
            cols[i][(current as i32 + offset) as usize] = 'O';
            cols[i][current] = '.';
        } else {
            cols[(current as i32 + offset) as usize][char] = 'O';
            cols[current][char] = '.';
        }

        if *tilt == Tilt::North || *tilt == Tilt::West {
            current -= 1;
        } else {
            current += 1;
        }
    }
}

fn spin(cols: &mut Vec<Vec<char>>, cycles: usize) -> usize {
    let mut visited = HashMap::new();

    for i in 1..=cycles {
        cycle(cols);
        match visited.entry(cols.clone()) {
            Occupied(e) => {
                let cycle_len = i - e.get();
                let remaining = (cycles - i) % cycle_len;
                for _ in 0..remaining {
                    cycle(cols)
                }
                break;
            }
            Vacant(e) => {
                e.insert(i);
            }
        }
    }

    let mut total = vec![];
    for row in (0..cols[0].len()).rev().enumerate() {
        let mut rocks = 0;
        for col in 0..cols.len() {
            if cols[col][row.1] == 'O' {
                rocks += 1
            }
        }
        total.push(rocks);
    }

    total
        .iter()
        .enumerate()
        .map(|n| (n.0 + 1) * n.1)
        .sum::<usize>()
}

fn cycle(cols: &mut Vec<Vec<char>>) {
    tilt(cols, Tilt::North);
    tilt(cols, Tilt::West);
    tilt(cols, Tilt::South);
    tilt(cols, Tilt::East);
}

fn process(input: &str) -> usize {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let mut cols: Vec<Vec<char>> = vec![vec![]; lines.len()];
    for i in 0..lines[0].len() {
        for row in lines.iter().enumerate() {
            cols[i].push(row.1.trim().chars().nth(i).unwrap());
        }
    }

    let weight = spin(&mut cols, 1000000000);

    weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....",
        );
        assert_eq!(result, 64);
    }
}
