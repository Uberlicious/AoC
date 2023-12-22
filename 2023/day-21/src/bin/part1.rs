use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let input = include_str!("./input1.txt");

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let output = part1(input, 64);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 1 - Time: {:?} Output {:?}", end - start, output);

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let output = part2(input, 26501365);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 2 - Time: {:?} Output {:?}", end - start, output);
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum MapExpand {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Garden {
    map: Box<[u8]>,
    // (x, y)
    // start: (usize, usize),
    steps: usize,
    offset: usize,
    garden: Vec<usize>,
}

impl Garden {
    fn parse(input: &str, steps: usize) -> Self {
        let mut lines = input.lines().peekable();
        let offset = lines.peek().map_or(0, |l| l.len());

        Garden {
            map: lines
                .flat_map(str::as_bytes)
                .map(|&c| c)
                .collect::<Box<_>>(),
            steps,
            offset,
            garden: Vec::new(),
        }
    }

    fn next_pos(
        &self,
        p: (usize, BTreeSet<MapExpand>),
        dir: u8,
    ) -> Option<(usize, BTreeSet<MapExpand>)> {
        // 0 = up, 1 = right, 2 = down, 3 = left
        Some(match dir {
            0 => {
                if p.0 < self.offset {
                    let mut m = p.1.clone();
                    if !m.contains(&MapExpand::Down) {
                        m.insert(MapExpand::Up);
                    }
                    println!("UP: {} => {}", p.0, self.map.len() - (self.offset - p.0));
                    return Some((self.map.len() - (self.offset - p.0), m));
                }

                (p.0 - self.offset, p.1)
            }
            1 => {
                if (p.0 + 1) % self.offset == 0 {
                    let mut m = p.1.clone();
                    if !m.contains(&MapExpand::Left) {
                        m.insert(MapExpand::Right);
                    }
                    println!("Right: {} => {}", p.0, p.0 - self.offset + 1);
                    return Some((p.0 - self.offset, m));
                }

                (p.0 + 1, p.1)
            }
            2 => {
                if p.0 >= self.map.len() - self.offset {
                    let mut m = p.1.clone();
                    if !m.contains(&MapExpand::Up) {
                        m.insert(MapExpand::Down);
                    }
                    println!("Down: {} => {}", p.0, self.offset - (self.map.len() - p.0));
                    return Some((self.map.len() - p.0, m));
                }

                (p.0 + self.offset, p.1)
            }
            3 => {
                if p.0 % self.offset == 0 {
                    let mut m = p.1.clone();
                    if !m.contains(&MapExpand::Right) {
                        m.insert(MapExpand::Left);
                    }
                    println!("Left: {} => {}", p.0, p.0 + self.offset - 1);
                    return Some((p.0 + self.offset - 1, m));
                }

                (p.0 - 1, p.1)
            }
            _ => return None,
        })
    }

    fn step(&mut self, start: usize) -> usize {
        let mut queue: BTreeSet<(usize, BTreeSet<MapExpand>)> = BTreeSet::new();
        queue.insert((start, BTreeSet::new()));
        let mut step = self.steps;

        let output = loop {
            step -= 1;
            let mut next: BTreeSet<(usize, BTreeSet<MapExpand>)> = BTreeSet::new();
            while queue.len() > 0 {
                let current = queue.pop_first().unwrap();
                for dir in 0..4 {
                    if let Some(plot) = self.next_pos(current.clone(), dir) {
                        if self.map[plot.0] == b'.' || self.map[plot.0] == b'S' {
                            next.insert(plot);
                        }
                    }
                }
            }

            if step == 0 {
                for i in &next {
                    self.garden.push(i.0);
                }
                break next;
            }

            queue = next;
        };

        println!("output: {output:?} len: {}", output.len());

        output.iter().len()
    }

    fn walk(&mut self) -> usize {
        let mut start = 0;

        for i in self.map.iter().enumerate() {
            if *i.1 == b'S' {
                start = i.0;
            }
        }

        self.step(start)
    }
}

impl Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Steps: {} Offset: {}\n", self.steps, self.offset)?;
        write!(f, "Map:")?;
        for (i, b) in self.map.iter().enumerate() {
            if i % self.offset == 0 {
                write!(f, "\n")?;
            }

            if self.garden.contains(&i) {
                write!(f, "O")?;
            } else {
                write!(f, "{}", *b as char)?;
            }
        }

        write!(f, "")
    }
}

fn part1(input: &str, steps: usize) -> usize {
    Garden::parse(input, steps).walk()
}

fn part2(input: &str, steps: usize) -> usize {
    let mut g = Garden::parse(input, steps);
    let walk = g.walk();
    println!("{g}");
    walk
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let result = part1(
            r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
            6,
        );
        assert_eq!(result, 16);
    }

    #[test]
    fn part_2() {
        let map = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part2(map, 10), 50);
        assert_eq!(part2(map, 50), 1594);
    }
}
