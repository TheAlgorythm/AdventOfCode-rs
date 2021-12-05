use std::collections::{HashMap, HashSet, VecDeque};

type Cards = VecDeque<u32>;
type Decks = HashMap<String, Cards>;

fn parse_decks(input: &str) -> Decks {
    input
        .split("\n\n")
        .map(|deck| {
            let deck_elements: Vec<&str> = deck.splitn(2, '\n').collect();
            (
                deck_elements[0][0..deck_elements[0].len() - 1].to_string(),
                deck_elements[1]
                    .lines()
                    .map(|card| card.parse().expect("Couldn't parse card!"))
                    .collect::<Cards>(),
            )
        })
        .collect()
}

fn get_winner(decks: &Decks) -> (String, Cards) {
    let mut decks = decks.clone();
    while decks.iter().all(|(_name, cards)| !cards.is_empty()) {
        let (round_winner, mut round_cards) = decks.iter_mut().fold(
            (String::new(), VecDeque::new()),
            |(round_winner, mut round_cards), (player, players_deck)| {
                let players_round_card = players_deck.pop_front().expect("Empty deck!");
                if round_cards
                    .iter()
                    .any(|round_card| *round_card > players_round_card)
                {
                    round_cards.push_back(players_round_card);
                    (round_winner, round_cards)
                } else {
                    round_cards.push_front(players_round_card);
                    (player.to_string(), round_cards)
                }
            },
        );
        let round_winner = round_winner.clone();
        decks
            .entry(round_winner)
            .and_modify(|players_deck| players_deck.append(&mut round_cards));
    }
    decks
        .into_iter()
        .find(|(_player, cards)| !cards.is_empty())
        .expect("No winner!")
}

fn get_recursive_winner(
    mut decks: Decks,
    mut previous_decks: HashMap<String, HashSet<Cards>>,
    outer: bool,
) -> (String, Cards) {
    if !outer
        && decks["Player 1"].iter().max()
            == decks
                .iter()
                .map(|(_player, deck)| deck.iter().max())
                .max()
                .expect("No player!")
    {
        return ("Player 1".to_string(), VecDeque::new());
    }
    while decks.iter().all(|(_name, cards)| !cards.is_empty()) {
        if decks
            .iter()
            .any(|(player, cards)| previous_decks.get(player).unwrap().contains(cards))
        {
            return decks
                .into_iter()
                .find(|(player, _cards)| *player == "Player 1")
                .expect("No winner!");
        }
        decks.iter().for_each(|(player, cards)| {
            previous_decks
                .entry(player.to_string())
                .and_modify(|previous_cards| {
                    previous_cards.insert(cards.clone());
                });
        });
        let (recursive_round, round_winner, mut round_cards, subdecks) = decks.iter_mut().fold(
            (true, String::new(), VecDeque::new(), HashMap::new()),
            |(recursive_round, round_winner, mut round_cards, mut subdecks),
             (player, players_deck)| {
                let players_round_card = players_deck.pop_front().expect("Empty deck!");
                subdecks.insert(
                    player.to_string(),
                    players_deck
                        .iter()
                        .take(players_round_card as usize)
                        .cloned()
                        .collect(),
                );
                if round_cards
                    .iter()
                    .any(|round_card| *round_card > players_round_card)
                {
                    round_cards.push_back(players_round_card);
                    (
                        recursive_round && (players_round_card <= players_deck.len() as u32),
                        round_winner,
                        round_cards,
                        subdecks,
                    )
                } else {
                    round_cards.push_front(players_round_card);
                    (
                        recursive_round && (players_round_card <= players_deck.len() as u32),
                        player.to_string(),
                        round_cards,
                        subdecks,
                    )
                }
            },
        );
        let round_winner = match recursive_round {
            true => {
                let (submatch_winner, _deck) =
                    get_recursive_winner(subdecks, previous_decks.clone(), false);
                if round_winner != submatch_winner {
                    round_cards = round_cards.into_iter().rev().collect();
                }
                submatch_winner
            }
            false => round_winner.clone(),
        };
        decks
            .entry(round_winner)
            .and_modify(|players_deck| players_deck.append(&mut round_cards));
    }
    decks
        .into_iter()
        .find(|(_player, cards)| !cards.is_empty())
        .expect("No winner!")
}

fn get_score(cards: &Cards) -> u32 {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(factor, card)| *card * (factor as u32 + 1))
        .sum()
}

fn solve_part_one(decks: &Decks) {
    let (winner, cards) = get_winner(decks);
    let score = get_score(&cards);

    println!("{} achieved a score of {}.", winner, score);
}

fn solve_part_two(decks: &Decks) {
    let (winner, cards) = get_recursive_winner(
        decks.clone(),
        decks
            .iter()
            .map(|(player, _cards)| (player.clone(), HashSet::new()))
            .collect(),
        true,
    );
    let score = get_score(&cards);

    println!(
        "{} achieved a score of {} at a recursive game.",
        winner, score
    );
}

fn main() {
    let input = include_str!("22_data.txt");

    let decks = parse_decks(input);

    solve_part_one(&decks);
    solve_part_two(&decks);
}
