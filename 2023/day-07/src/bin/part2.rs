use std::{
    cmp::{self, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let input = include_str!("./input1.txt");
    let output = process(input);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time: {:?} Output {}", end - start, output);
}

const CARDSTRENGTH: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hands {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hands {
    fn get_hand(cards: [i32; 13]) -> Hands {
        let jokers = cards[0];
        let mut highest: (usize, i32) = (0, i32::MIN);
        let mut cards = cards.clone();

        for index in 1..13 {
            if cards[index] > highest.1 {
                highest = (index, cards[index]);
            }
        }

        if jokers > 0 {
            cards[0] = 0;
            cards[highest.0] += jokers;
        }

        for index in 1..13 {
            if cards[index] == 5 {
                return Hands::FiveKind;
            }

            if cards[index] == 4 {
                return Hands::FourKind;
            }

            if cards[index] == 3 {
                for idx in 1..13 {
                    if idx == index {
                        continue;
                    }

                    if cards[idx] == 2 {
                        return Hands::FullHouse;
                    }
                }

                return Hands::ThreeKind;
            }

            if cards[index] == 2 {
                for idx in 1..13 {
                    if idx == index {
                        continue;
                    }

                    if cards[idx] == 2 {
                        return Hands::TwoPair;
                    }

                    if cards[idx] == 3 {
                        return Hands::FullHouse;
                    }
                }

                return Hands::OnePair;
            }
        }

        return Hands::HighCard;
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: String,
    hand: Hands,
    bid: i32,
}

impl Hand {
    fn new(cards: String, hand: Hands, bid: i32) -> Hand {
        Hand { cards, hand, bid }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<cmp::Ordering> {
        if self.hand == other.hand {
            for idx in 0..13 as usize {
                let s_cards = self.cards.chars().collect::<Vec<char>>();
                let o_cards = other.cards.chars().collect::<Vec<char>>();
                if s_cards[idx] != o_cards[idx] {
                    let s = CARDSTRENGTH
                        .iter()
                        .position(|n| *n == s_cards[idx])
                        .unwrap();
                    let o = CARDSTRENGTH
                        .iter()
                        .position(|n| *n == o_cards[idx])
                        .unwrap();

                    if s > o {
                        return Some(Ordering::Greater);
                    }

                    return Some(Ordering::Less);
                }
            }
        };
        Some(other.cmp(self))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> cmp::Ordering {
        if self.hand == other.hand {
            for idx in 0..13 as usize {
                let s_cards = self.cards.chars().collect::<Vec<char>>();
                let o_cards = other.cards.chars().collect::<Vec<char>>();
                if s_cards[idx] != o_cards[idx] {
                    let s = CARDSTRENGTH
                        .iter()
                        .position(|n| *n == s_cards[idx])
                        .unwrap();
                    let o = CARDSTRENGTH
                        .iter()
                        .position(|n| *n == o_cards[idx])
                        .unwrap();

                    if s > o {
                        return Ordering::Greater;
                    }

                    return Ordering::Less;
                }
            }
        };
        self.hand.cmp(&other.hand)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        if self.hand == other.hand {
            for idx in 0..13 as usize {
                let s_cards = self.cards.chars().collect::<Vec<char>>();
                let o_cards = other.cards.chars().collect::<Vec<char>>();
                if s_cards[idx] != o_cards[idx] {
                    return false;
                }
            }
            return true;
        }

        false
    }
}

fn process(input: &str) -> i32 {
    let input = input.replace("\r\n", "\n");
    let lines = &mut input.split("\n").collect::<Vec<_>>();

    let mut hands: Vec<Hand> = vec![];

    for l in lines {
        let mut cards = [0; 13];
        let h = l.split_ascii_whitespace().collect::<Vec<&str>>();

        h[0].chars()
            .for_each(|c| cards[CARDSTRENGTH.iter().position(|n| *n == c).unwrap()] += 1);

        let hand = Hands::get_hand(cards);

        let bet = h[1].parse::<i32>().unwrap();

        hands.push(Hand::new(h[0].to_string(), hand, bet));
    }

    hands.sort();

    for h in &hands {
        println!("Hand: {:?}", h);
    }

    hands
        .iter()
        .enumerate()
        .map(|(idx, h)| h.bid * (idx as i32 + 1))
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let result = process(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 5905);
    }
}
