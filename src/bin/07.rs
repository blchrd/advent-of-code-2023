use std::collections::HashSet;

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    hand_power: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum HandType {
    HighCard=1,
    OnePair=3,
    TwoPair=6,
    ThreeOfAKind=15,
    FullHouse=30,
    FourOfAKind=60,
    FiveOfAKind=120,
}

fn get_hand_power(hand: Vec<u32>) -> u32 {
    let mut card_counts: Vec<(u32, usize)> = hand.clone().into_iter().map(|cv| (cv ,hand.clone().into_iter().filter(|v| v == &cv).count())).collect();
    let mut hand_type = (0, 0, HandType::HighCard);

    let set: HashSet<_> = card_counts.drain(..).collect(); // dedup
    card_counts.extend(set.into_iter());

    for card_count in card_counts.clone() {
        if card_count.0 != 0 {
            if card_count.1 == 5 {
                hand_type = (card_count.0, 0, HandType::FiveOfAKind);
            } else if card_count.1 == 4 {
                hand_type = (card_count.0, 0, HandType::FourOfAKind);
            } else if card_count.1 == 3 && hand_type.2 == HandType::OnePair {
                hand_type = (card_count.0, hand_type.0 ,HandType::FullHouse);
            } else if card_count.1 == 2 && hand_type.2 == HandType::ThreeOfAKind {
                hand_type = (hand_type.0, card_count.0, HandType::FullHouse);
            } else if card_count.1 == 3 && hand_type.2 == HandType::HighCard {
                hand_type = (hand_type.0, 0, HandType::ThreeOfAKind);
            } else if card_count.1 == 2 && hand_type.2 == HandType::OnePair {
                if card_count.0 > hand_type.0 {
                    hand_type = (card_count.0, hand_type.0, HandType::TwoPair);
                } else {
                    hand_type = (hand_type.0, card_count.0, HandType::TwoPair);
                }
            } else if card_count.1 == 2 && hand_type.2 == HandType::HighCard {
                hand_type = (card_count.0, 0, HandType::OnePair);
            }
        }
    }

    for card_count in card_counts.clone() {
        if card_count.0 == 0 {
            if card_count.1 == 5 {
                hand_type.2 = HandType::FiveOfAKind
            } else {
                match hand_type.2 {
                    HandType::FourOfAKind => {hand_type.2 = HandType::FiveOfAKind}
                    HandType::ThreeOfAKind => {
                        if card_count.1 == 2 {
                            hand_type.2 = HandType::FiveOfAKind;
                        } else {
                            hand_type.2 = HandType::FourOfAKind;
                        }
                    }
                    HandType::TwoPair => {hand_type.2 = HandType::FullHouse}
                    HandType::OnePair => {
                        if card_count.1 == 3 {
                            hand_type.2 = HandType::FiveOfAKind;
                        } else if card_count.1 == 2 {
                            hand_type.2 = HandType::FourOfAKind;
                        } else if card_count.1 == 1 {
                            hand_type.2 = HandType::ThreeOfAKind;
                        }
                    }
                    HandType::HighCard => {
                        if card_count.1 == 4 {
                            hand_type.2 = HandType::FiveOfAKind;
                        } else if card_count.1 == 3 {
                            hand_type.2 = HandType::FourOfAKind;
                        } else if card_count.1 == 2 {
                            hand_type.2 = HandType::ThreeOfAKind;
                        } else if card_count.1 == 1 {
                            hand_type.2 = HandType::OnePair;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    hand_type.2 as u32// * (hand_type.0*2 + hand_type.1+1)
}

fn get_hands(input: &str, joker: bool) -> Vec<Hand> {
    let mut hands = Vec::<Hand>::new();
    for line in input.lines() {
        if line != "" {
            let hand: Vec<u32> = line.split(" ").collect::<Vec<&str>>()[0].trim().chars().into_iter().map(|s| {
                match s {
                    'A' => {14},
                    'K' => {13},
                    'Q' => {12},
                    'J' => {
                        if !joker {
                            11
                        } else {
                            0
                        }
                    },
                    'T' => {10},
                    _ => {s.to_digit(10).unwrap()}
                }
            }).collect();
            let bid = line.split(" ").collect::<Vec<&str>>()[1].trim().parse().unwrap();

            hands.push(Hand {
                cards: hand.clone(),
                bid: bid,
                hand_power: get_hand_power(hand.clone()),
            });
        }
    }

    hands
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = get_hands(input, false);

    hands.sort_by(|a, b| {
        if a.hand_power != b.hand_power {
            return a.hand_power.cmp(&b.hand_power);
        } else {
            for i in 0..5 {
                if a.cards[i] != b.cards[i] {
                    return a.cards[i].cmp(&b.cards[i]);
                }
            }
        }

        std::cmp::Ordering::Equal
    });

    dbg!(&hands);

    let mut result: u32 = 0;
    let mut i = 1;
    for hand in hands {
        result += hand.bid * i;
        i += 1;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = get_hands(input, true);

    hands.sort_by(|a, b| {
        if a.hand_power != b.hand_power {
            return a.hand_power.cmp(&b.hand_power);
        } else {
            for i in 0..5 {
                if a.cards[i] != b.cards[i] {
                    return a.cards[i].cmp(&b.cards[i]);
                }
            }
        }

        std::cmp::Ordering::Equal
    });

    dbg!(&hands);

    let mut result: u32 = 0;
    let mut i = 1;
    for hand in hands {
        result += hand.bid * i;
        i += 1;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
