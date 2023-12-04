advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Card {
    card_index: i32,
    first_number_list: Vec<i32>,
    second_number_list: Vec<i32>,
}

fn get_cards(input: &str) -> Vec<Card> {
    let mut cards = Vec::<Card>::new();
    for line in input.lines() {
        if line != "" {
            let card = Card{
                card_index: line.replace("  ", " ").split(':').collect::<Vec<&str>>()[0]
                            .replace("Card ", "").trim().parse().unwrap(),
                first_number_list: line.replace("  ", " ").split(':').collect::<Vec<&str>>()[1]
                            .split('|').collect::<Vec<&str>>()[0].trim().split(' ').collect::<Vec<&str>>()
                            .into_iter().map(|s| s.trim().parse().unwrap()).collect(),
                second_number_list: line.replace("  ", " ").split(':').collect::<Vec<&str>>()[1]
                            .split('|').collect::<Vec<&str>>()[1].trim().split(' ').collect::<Vec<&str>>()
                            .into_iter().map(|s| s.trim().parse().unwrap()).collect(),
            };

            cards.push(card);
        }
    }

    cards
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = get_cards(input);
    let mut card_pile_score: u32 = 0;

    for card in cards {
        let mut nb_matching_number: i32 = 0;
        for winning_number in card.first_number_list {
            if card.second_number_list.contains(&winning_number) {
                nb_matching_number += 1;
            }
        }

        let mut card_score = 0;
        if nb_matching_number > 0 {
            card_score = 1;
            for _ in 1..nb_matching_number {
                card_score *= 2;
            }
        }

        card_pile_score += card_score;
    }

    Some(card_pile_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = get_cards(input);
    let mut cards_copy = vec![0; cards.len()];
    let mut nb_cards: u32 = 0;

    for card in cards.clone() {
        let mut nb_matching_number: i32 = 0;
        let nb_copy = cards_copy[(card.card_index - 1) as usize];

        for winning_number in card.first_number_list {
            if card.second_number_list.contains(&winning_number) {
                nb_matching_number += 1;
            }
        }

        for i in 0..nb_matching_number {
            if (card.card_index+i) < cards.clone().len() as i32 {
                cards_copy[(card.card_index+i) as usize] += 1;
                cards_copy[(card.card_index+i) as usize] += nb_copy;
            }
        }

        nb_cards += 1;
        if nb_copy != 0 {
            nb_cards += cards_copy[(card.card_index - 1) as usize];
        }
    }

    Some(nb_cards)
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
