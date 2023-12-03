fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");
    let lines = input.split("\n").collect::<Vec<_>>();

    let mut cols: Vec<u32> = vec![];

    for l in lines {
        let mut totals = (0, 0, 0);

        let game_split = l.split(":").collect::<Vec<_>>();

        let bags = game_split[1].split(";").collect::<Vec<_>>();

        for b in bags {
            let colors = b.split(",").collect::<Vec<_>>();
            for c in colors {
                let color = c.trim().split(" ").collect::<Vec<_>>();
                match color[1] {
                    "red" => {
                        let c = color[0].parse::<u32>().expect("NaN");
                        if c > totals.0 {
                            totals.0 = c
                        }
                    }
                    "green" => {
                        let c = color[0].parse::<u32>().expect("NaN");
                        if c > totals.1 {
                            totals.1 = c
                        }
                    }
                    "blue" => {
                        let c = color[0].parse::<u32>().expect("NaN");
                        if c > totals.2 {
                            totals.2 = c
                        }
                    }
                    _ => {}
                }
            }
        }

        cols.push(totals.0 * totals.1 * totals.2);
    }

    cols.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
