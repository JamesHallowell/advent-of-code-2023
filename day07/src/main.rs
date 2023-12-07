use std::{collections::HashMap, ops::AddAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: i32,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid card"),
        }
    }
}

fn hand_type(cards: [Card; 5]) -> Type {
    let counts: HashMap<_, _> = cards.into_iter().filter(|&card| card != Card::Joker).fold(
        HashMap::default(),
        |mut count, card| {
            count.entry(card).or_insert(0).add_assign(1);
            count
        },
    );

    let jokers = cards
        .into_iter()
        .filter(|&card| card == Card::Joker)
        .count();

    if jokers == 5 {
        return Type::FiveOfAKind;
    }

    match counts.len() {
        1 => Type::FiveOfAKind,
        2 => {
            if counts.values().any(|&count| count + jokers == 4) {
                Type::FourOfAKind
            } else {
                Type::FullHouse
            }
        }
        3 => {
            if counts.values().any(|&count| count + jokers == 3) {
                Type::ThreeOfAKind
            } else {
                Type::TwoPair
            }
        }
        4 => {
            if counts.values().any(|&count| count + jokers == 2) {
                Type::OnePair
            } else {
                Type::HighCard
            }
        }
        5 => Type::HighCard,
        _ => unreachable!("invalid hand"),
    }
}

fn hands(input: &'static str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            Hand {
                cards: cards
                    .chars()
                    .map(Card::from)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                bid: bid.parse().unwrap(),
            }
        })
        .collect()
}

fn replace_jacks(mut hand: Hand) -> Hand {
    for card in &mut hand.cards {
        if *card == Card::Jack {
            *card = Card::Joker;
        }
    }
    hand
}

fn part_1(mut hands: Vec<Hand>) -> i32 {
    hands.sort_by(|a, b| (hand_type(a.cards), a.cards).cmp(&(hand_type(b.cards), b.cards)));
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, play)| (rank + 1) as i32 * play.bid)
        .sum()
}

fn part_2(hands: Vec<Hand>) -> i32 {
    part_1(hands.into_iter().map(replace_jacks).collect())
}

fn main() {
    let hands = hands(include_str!("input.txt"));

    println!("Part 1: {}", part_1(hands.clone()));
    println!("Part 2: {}", part_2(hands));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hand_types() {
        assert_eq!(
            hand_type([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]),
            Type::FiveOfAKind
        );
        assert_eq!(
            hand_type([Card::Ace, Card::Ace, Card::Eight, Card::Ace, Card::Ace]),
            Type::FourOfAKind
        );
        assert_eq!(
            hand_type([Card::Two, Card::Three, Card::Three, Card::Three, Card::Two]),
            Type::FullHouse
        );
        assert_eq!(
            hand_type([Card::Ten, Card::Ten, Card::Ten, Card::Nine, Card::Eight]),
            Type::ThreeOfAKind
        );
        assert_eq!(
            hand_type([Card::Two, Card::Three, Card::Four, Card::Three, Card::Two]),
            Type::TwoPair
        );
        assert_eq!(
            hand_type([Card::Ace, Card::Two, Card::Three, Card::Ace, Card::Four]),
            Type::OnePair
        );
        assert_eq!(
            hand_type([Card::Two, Card::Three, Card::Four, Card::Five, Card::Six]),
            Type::HighCard
        );
    }

    #[test]
    fn hand_order() {
        assert!(Type::HighCard < Type::OnePair);
        assert!(Type::OnePair < Type::TwoPair);
        assert!(Type::TwoPair < Type::ThreeOfAKind);
        assert!(Type::ThreeOfAKind < Type::FullHouse);
        assert!(Type::FullHouse < Type::FourOfAKind);
        assert!(Type::FourOfAKind < Type::FiveOfAKind);
    }
}
