fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn check_surrounding(map: &Vec<Vec<char>>, line: usize, s: usize, e: usize, number: &u32) -> u32 {
    // .....
    // .123.
    // .....

    let mut matches = 0;

    let mut l = s;
    let mut r = e;

    if s > 0 {
        l = s - 1;
    }

    if e < map[line].len() {
        r = e + 1;
    }

    print!("top: ");
    // check above
    if line > 0 {
        for top in &map[line - 1][l..=r] {
            print!("{}", top);
            if *top != '.' {
                matches += 1;
            }
        }
        println!();
    }

    // check left
    print!("mid: ");
    if s > 0 {
        let left = map[line][l];
        print!("{left}");
        if left != '.' {
            matches += 1;
        }
    }

    print!("{number}");
    // check right
    if e < map[line].len() {
        let right = map[line][r];
        println!("{right}");
        if right != '.' {
            matches += 1;
        }
    }

    print!("bot: ");
    // check below
    if line < map.len() - 1 {
        for bottom in &map[line + 1][l..=r] {
            print!("{}", bottom);
            if *bottom != '.' {
                matches += 1;
            }
        }
        println!()
    }

    matches
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
        println!("lidx: {} line: {:?}", lidx, line);
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

            println!();
            let matches = check_surrounding(&lines, lidx, l, r, &number);

            println!("\nnumber: {} add: {}", number, matches);

            holder.push(*number * matches);

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
