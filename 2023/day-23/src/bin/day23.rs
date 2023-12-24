use std::{
    collections::BTreeSet,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let input = include_str!("./input1.txt");

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let output = part1(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 1 - Time: {:?} Output {:?}", end - start, output);

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let output = part2(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 2 - Time: {:?} Output {:?}", end - start, output);
}

#[derive(Debug, Clone)]
struct Path {
    current: usize,
    visited: BTreeSet<usize>,
    costs: Vec<i32>,
}

impl Path {
    fn new(current: usize, visited: BTreeSet<usize>, costs: Vec<i32>) -> Self {
        Path {
            current,
            visited,
            costs,
        }
    }
}

struct TrailMap {
    map: Box<[u8]>,
    start: usize,
    end: usize,
    offset: usize,
}

impl TrailMap {
    fn parse(input: &str) -> TrailMap {
        let mut peek = input.lines().peekable();
        let offset = peek.peek().unwrap().len();
        let map = peek.flat_map(str::as_bytes).map(|&c| c).collect::<Box<_>>();
        let start = map[..offset]
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| {
                if n == b'.' {
                    return Some(i);
                }
                None
            })
            .collect::<Vec<_>>()[0];
        let end = map[map.len() - offset..]
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| {
                if n == b'.' {
                    return Some(i);
                }
                None
            })
            .collect::<Vec<_>>()[0];
        let end = map.len() - (offset - end);

        TrailMap {
            map,
            start,
            end,
            offset,
        }
    }

    fn next_pos(&self, p: usize, sloped: bool) -> Option<Vec<usize>> {
        let mut next: Vec<usize> = vec![];
        let mut pos = self.map[p];
        let slopes = [b'^', b'>', b'v', b'<'];
        // println!("pos: {pos} p: {p}");

        if !sloped && slopes.contains(&pos) {
            pos = b'.'
        }

        // up
        if p > self.offset && pos == b'.' || pos == b'^' {
            let n = p - self.offset;
            if self.map[n] != b'#' {
                next.push(n);
            }
        }

        // right
        if (p + 1) % self.offset != 0 && pos == b'.' || pos == b'>' {
            let n = p + 1;
            if self.map[n] != b'#' {
                next.push(n)
            }
        }

        // down
        if p < self.map.len() - self.offset && pos == b'.' || pos == b'v' {
            let n = p + self.offset;
            if self.map[n] != b'#' {
                next.push(n);
            }
        }

        // left
        if p % self.offset != 0 && pos == b'.' || pos == b'<' {
            let n = p - 1;
            if self.map[n] != b'#' {
                next.push(n);
            }
        }

        if next.len() > 0 {
            return Some(next);
        }

        None
    }

    fn find_path(&self, sloped: bool) -> i32 {
        let mut cost = vec![i32::MIN; self.map.len()];
        cost[self.start] = 0;
        let mut paths = vec![Path::new(self.start, BTreeSet::new(), cost.clone())];
        let mut longest = 0;

        while paths.len() > 0 {
            let cur_idx =
                paths.iter().enumerate().fold(
                    0,
                    |acc, (idx, i)| if i.costs[i.current] > 0 { idx } else { acc },
                );
            let mut cur = paths[cur_idx].clone();
            paths.remove(cur_idx);
            if let Some(next) = self.next_pos(cur.current, sloped) {
                for &n in &next {
                    if !cur.visited.contains(&n) {
                        cur.visited.insert(cur.current);
                        cur.costs[n] = cur.costs[cur.current] + 1;
                        paths.push(Path::new(n, cur.visited.clone(), cur.costs.clone()));

                        if n == self.end && cur.costs[n] > longest {
                            longest = cur.costs[n];
                        }
                    }
                }
            }
        }
        longest
    }
}

pub fn part1(input: &str) -> i32 {
    TrailMap::parse(input).find_path(true)
}

pub fn part2(input: &str) -> i32 {
    TrailMap::parse(input).find_path(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 154);
    }
}
