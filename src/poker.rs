use crate::structures::card::Card;
use crate::structures::hand::HandRank;
use std::collections::HashMap;

// --- Helper Functions ---
fn get_counts(hand: &[Card]) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();
    for card in hand {
        *counts.entry(card.value).or_insert(0) += 1;
    }
    counts
}

fn is_flush(hand: &[Card]) -> bool {
    if hand.len() < 5 { return false; }
    let first_suit = hand[0].suit;
    hand.iter().all(|card| card.suit == first_suit)
}

fn is_straight(hand: &[Card]) -> bool {
    if hand.len() < 5 { return false; }
    let mut values: Vec<i32> = hand.iter().map(|card| card.value).collect();
    values.sort();
    values.dedup();
    if values.len() < 5 { return false; }

    let mut straight = false;
    for i in 0..=(values.len() - 5) {
        if values[i+4] - values[i] == 4 {
            straight = true;
            break;
        }
    }
    // Ace-low straight (A, 2, 3, 4, 5)
    if !straight && values.contains(&14) && values.contains(&2) && values.contains(&3) && values.contains(&4) && values.contains(&5) {
        straight = true;
    }
    straight
}

// UPDATE: Added this function to map HandRank to base Chips and Mult
pub fn get_hand_base_score(rank: HandRank) -> (i32, i32) {
    match rank {
        HandRank::HighCard => (5, 1),
        HandRank::Pair => (10, 2),
        HandRank::TwoPair => (20, 2),
        HandRank::ThreeOfAKind => (30, 3),
        HandRank::Straight => (30, 4),
        HandRank::Flush => (35, 4),
        HandRank::FullHouse => (40, 4),
        HandRank::FourOfAKind => (60, 7),
        HandRank::StraightFlush => (100, 8),
    }
}

// UPDATE: Added this function to get individual card chip values (J,Q,K = 10, A = 11)
pub fn get_card_chip_value(card: &Card) -> i32 {
    match card.value {
        14 => 11,           // Ace
        11 | 12 | 13 => 10, // Face cards
        v => v,             // Number cards
    }
}

pub fn get_hand_rank(hand: &[Card]) -> HandRank {
    let counts = get_counts(hand);
    let flush = is_flush(hand);
    let straight = is_straight(hand);

    if straight && flush { return HandRank::StraightFlush; }

    // UPDATE: Reordered checks slightly to ensure FourOfAKind catches before FullHouse/ThreeOfAKind
    let mut pairs = 0;
    let mut threes = 0;
    let mut fours = 0;

    for &count in counts.values() {
        match count {
            2 => pairs += 1,
            3 => threes += 1,
            4 => fours += 1,
            _ => (),
        }
    }

    if fours == 1 { return HandRank::FourOfAKind; }
    if threes == 1 && pairs == 1 { return HandRank::FullHouse; }
    if flush { return HandRank::Flush; }
    if straight { return HandRank::Straight; }
    if threes == 1 { return HandRank::ThreeOfAKind; }
    if pairs == 2 { return HandRank::TwoPair; }
    if pairs == 1 { return HandRank::Pair; }

    HandRank::HighCard
}