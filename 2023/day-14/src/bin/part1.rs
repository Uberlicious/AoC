use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

fn tilt_north(cols: &mut Vec<Vec<char>>) -> Vec<usize> {
    let mut weight: Vec<usize> = vec![0; cols[0].len()];

    for i in 0..cols.len() {
        for char in 0..cols[i].len() {
            if cols[i][char] == 'O' {
                let mut current = char;
                loop {
                    if current <= 0 || cols[i][current - 1] != '.' {
                        break;
                    }

                    cols[i][current - 1] = 'O';
                    cols[i][current] = '.';

                    current -= 1;
                }
                weight[current] += 1;
            }
        }
    }

    weight
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

    let weight = tilt_north(&mut cols);

    weight
        .iter()
        .rev()
        .enumerate()
        .map(|i| (i.0 + 1) * i.1)
        .sum::<usize>()
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
        assert_eq!(result, 136);
    }
}
