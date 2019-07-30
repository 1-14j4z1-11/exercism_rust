extern crate regex;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;
use regex::Regex;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let hand_cards = hands
        .iter()
        .map(|h| PokerHand::parse(h))
        .filter(|h| h.is_some())
        .map(|h| h.unwrap())
        .collect::<Vec<_>>();

    for hand in hand_cards {
        println!("{:?} : {}", hand.rank().category, hand.raw_str)
    }

    None
}

const MAX_NUMBER: u32 = 13;
const NUM_OF_HANDS: usize = 5;

struct Card {
    suit: char,
    number: u32,
}

impl Card {
    fn parse(s: &str) -> Option<Self> {
        let regex = Regex::new(r"^(?:([2-9]|10)|(J|Q|K|A))(S|H|C|D)$").unwrap();
        match regex.captures(s) {
            None => None,
            Some(caps) => {
                let suit = caps
                    .get(3)
                    .unwrap()
                    .as_str()
                    .to_uppercase()
                    .chars()
                    .nth(0)
                    .unwrap();
                let number = match caps.get(1) {
                    Some(mat_num) => mat_num.as_str().parse().unwrap(),
                    None => match caps.get(2).unwrap().as_str() {
                        "J" => 11,
                        "Q" => 12,
                        "K" => 13,
                        "A" => 1,
                        _ => panic!(),
                    },
                };

                Some(Card {
                    suit: suit,
                    number: number,
                })
            }
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
enum PokerHandCategory {
    FiveOfAKind = 9,
    StraightFlush = 8,
    FourOfAKind = 7,
    FullHouse = 6,
    Flush = 5,
    Straight = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

struct PokerHandRank {
    category: PokerHandCategory,
    number_ranks: Vec<u32>,
}

impl PartialEq for PokerHandRank {
    fn eq(&self, other: &PokerHandRank) -> bool {
        if self.category != other.category {
            return false;
        }

        for (s, o) in self.number_ranks.iter().zip(other.number_ranks.iter()) {
            if s != o {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for PokerHandRank {
    fn partial_cmp(&self, other: &PokerHandRank) -> Option<Ordering> {
        match self.category.partial_cmp(&other.category) {
            None => return None,
            Some(cmp) => match cmp {
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Less => return Some(Ordering::Less),
                Ordering::Equal => {},
            }
        }

        for (s, o) in self.number_ranks.iter().zip(other.number_ranks.iter()) {
            match s.cmp(o) {
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Less => return Some(Ordering::Less),
                Ordering::Equal => {},
            }
        }

        Some(Ordering::Equal)
    }
}

struct PokerHand<'a> {
    cards: Vec<Card>,
    raw_str: &'a str,
}

impl<'a> PokerHand<'a> {
    fn parse(s: &'a str) -> Option<Self> {
        let mut cards = vec![];

        for word in s.split(' ') {
            match Card::parse(word) {
                None => return None,
                Some(card) => cards.push(card),
            };
        }

        if cards.len() == NUM_OF_HANDS {
            Some(PokerHand {
                cards: cards,
                raw_str: s,
            })
        } else {
            None
        }
    }

    fn rank(&self) -> PokerHandRank {
        let num_of_suits = self
            .cards
            .iter()
            .map(|c| c.suit)
            .collect::<HashSet<_>>()
            .len();
        let is_flush = num_of_suits == 1;
        let mut number_count_map = HashMap::new();

        for i in 1..=NUM_OF_HANDS {
            let mut numbers = vec![];
            for n in 1..=MAX_NUMBER {
                let num_of_n = self.cards.iter().filter(|c| c.number == i as u32).count();
                if num_of_n == i as usize {
                    numbers.insert(0, n);
                }
            }
            number_count_map.insert(i, numbers);
        }

        fn num_rank(n: u32) -> u32 {
            match n {
                1 => MAX_NUMBER + 1,
                _ => n,
            }
        };

        fn is_straight(cards: &[Card]) -> (bool, u32) {
            let mut chain = 0;
            let numbers = cards.iter().map(|c| c.number).collect::<HashSet<_>>();

            for i in 1..=(MAX_NUMBER + 1) {
                if numbers.contains(&((i - 1) % MAX_NUMBER + 1)) {
                    chain += 1;
                } else {
                    chain = 0;
                }

                if chain == NUM_OF_HANDS {
                    return (true, i);
                }
            }

            (false, 0)
        }

        let get_number_having_count = |count: usize| {
            let numbers = number_count_map.get(&count).unwrap();

            if numbers.len() == 0 {
                None
            } else {
                Some(*numbers.first().unwrap())
            }
        };

        let get_numbers_having_count = |count: usize| {
            let numbers = number_count_map.get(&count).unwrap();

            if numbers.len() == 0 {
                None
            } else {
                Some(numbers)
            }
        };

        if let Some(number) = get_number_having_count(5) {
            return PokerHandRank {
                category: PokerHandCategory::FiveOfAKind,
                number_ranks: vec![num_rank(number)],
            };
        }

        if let Some(number) = get_number_having_count(4) {
            return PokerHandRank {
                category: PokerHandCategory::FiveOfAKind,
                number_ranks: vec![
                    num_rank(number),
                    num_rank(get_number_having_count(1).unwrap()),
                ],
            };
        }

        if let Some(number) = get_number_having_count(3) {
            if let Some(sub_number) = get_number_having_count(2) {
                return PokerHandRank {
                    category: PokerHandCategory::FullHouse,
                    number_ranks: vec![num_rank(number), num_rank(sub_number)],
                };
            } else {
                let mut num_ranks = vec![num_rank(number)];

                for &n in get_numbers_having_count(1).unwrap() {
                    num_ranks.push(n);
                }

                return PokerHandRank {
                    category: PokerHandCategory::ThreeOfAKind,
                    number_ranks: num_ranks,
                };
            }
        }

        if let Some(numbers) = get_numbers_having_count(2) {
            let mut num_ranks = numbers.iter().map(|&n| n).collect::<Vec<_>>();

            for &n in get_numbers_having_count(1).unwrap() {
                num_ranks.push(n);
            }

            let category = if numbers.len() == 2 {
                PokerHandCategory::TwoPair
            } else {
                PokerHandCategory::OnePair
            };

            return PokerHandRank {
                category: category,
                number_ranks: num_ranks,
            };
        }

        if let (true, num) = is_straight(&self.cards) {
            let category = if is_flush {
                PokerHandCategory::StraightFlush
            } else {
                PokerHandCategory::Straight
            };

            return PokerHandRank {
                category: category,
                number_ranks: vec![num],
            };
        }

        let category = if is_flush {
            PokerHandCategory::Flush
        } else {
            PokerHandCategory::HighCard
        };

        println!("{:?}", );

        let num_ranks = get_numbers_having_count(1)
            .unwrap()
            .iter()
            .map(|&n| num_rank(n))
            .collect::<Vec<_>>();

        PokerHandRank {
            category: category,
            number_ranks: num_ranks,
        }
    }
}
