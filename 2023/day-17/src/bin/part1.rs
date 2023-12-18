use std::num::NonZeroUsize;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {:?}", end - start, output);
}

pub struct Grid {
    data: Box<[u8]>,
    offset: usize,
}
impl Grid {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let line_len = lines.peek().map_or(0, |l| l.len());
        Self {
            data: lines
                .flat_map(str::as_bytes)
                .map(|&c| c - b'0')
                .collect::<Box<_>>(),
            offset: line_len,
        }
    }
    const fn next_pos(&self, p: usize, dir: u8) -> Option<usize> {
        // 0 = up, 1 = right, 2 = down, 3 = left
        Some(match dir {
            0 if p > self.offset => p - self.offset,
            1 if (p + 1) % self.offset != 0 => p + 1,
            2 if p < self.data.len() - self.offset => p + self.offset,
            3 if p % self.offset != 0 => p - 1,
            _ => return None,
        })
    }
    fn run(&self, dmin: usize, dmax: usize) -> Option<NonZeroUsize> {
        use std::cmp::Reverse as Rev;
        let lp = self.data.len() - 1;
        let mut visit = vec![0u8; self.data.len()];
        // Cache for vertical and horizontal directions
        let mut cache = vec![usize::MAX; 2 * self.data.len()];
        let mut q = std::collections::BinaryHeap::new();

        // start with both starting path direction options
        q.push((Rev(0), 0, 0));
        q.push((Rev(0), 0, 1));
        while let Some((Rev(cost), p, dir)) = q.pop() {
            if p == lp {
                return NonZeroUsize::new(cost);
            }
            if visit[p] & (1u8 << dir) != 0 {
                continue;
            }
            // visit[p] or added dir
            visit[p] |= 1u8 << dir;

            // old direction
            let odir = dir ^ 1;

            // next direction
            for nd in [odir, odir ^ 2] {
                let mut costsum = 0;

                // next position
                let mut np = p;

                // can only go max of dmax in same direction
                for dist in 1..=dmax {
                    if let Some(op) = self.next_pos(np, nd) {
                        costsum += self.data[op] as usize;

                        if dist >= dmin {
                            // add to the new path cost
                            let ncost = cost + costsum;
                            let cache_idx = (op << 1) | odir as usize;
                            if cache[cache_idx] > ncost {
                                cache[cache_idx] = ncost;
                                q.push((Rev(ncost), op, odir));
                            }
                        }
                        np = op;
                    }
                }
            }
        }
        None
    }
}

fn process(input: &str) -> Option<NonZeroUsize> {
    // min and max blocks
    Grid::from_str(input).run(4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "111111111111
999999999991
999999999991
999999999991
999999999991",
        );
        assert_eq!(result, NonZeroUsize::new(71))
    }
}
