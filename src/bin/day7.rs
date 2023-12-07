use std::cmp::Ordering;

use nom::InputIter;

#[derive(PartialOrd, PartialEq, Eq, Debug, Hash)]
enum Card {
    Num(u32),
    T,
    Q,
    K,
    A,
    J,
}

impl Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            n => Card::Num(n.to_digit(10).expect("invalid number")),
        }
    }
}

#[derive(PartialEq, Debug, PartialOrd, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from(hand: &Hand) -> Self {
        use HandType::*;
        let mut hash = std::collections::HashMap::<_, usize>::new();
        for card in hand.cards.iter() {
            *hash.entry(card).or_default() += 1;
        }

        let jokers = hash.get(&Card::J).unwrap_or(&0);

        let without_jokers =
            match [5, 4, 3, 2].map(|v| hash.iter().filter(|(_, val)| **val == v).count()) {
                [1, _, _, _] => FiveOfAKind,
                [_, 1, _, _] => FourOfAKind,
                [_, _, 1, 1] => FullHouse,
                [_, _, 1, _] => ThreeOfAKind,
                [_, _, _, 2] => TwoPair,
                [_, _, _, 1] => OnePair,
                _ => HighCard,
            };
        match (jokers, without_jokers) {
            (5, _)
            | (4, _)
            | (3, OnePair)
            | (2, ThreeOfAKind)
            | (1, FourOfAKind)
            | (_, FiveOfAKind) => FiveOfAKind,
            (3, _) | (2, OnePair) | (1, ThreeOfAKind) | (_, FourOfAKind) => FourOfAKind,
            (1, TwoPair) => FullHouse,
            (2, _) | (1, OnePair) | (_, ThreeOfAKind) => ThreeOfAKind,
            (1, _) => OnePair,
            (_, no_jocker) => no_jocker,
        }
    }
}

fn run1(input: &str) -> usize {
    let mut hands: Vec<_> = input.lines().map(Hand::from).collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}
fn main() {
    let contents = std::fs::read_to_string("inputs/day_7").expect("could not read input");
    println!("First: {}", run1(&contents));
}

#[derive(PartialEq, Debug, Eq)]
struct Hand {
    bid: usize,
    cards: Vec<Card>,
}

impl Hand {
    fn from(line: &str) -> Self {
        let (card, bid) = line.split_once(" ").expect("invalid line");
        assert_eq!(card.len(), 5);
        Self {
            cards: card.iter_elements().map(Card::from).collect(),
            bid: bid.parse().expect("invalid bid"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match HandType::from(self).partial_cmp(&HandType::from(other)) {
            Some(std::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            ord => ord,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[test]
fn test_ord_of_enums() {
    assert!(Card::K < Card::A);
    assert!(Card::T > Card::Num(9));
    assert!(Card::Num(8) > Card::Num(7));
    assert_eq!(Card::K, Card::K);
    assert_eq!(Card::Num(8), Card::Num(8));
}

#[test]
fn test_parsing() {
    assert_eq!(Card::from('A'), Card::A);
    assert_eq!(Card::from('1'), Card::Num(1));
}

#[test]
fn test_parse_line() {
    use Card::*;
    assert_eq!(
        Hand {
            cards: vec![Num(3), Num(2), T, Num(3), K],
            bid: 765
        },
        Hand::from("32T3K 765")
    );
}

#[test]
fn test_hand_type() {
    use HandType::*;
    assert_eq!(HandType::from(&Hand::from("32T3K 765")), OnePair);
    assert_eq!(HandType::from(&Hand::from("T55J5 765")), FourOfAKind);
    assert_eq!(HandType::from(&Hand::from("KK677 765")), TwoPair);
    assert!(HandType::from(&Hand::from("KK677 1")) > HandType::from(&Hand::from("32T3K 765")));
}

fn test_first_star() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    assert_eq!(5905, run1(input))
}
