use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let string = include_str!("../input.txt");
    let r_string = string.replace("\r\n", "\n");

    let sum = r_string
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!("{:?}", sum);

    Ok(())
}
