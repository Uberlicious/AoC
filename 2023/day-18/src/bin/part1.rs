use ansi_hex_color;
use std::{
    collections::{BTreeSet, VecDeque},
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {:?}", end - start, output);
}

#[derive(Debug, Clone)]
struct Step {
    dir: char,
    steps: u8,
    color: String,
}

impl Step {
    fn new(dir: char, steps: u8, color: String) -> Self {
        Step { dir, steps, color }
    }

    fn color(&self) -> String {
        let background = "#1e1e2e";
        let text = "#";
        ansi_hex_color::colored(self.color.as_str(), background, text)
    }
}

#[derive(Debug)]
struct Dig {
    steps: Vec<Step>,
    map: VecDeque<VecDeque<String>>,
    origin: (i32, i32),
    height: u32,
    width: u32,
}

impl Dig {
    fn parse(input: &str) -> Dig {
        let steps = input
            .lines()
            .map(|l| {
                let sections = l.trim().split_ascii_whitespace().collect::<Vec<_>>();
                let dir = sections[0].chars().nth(0).unwrap();
                let steps = sections[1].parse::<u8>().unwrap();
                let color = sections[2].trim_matches('(').trim_matches(')').to_string();

                Step::new(dir, steps, color)
            })
            .collect::<Vec<_>>();

        Dig {
            steps,
            map: VecDeque::new(),
            origin: (0, 0),
            height: 0,
            width: 0,
        }
    }

    fn draw_map(&self) {
        for (row, _) in self.map.iter().enumerate() {
            let row_string = self.map[row]
                .iter()
                .flat_map(|c| c.chars())
                .collect::<String>();
            println!("{row_string}");
        }
    }

    fn expand_map(&mut self, row: i32, col: i32) {
        if row < 0 {
            self.origin.0 += row.abs();
            self.height += row.abs() as u32;
        } else {
            self.height = self.height.max(row.abs() as u32);
        }

        if col < 0 {
            self.origin.1 += col.abs();
            self.width += col.abs() as u32;
        } else {
            self.width = self.width.max(col.abs() as u32);
        }

        if self.map.len() as u32 <= self.height {
            while self.map.len() as u32 <= self.height {
                if row < 0 {
                    self.map.push_front(VecDeque::new());
                } else {
                    self.map.push_back(VecDeque::new());
                }
            }
        }

        for r in 0..self.map.len() {
            if self.map[r].len() as u32 <= self.width {
                while self.map[r].len() as u32 <= self.width {
                    if col < 0 {
                        self.map[r].push_front(".".to_string());
                    } else {
                        self.map[r].push_back(".".to_string());
                    }
                }
            }
        }
    }

    fn blank_node(&self, color: Option<String>) -> String {
        let background = "#1e1e2e";
        let text = "#";

        if let Some(c) = color {
            return ansi_hex_color::colored(&c, background, text);
        }

        ansi_hex_color::colored("#FFFFFF", background, text)
    }

    fn make_map(&mut self) -> usize {
        let (mut row, mut col) = (0, 0);

        self.expand_map(row, col);
        self.map[0][0] = self.blank_node(Some("#000000".to_string()));

        for s in &self.steps.clone() {
            for _ in 0..s.steps {
                if s.dir == 'R' {
                    col += 1;
                }

                if s.dir == 'D' {
                    row += 1;
                }

                if s.dir == 'L' {
                    col -= 1;
                }

                if s.dir == 'U' {
                    row -= 1;
                }

                self.expand_map(row + self.origin.0, col + self.origin.1);
                self.map[(row + self.origin.0) as usize][(col + self.origin.1) as usize] =
                    s.color();
            }
        }

        self.fill();
        self.draw_map();
        self.calculate()
    }

    fn valid(&self, row: i32, col: i32) -> bool {
        if row <= 0
            || row >= self.height as i32
            || col <= 0
            || col >= self.width as i32
            || self.map[row as usize][col as usize] != "."
        {
            return false;
        }

        return true;
    }

    fn fill(&mut self) {
        let mut queue = BTreeSet::new();

        for row in 1..self.map.len() {
            for col in 0..self.map[row].len() {
                if self.map[row][col] != "."
                    && self.map[row][col + 1] == "."
                    && self.map[row - 1][col + 1] != "."
                {
                    queue.insert((row as i32, (col + 1) as i32));
                    break;
                }
            }
            if queue.len() > 0 {
                break;
            }
        }

        while queue.len() > 0 {
            let current = queue.iter().next().unwrap().clone();
            queue.remove(&current);

            if self.valid(current.0 + 1, current.1) {
                queue.insert((current.0 + 1, current.1));
            }

            if self.valid(current.0 - 1, current.1) {
                queue.insert((current.0 - 1, current.1));
            }

            if self.valid(current.0, current.1 + 1) {
                queue.insert((current.0, current.1 + 1));
            }

            if self.valid(current.0, current.1 - 1) {
                queue.insert((current.0, current.1 - 1));
            }

            self.map[current.0 as usize][current.1 as usize] = self.blank_node(None);
        }
    }

    fn calculate(&self) -> usize {
        self.map.iter().fold(0, |mut acc, row| {
            row.iter().for_each(|c| {
                if c != "." {
                    acc += 1;
                }
            });
            acc
        })
    }
}

fn process(input: &str) -> usize {
    Dig::parse(input).make_map()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)",
        );
        assert_eq!(result, 62)
    }

    // #[test]
    // fn test_part_1_2() {
    //     let result = process(
    //         "R 6 (#3a8da2)
    //         U 3 (#2f9f73)
    //         L 3 (#1fe640)
    //         U 3 (#450603)
    //         L 6 (#42b340)
    //         U 5 (#4a2f93)
    //         L 9 (#2141a2)",
    //     );
    //     assert_eq!(result, 62)
    // }
}
