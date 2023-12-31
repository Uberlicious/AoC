pub fn main() {
    let map = include_bytes!("input1.txt");
    let width = map.iter().position(|&b| b == b'\n').unwrap();
    let start = map.iter().position(|&b| b == b'S').unwrap();

    let mut covered = vec![false; (width + 1) * width];
    let (mut pos, mut dir) = {
        if matches!(map[start - width - 1], b'|' | b'7' | b'F') {
            (start - width - 1, 0)
        } else if matches!(map[start + width + 1], b'|' | b'L' | b'J') {
            (start + width + 1, 2)
        } else {
            (start - 1, 3)
        }
    };

    std::iter::repeat(())
        .position(|_| unsafe {
            *covered.get_unchecked_mut(pos) = true;
            match (map.get_unchecked(pos), dir) {
                (b'|', 2) => pos += width + 1,
                (b'|', 0) => pos -= width + 1,
                (b'-', 3) => pos -= 1,
                (b'-', 1) => pos += 1,
                (b'L', 2) | (b'F', 0) => {
                    pos += 1;
                    dir = 1;
                }
                (b'L', 3) | (b'J', 1) => {
                    pos -= width + 1;
                    dir = 0;
                }
                (b'7', 0) | (b'J', 2) => {
                    pos -= 1;
                    dir = 3;
                }
                (b'7', 1) | (b'F', 3) => {
                    pos += width + 1;
                    dir = 2;
                }
                (b'S', _) => return true,
                (_, _) => unreachable!(),
            }
            false
        })
        .unwrap();

    let mut inside = false;
    println!(
        "{}",
        map.iter()
            .enumerate()
            .filter(|(pos, cell)| {
                let pipe = unsafe { *covered.get_unchecked(*pos) };
                inside &= pos % (width + 1) != 0;
                inside ^= pipe && matches!(*cell, b'|' | b'F' | b'7');
                inside && (!pipe || **cell == b'.') && (pos % (width + 1) != width)
            })
            .count()
    );
}
