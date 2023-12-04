fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Card {
    id: u32,
    amount: u32,
    numbers: Vec<u32>,
    winners: Vec<u32>,
}

impl Card {
    fn new(id: u32, numbers: Vec<u32>, winners: Vec<u32>) -> Card {
        Card {
            id,
            numbers,
            winners,
            amount: 1,
        }
    }

    fn increment(&mut self, amount: u32) -> &mut Self {
        self.amount += amount;
        self
    }
}

fn recurse(cards: &mut Vec<Card>, idx: usize, total: &mut u32) {
    if idx >= cards.len() {
        return;
    }
    let mut matches = 0;

    for n in &cards[idx].numbers {
        if cards[idx].winners.contains(&n) {
            matches += 1
        }
    }

    for m in cards[idx].id + 1..=cards[idx].id + matches {
        let a = cards[idx].amount;
        cards[m as usize - 1].increment(1 * a);
    }

    *total += cards[idx].amount;

    recurse(cards, idx + 1, total);
}

fn process(input: &str) -> u32 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();
    let mut cards: Vec<Card> = vec![];

    let mut total: u32 = 0;

    // get card data
    for line in lines {
        let game = line.split(":").collect::<Vec<_>>();
        let game_id = game[0].split_ascii_whitespace().collect::<Vec<_>>()[1]
            .parse::<u32>()
            .expect("card id");

        let nums = game[1]
            .split("|")
            .collect::<Vec<_>>()
            .iter()
            .map(|n| {
                return n
                    .trim()
                    .split_ascii_whitespace()
                    .map(|n| n.trim().parse::<u32>().expect("num"))
                    .collect::<Vec<u32>>();
            })
            .collect::<Vec<Vec<u32>>>();

        let numbers = &nums[0];
        let winners = &nums[1];

        cards.push(Card::new(game_id, numbers.clone(), winners.clone()))
    }

    recurse(&mut cards, 0, &mut total);

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
