use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let input = include_str!("./input1.txt");

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let output = part1(&parse(input));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 1 - Time: {:?} Output {:?}", end - start, output);

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let output = part2(&parse(input));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 2 - Time: {:?} Output {:?}", end - start, output);
}

use std::collections::VecDeque;

pub struct Input {
    up: Vec<Vec<usize>>,
    down: Vec<Vec<usize>>,
}

pub fn parse(input: &str) -> Input {
    // let mut bricks: Vec<_> = input.iter_unsigned::<usize>().chunk::<6>().collect();
    let mut bricks: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            let brick = l.split("~").into_iter().collect::<Vec<_>>();
            let brick_parts: Vec<&str> = brick.iter().flat_map(|p| p.split(",")).collect();
            brick_parts
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut heights = [[0; 10]; 10];
    let mut indices = [[0; 10]; 10];
    let mut up = vec![Vec::new(); bricks.len()];
    let mut down = vec![Vec::new(); bricks.len()];

    // Sort ascending by lowest z coordinate.
    bricks.sort_unstable_by_key(|b| b[2]);

    for (i, brick) in bricks.iter().enumerate() {
        let x1 = brick[0];
        let y1 = brick[1];
        let z1 = brick[2];
        let x2 = brick[3];
        let y2 = brick[4];
        let z2 = brick[5];
        let mut top = 0;

        if x2 > x1 {
            #[allow(clippy::needless_range_loop)]
            for x in x1..=x2 {
                top = top.max(heights[x][y1]);
            }

            if top > 0 {
                let mut previous = usize::MAX;

                for x in x1..=x2 {
                    if heights[x][y1] == top {
                        let index = indices[x][y1];
                        if index != previous {
                            up[index].push(i);
                            down[i].push(index);
                            previous = index;
                        }
                    }
                }
            }

            let next = top + z2 - z1 + 1;

            for x in x1..=x2 {
                heights[x][y1] = next;
                indices[x][y1] = i;
            }
        } else {
            for y in y1..=y2 {
                top = top.max(heights[x1][y]);
            }

            if top > 0 {
                let mut previous = usize::MAX;

                for y in y1..=y2 {
                    if heights[x1][y] == top {
                        let index = indices[x1][y];
                        if index != previous {
                            up[index].push(i);
                            down[i].push(index);
                            previous = index;
                        }
                    }
                }
            }

            let next = top + z2 - z1 + 1;

            for y in y1..=y2 {
                heights[x1][y] = next;
                indices[x1][y] = i;
            }
        }
    }

    Input { up, down }
}

pub fn part1(input: &Input) -> usize {
    let Input { down, .. } = input;
    let mut safe = vec![true; down.len()];

    for underneath in down {
        if underneath.len() == 1 {
            safe[underneath[0]] = false;
        }
    }

    safe.iter().filter(|&&b| b).count()
}

pub fn part2(input: &Input) -> usize {
    let Input { up, down } = input;
    let mut safe = vec![true; down.len()];

    for underneath in down {
        if underneath.len() == 1 {
            safe[underneath[0]] = false;
        }
    }

    let mut result = 0;
    let mut todo = VecDeque::new();
    let mut removed = vec![usize::MAX; down.len()];

    for (start, &safe) in safe.iter().enumerate() {
        if safe {
            continue;
        }

        todo.push_back(start);
        removed[start] = start;

        while let Some(current) = todo.pop_front() {
            for &next in &up[current] {
                if removed[next] != start && down[next].iter().all(|&i| removed[i] == start) {
                    result += 1;
                    removed[next] = start;
                    todo.push_back(next);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(&parse(
            r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9",
        ));
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let result = part2(&parse(
            r"1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9",
        ));
        assert_eq!(result, 7);
    }
}
