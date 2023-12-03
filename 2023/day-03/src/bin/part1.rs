fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn check_surrounding(map: &Vec<Vec<char>>, line: usize, s: usize, e: usize) -> bool {
    // .....
    // .123.
    // .....

    let mut left = s;
    let mut right = e;

    if s > 0 {
        left = s - 1;
    }

    if e < map[line].len() {
        right = s + 1;
    }

    // check above
    if line > 0 {
        for top in &map[line - 1][left..=right] {
            if !top.is_ascii_digit() && *top != '.' {
                return true;
            }
        }
    }

    if line < map.len() - 1 {
        for bottom in &map[line + 1][left..=right] {
            if !bottom.is_ascii_digit() && *bottom != '.' {
                return true;
            }
        }
    }

    // check left
    if s > 0 {
        let left = map[line][left];
        if !left.is_ascii_digit() && left != '.' {
            return true;
        }
    }

    // check right
    if e < map[line].len() - 1 {
        let right = map[line][right];
        if !right.is_ascii_digit() && right != '.' {
            return true;
        }
    }

    false
}

fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");
    let lines = input.split("\n").collect::<Vec<_>>();
    let mut holder: Vec<u32> = vec![];

    let lines = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (lidx, line) in lines.iter().enumerate() {
        let mut start_idx: Option<usize> = None;
        let mut end_idx: Option<usize> = None;

        for (idx, c) in line.iter().enumerate() {
            if c.is_ascii_digit() && start_idx.is_none() {
                start_idx = Some(idx);
            }

            if c.is_ascii_digit() {
                continue;
            }

            if start_idx.is_none() && end_idx.is_none() {
                continue;
            }

            if start_idx.is_some() && end_idx.is_none() {
                end_idx = Some(idx - 1);
            }

            let l = start_idx.ok_or("no start").unwrap();
            let r = end_idx.ok_or("no end").unwrap();

            let number = &line[l..=r]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            // let number: u32 = num
            //     .iter()
            //     .map(|c| c.to_string().parse::<u32>().expect("number"))
            //     .collect::<Vec<u32>>()
            //     .iter()
            //     .sum();

            println!("number: {}", number);
            if check_surrounding(&lines, lidx, l, r) {
                holder.push(*number);
            }

            start_idx = None;
            end_idx = None;
        }
    }

    holder.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361);
    }
}
