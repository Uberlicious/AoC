fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn check_number(line: &Vec<char>, idx: usize) -> u64 {
    let mut l = idx.clone();
    let mut r = idx.clone();

    loop {
        // check left
        if l > 0 && (line[l - 1].is_ascii_digit() || l < l - 1) {
            l -= 1;
            continue;
        }

        // check right
        if r < line.len() - 1 && (line[r + 1].is_ascii_digit() || r > r + 1) {
            r += 1;
            continue;
        }

        break;
    }

    line[l..=r]
        .iter()
        .collect::<String>()
        .parse::<u64>()
        .expect("number")
}

fn check_surrounding(map: &Vec<Vec<char>>, line: usize, idx: usize) -> u64 {
    // .....
    // .123.
    // .....

    let mut holder: Vec<u64> = vec![];
    let mut l = idx.clone();
    let mut r = idx.clone();

    if idx > 0 {
        l = idx - 1;
    }

    if idx < map[line].len() {
        r = idx + 1;
    }

    // check above
    if line > 0 {
        for (idx, top) in map[line - 1].iter().enumerate() {
            if idx < l || idx > r {
                continue;
            }

            if top.is_ascii_digit() && (map[line - 1][idx - 1] == '.' || idx == l) {
                holder.push(check_number(&map[line - 1], idx));
            }
        }
    }

    // check left
    if idx > 0 {
        let left = map[line][l];
        if left.is_ascii_digit() {
            holder.push(check_number(&map[line], l));
        }
    }

    // check right
    if idx < map[line].len() {
        let right = map[line][r];
        if right.is_ascii_digit() {
            holder.push(check_number(&map[line], r));
        }
    }

    // check below
    if line < map.len() - 1 {
        for (idx, bottom) in map[line + 1].iter().enumerate() {
            if idx < l || idx > r {
                continue;
            }

            if bottom.is_ascii_digit() && (map[line + 1][idx - 1] == '.' || idx == l) {
                holder.push(check_number(&map[line + 1], idx));
            }
        }
    }

    if holder.len() < 2 {
        return 0;
    }

    holder.iter().product()
}

fn process(input: &str) -> u64 {
    let input = input.replace("\r\n", "\n");
    let lines = input.split("\n").collect::<Vec<_>>();

    let mut holder: Vec<u64> = vec![];

    let lines = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (lidx, line) in lines.iter().enumerate() {
        for (idx, c) in line.iter().enumerate() {
            if *c != '*' {
                continue;
            }

            holder.push(check_surrounding(&lines, lidx, idx));
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
        assert_eq!(result, 467835);
    }
}
