use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

#[derive(Debug, Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn trace_path(map: &Vec<&[u8]>, visited: &mut Vec<i32>) {
    let mut beams = vec![
        (0, 0, Dir::Right),
        // (0, 0, Dir::Down),
        // (0, 9, Dir::Up),
        // (9, 0, Dir::Left),
    ];

    while beams.len() > 0 {
        let mut remove = vec![];

        for b in 0..beams.len() {
            let x = beams[b].0;
            let y = beams[b].1;
            let dir = &beams[b].2;

            if *dir == Dir::Right {
                for i in map[y].iter().enumerate().skip(x) {
                    println!("Right: ({},{}): {}", i.0, y, *i.1 as char);
                    visited[i.0] += 1;
                    match i.1 {
                        b'|' => {
                            if y > 0 {
                                beams[b] = (i.0, y - 1, Dir::Up);
                            } else {
                                remove.push(b);
                            }

                            if y < map.len() - 1 {
                                beams.push((i.0, y + 1, Dir::Down))
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'\\' => {
                            if y < map.len() - 1 {
                                beams[b] = (i.0, y + 1, Dir::Down);
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'/' => {
                            if y < map.len() - 1 {
                                beams[b] = (i.0, y - 1, Dir::Up);
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        _ => {}
                    }
                    if i.0 == map[y].len() - 1 {
                        remove.push(b)
                    }
                }
            } else if *dir == Dir::Left {
                for i in map[y].iter().enumerate().skip(map[x].len() - x).rev() {
                    println!("Left: ({},{}): {}", i.0, y, *i.1 as char);
                    visited[i.0] += 1;
                    match i.1 {
                        b'|' => {
                            if y > 0 {
                                beams[b] = (i.0, y - 1, Dir::Up);
                            } else {
                                remove.push(b);
                            }

                            if y < map.len() - 1 {
                                beams.push((i.0, y + 1, Dir::Down))
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'\\' => {
                            if y < map.len() - 1 {
                                beams[b] = (i.0, y + 1, Dir::Down);
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'/' => {
                            if y < map.len() - 1 {
                                beams[b] = (i.0, y - 1, Dir::Up);
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        _ => {}
                    }
                    if i.0 == 0 {
                        remove.push(b)
                    }
                }
            } else if *dir == Dir::Down {
                for i in y..map.len() {
                    println!("Down: ({},{}): {}", x, i, map[i][x] as char);
                    visited[i * map[x].len()] += 1;
                    match map[i][x] {
                        b'-' => {
                            if x > 0 {
                                beams[b] = (x - 1, i, Dir::Left);
                            } else {
                                remove.push(b);
                            }

                            if x < map[x].len() - 1 {
                                beams.push((x + 1, i, Dir::Right))
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'\\' => {
                            if x < map[x].len() - 1 {
                                beams[b] = (x + 1, i, Dir::Right);
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'/' => {
                            if x > 0 {
                                beams[b] = (x - 1, i, Dir::Left);
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        _ => {}
                    }
                    if i == map.len() - 1 {
                        remove.push(b);
                    }
                    break;
                }
            } else if *dir == Dir::Up {
                for i in (0..=y).rev() {
                    println!("Up: ({},{}): {}", i, x, map[i][x] as char);
                    visited[i * map[x].len()] += 1;
                    match map[i][x] {
                        b'-' => {
                            if x > 0 {
                                beams[b] = (x - 1, i, Dir::Left);
                            } else {
                                remove.push(b);
                            }

                            if x < map[x].len() - 1 {
                                beams.push((x + 1, i, Dir::Right))
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'\\' => {
                            if x > 0 {
                                beams[b] = (x - 1, i, Dir::Left);
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'/' => {
                            if x < map[x].len() - 1 {
                                beams[b] = (x + 1, i, Dir::Right);
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        _ => {}
                    }
                    if i == 0 {
                        remove.push(b)
                    }
                }
            }
        }
        remove.sort();
        for &r in remove.iter().rev() {
            beams.remove(r);
        }
    }

    println!("beams: {beams:?} visited: {visited:?} ");
}

fn process(input: &str) -> usize {
    let lines = input.lines();

    let map = lines.map(|l| l.trim().as_bytes()).collect::<Vec<_>>();

    let mut visited = vec![0; map.len() * map[0].len()];

    trace_path(&map, &mut visited);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            r".|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....",
        );
        assert_eq!(result, 46)
    }
}
