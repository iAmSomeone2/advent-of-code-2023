use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct ParseHandError;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum PartOneCard {
    Ace,
    King,
    Queen,
    Jack,
    Number(u32),
}

impl From<PartOneCard> for u32 {
    fn from(value: PartOneCard) -> Self {
        match value {
            PartOneCard::Ace => 14,
            PartOneCard::King => 13,
            PartOneCard::Queen => 12,
            PartOneCard::Jack => 11,
            PartOneCard::Number(val) => val,
        }
    }
}

impl PartialOrd for PartOneCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PartOneCard {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_num: u32 = (*self).into();
        let other_num: u32 = (*other).into();

        self_num.cmp(&other_num)
    }
}

impl TryFrom<char> for PartOneCard {
    type Error = ParseHandError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Number(10)),
            '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' => {
                let val = match value.to_digit(10) {
                    Some(num) => num,
                    None => return Err(ParseHandError),
                };
                Ok(Self::Number(val))
            }
            _ => Err(ParseHandError),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum PartTwoCard {
    Ace,
    King,
    Queen,
    Number(u32),
    Joker,
}

impl From<PartTwoCard> for u32 {
    fn from(value: PartTwoCard) -> Self {
        match value {
            PartTwoCard::Ace => 14,
            PartTwoCard::King => 13,
            PartTwoCard::Queen => 12,
            PartTwoCard::Joker => 1,
            PartTwoCard::Number(val) => val,
        }
    }
}

impl PartialOrd for PartTwoCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PartTwoCard {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_num: u32 = (*self).into();
        let other_num: u32 = (*other).into();

        self_num.cmp(&other_num)
    }
}

impl TryFrom<char> for PartTwoCard {
    type Error = ParseHandError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'T' => Ok(Self::Number(10)),
            '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' => {
                let val = match value.to_digit(10) {
                    Some(num) => num,
                    None => return Err(ParseHandError),
                };
                Ok(Self::Number(val))
            }
            'J' => Ok(Self::Joker),
            _ => Err(ParseHandError),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<HandType> for u32 {
    fn from(value: HandType) -> Self {
        match value {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

impl PartialOrd<Self> for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_num: u32 = (*self).into();
        let other_num: u32 = (*other).into();

        self_num.cmp(&other_num)
    }
}

impl HandType {
    fn new(cards: &[PartOneCard; 5]) -> Self {
        let mut card_map: HashMap<PartOneCard, u32> = HashMap::new();
        for card in cards {
            match card_map.get_mut(card) {
                Some(count) => *count += 1,
                None => {
                    let _ = card_map.insert(*card, 1);
                }
            }
        }

        return match card_map.len() {
            1 => {
                // Must be 5 of a kind since they're all the same
                Self::FiveOfAKind
            }
            2 => {
                // Can either be four of a kind or a full house
                let max_count = card_map.values().copied().max().unwrap();
                if max_count == 4 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                // Can either be three of a kind or two pairs
                let max_count = card_map.values().copied().max().unwrap();
                if max_count == 3 {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            _ => Self::HighCard,
        };
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    cards: [PartOneCard; 5],
    hand_type: HandType,
    bet: u64,
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        // Parse cards
        let cards_str = split.next().ok_or(ParseHandError)?;
        if cards_str.len() < 5 {
            return Err(ParseHandError);
        }

        let mut cards = [PartOneCard::Ace; 5];
        for (i, c) in cards_str.chars().take(5).enumerate() {
            let card: PartOneCard = c.try_into()?;
            cards[i] = card;
        }

        // Parse bet
        let bet_str = split.next().ok_or(ParseHandError)?;
        let bet = bet_str.parse::<u64>().or(Err(ParseHandError))?;

        // Determine HandType
        let hand_type = HandType::new(&cards);

        Ok(Self {
            cards,
            hand_type,
            bet,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Order by HandTypes if they aren't the same
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for (i, card) in self.cards.iter().enumerate() {
            let other_card = &other.cards[i];
            if card != other_card {
                return (*card).cmp(other_card);
            }
        }

        Ordering::Equal
    }
}

impl Hand {
    fn calculate_winnings(hands: &mut [Hand]) -> u64 {
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| {
                let rank = (i + 1) as u64;
                hand.bet * rank
            })
            .sum()
    }
}

fn main() {
    let mut hands = fs::read_to_string("input.txt")
        .expect("failed to open input file")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Hand>>();

    let winnings = Hand::calculate_winnings(&mut hands);

    println!("Part 1 result: {winnings}");
}

#[cfg(test)]
mod test {
    mod card {
        use crate::{ParseHandError, PartOneCard};

        #[test]
        fn try_parse_from_char() {
            let test_data = [
                ('A', Ok(PartOneCard::Ace)),
                ('4', Ok(PartOneCard::Number(4))),
                ('Z', Err(ParseHandError)),
            ];

            for (input, expected) in test_data {
                assert_eq!(input.try_into(), expected);
            }
        }
    }
    mod hand {
        use crate::{Hand, HandType, ParseHandError, PartOneCard};

        const INPUT_DATA: &str = "32T3K 765\n\
                                  T55J5 684\n\
                                  KK677 28\n\
                                  KTJJT 220\n\
                                  QQQJA 483";

        #[test]
        fn parse_from_str() {
            let test_data = [
                (
                    "TTT98 256",
                    Ok(Hand {
                        cards: [
                            PartOneCard::Number(10),
                            PartOneCard::Number(10),
                            PartOneCard::Number(10),
                            PartOneCard::Number(9),
                            PartOneCard::Number(8),
                        ],
                        hand_type: HandType::ThreeOfAKind,
                        bet: 256,
                    }),
                ),
                ("TTTZ9", Err(ParseHandError)),
                ("3", Err(ParseHandError)),
            ];

            for (s, expected) in test_data {
                assert_eq!(s.parse(), expected);
            }
        }

        #[test]
        fn calculate_winnings() {
            let mut hands = INPUT_DATA
                .lines()
                .filter_map(|line| line.parse::<Hand>().ok())
                .collect::<Vec<Hand>>();
            let expected = 6440;

            assert_eq!(Hand::calculate_winnings(&mut hands), expected);
        }
    }
    // mod hand_type {
    //     use crate::{Hand, HandType};
    //
    //     #[test]
    //     fn parse_from_cards() {
    //         let test_data = [
    //             ("AAAAA".parse::<Hand>().unwrap(), HandType::FiveOfAKind),
    //             ("AA8AA".parse().unwrap(), HandType::FourOfAKind),
    //             ("23332".parse().unwrap(), HandType::FullHouse),
    //             ("TTT98".parse().unwrap(), HandType::ThreeOfAKind),
    //             ("23432".parse().unwrap(), HandType::TwoPair),
    //             ("A23A4".parse().unwrap(), HandType::OnePair),
    //             ("23456".parse().unwrap(), HandType::HighCard),
    //         ];
    //
    //         for (hand, expected) in test_data {
    //             assert_eq!(HandType::new(&hand), expected);
    //         }
    //     }
    // }
}
