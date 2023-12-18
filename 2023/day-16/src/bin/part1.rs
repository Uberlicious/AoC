use std::{
    collections::HashMap,
    collections::{hash_map::Entry::*, HashSet},
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    row: usize,
    col: usize,
    dir: Dir,
}

fn trace_path(map: &Vec<&[u8]>, visited: &mut Vec<i32>) {
    let mut beams = vec![Beam {
        row: 0,
        col: 0,
        dir: Dir::Right,
    }];

    let mut beams_checked: HashSet<Beam> = HashSet::new();

    while beams.len() > 0 {
        let mut remove = vec![];

        for b in 0..beams.len() {
            if b != 0 {
                continue;
            }
            let row = beams[b].row;
            let col = beams[b].col;
            let dir = &beams[b].dir;

            if *dir == Dir::Right {
                for i in map[row].iter().enumerate().skip(col) {
                    let col = i.0;
                    visited[(row * map[row].len()) + col] += 1;
                    match i.1 {
                        b'|' => {
                            if row > 0 {
                                beams[b] = Beam {
                                    row: row - 1,
                                    col,
                                    dir: Dir::Up,
                                };
                            } else {
                                remove.push(b);
                            }

                            if row < map.len() - 1 {
                                beams.push(Beam {
                                    row: row + 1,
                                    col,
                                    dir: Dir::Down,
                                })
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'\\' => {
                            if row < map.len() - 1 {
                                beams[b] = Beam {
                                    row: row + 1,
                                    col,
                                    dir: Dir::Down,
                                };
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'/' => {
                            if row > 0 {
                                beams[b] = Beam {
                                    row: row - 1,
                                    col,
                                    dir: Dir::Up,
                                };
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        _ => {}
                    }
                    if col == map[row].len() - 1 {
                        remove.push(b);
                        break;
                    }
                }
            } else if *dir == Dir::Left {
                for i in (0..=col).rev() {
                    let col = i;
                    visited[(row * map[row].len()) + col] += 1;
                    match map[row][col] {
                        b'|' => {
                            if row > 0 {
                                beams[b] = Beam {
                                    row: row - 1,
                                    col,
                                    dir: Dir::Up,
                                };
                            } else {
                                remove.push(b);
                            }

                            if row < map.len() - 1 {
                                beams.push(Beam {
                                    row: row + 1,
                                    col,
                                    dir: Dir::Down,
                                })
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'\\' => {
                            if row > 0 {
                                beams[b] = Beam {
                                    row: row - 1,
                                    col,
                                    dir: Dir::Up,
                                };
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'/' => {
                            if row < map.len() - 1 {
                                beams[b] = Beam {
                                    row: row + 1,
                                    col,
                                    dir: Dir::Down,
                                };
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        _ => {}
                    }
                    if col == 0 {
                        remove.push(b);
                        break;
                    }
                }
            } else if *dir == Dir::Down {
                for row in row..map.len() {
                    visited[(row * map[row].len()) + col] += 1;
                    match map[row][col] {
                        b'-' => {
                            if col > 0 {
                                beams[b] = Beam {
                                    row,
                                    col: col - 1,
                                    dir: Dir::Left,
                                };
                            } else {
                                remove.push(b);
                            }

                            if col < map[row].len() - 1 {
                                beams.push(Beam {
                                    row,
                                    col: col + 1,
                                    dir: Dir::Right,
                                })
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'\\' => {
                            if col < map[row].len() - 1 {
                                beams[b] = Beam {
                                    row,
                                    col: col + 1,
                                    dir: Dir::Right,
                                };
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'/' => {
                            if col > 0 {
                                beams[b] = Beam {
                                    row,
                                    col: col - 1,
                                    dir: Dir::Left,
                                };
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        _ => {}
                    }
                    if row == map.len() - 1 {
                        remove.push(b);
                        break;
                    }
                }
            } else if *dir == Dir::Up {
                for row in (0..=row).rev() {
                    visited[(row * map[row].len()) + col] += 1;
                    match map[row][col] {
                        b'-' => {
                            if col > 0 {
                                beams[b] = Beam {
                                    row,
                                    col: col - 1,
                                    dir: Dir::Left,
                                };
                            } else {
                                remove.push(b);
                            }

                            if col < map[col].len() - 1 {
                                beams.push(Beam {
                                    row,
                                    col: col + 1,
                                    dir: Dir::Right,
                                })
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'\\' => {
                            if col > 0 {
                                beams[b] = Beam {
                                    row,
                                    col: col - 1,
                                    dir: Dir::Left,
                                };
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        b'/' => {
                            if col < map[row].len() - 1 {
                                beams[b] = Beam {
                                    row,
                                    col: col + 1,
                                    dir: Dir::Right,
                                };
                            } else {
                                remove.push(b);
                            }
                            break;
                        }
                        _ => {}
                    }
                    if row == 0 {
                        remove.push(b);
                        break;
                    }
                }
            }

            if beams_checked.contains(&beams[b]) {
                remove.push(b);
                remove.dedup();
            } else {
                beams_checked.insert(beams[b].clone());
            }
        }
        remove.sort();
        for &r in remove.iter().rev() {
            beams.remove(r);
        }
    }
}

fn process(input: &str) -> i32 {
    let lines = input.lines();

    let map = lines.map(|l| l.trim().as_bytes()).collect::<Vec<_>>();

    let mut visited = vec![0; map.len() * map[0].len()];

    trace_path(&map, &mut visited);

    visited
        .into_iter()
        .map(|n| {
            if n > 0 {
                return 1;
            }
            0
        })
        .sum()
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
