fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn check_number(map: &Vec<Vec<char>>, lidx: usize, idx: usize) -> u64 {
    let len = map[lidx].len();
    let mut l = idx.clone();
    let mut r = idx.clone();

    let lmin = idx - 1;
    let rmax = idx + 1;

    loop {
        println!("l: {}", map[lidx][l]);
        if l > 0 && (map[lidx][l - 1].is_ascii_digit() || l > lmin) {
            l -= 1;
        } else {
            break;
        }
    }

    loop {
        println!("ridx: {} r: {}", r, map[lidx][r]);
        if r < len - 1 && (map[lidx][r + 1].is_ascii_digit() || r < rmax) {
            r += 1;
        } else {
            break;
        }
    }

    let s = &map[lidx][l..=r];

    let numstr = map[lidx][l..=r]
        .iter()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();

    println!("num: {}", numstr);

    if numstr.len() > 0 {
        return numstr.parse::<u64>().expect("num");
    } else {
        return 0;
    }
}

fn check_surrounding(map: &Vec<Vec<char>>, line: usize, idx: usize) -> u64 {
    // .....
    // .123.
    // .....

    println!("surround - line {} idx: {}", line, idx);
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
        for (idx, top) in map[line - 1].iter().enumerate().skip(l) {
            println!("idx: {} top: {}", idx, top);
            if idx > r {
                break;
            }

            if top.is_ascii_digit() {
                holder.push(check_number(map, line - 1, idx));
                break;
            }
        }
        println!();
    }

    // check left
    if idx > 0 {
        let left = map[line][l];
        if left.is_ascii_digit() {
            holder.push(check_number(map, line, l));
        }
    }

    // check right
    if idx < map[line].len() {
        let right = map[line][r];
        if right.is_ascii_digit() {
            holder.push(check_number(map, line, r));
        }
    }

    // check below
    if line < map.len() - 1 {
        for (idx, bottom) in map[line - 1].iter().enumerate().skip(l) {
            println!("idx: {} bottom {}", idx, bottom);
            if idx > r {
                break;
            }

            if bottom.is_ascii_digit() {
                holder.push(check_number(map, line + 1, idx));
                break;
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
        println!("lidx: {} line: {:?}", lidx, line);

        for (idx, c) in line.iter().enumerate() {
            if *c != '*' {
                continue;
            }

            println!("idx: {} c: {}", idx, c);
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
