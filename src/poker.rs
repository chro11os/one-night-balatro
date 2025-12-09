use crate::structures::card::Card;
use crate::structures::hand::HandRank;
use std::collections::HashMap;
use crate::structures::stats::{BaseModifiers, BossAbility}; // Added this import
// ... [get_counts, is_flush, is_straight, get_hand_base_score, get_card_chip_value remain exactly the same] ...
// (Paste previous helper functions here if replacing file, or append this new function)

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
    if !straight && values.contains(&14) && values.contains(&2) && values.contains(&3) && values.contains(&4) && values.contains(&5) {
        straight = true;
    }
    straight
}

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

pub fn get_card_chip_value(card: &Card) -> i32 {
    match card.value {
        14 => 11,
        11 | 12 | 13 => 10,
        v => v,
    }
}

pub fn get_hand_rank(hand: &[Card], stats: &BaseModifiers) -> HandRank {
    let counts = get_counts(hand);
    let flush = is_flush(hand);
    let straight = is_straight(hand);

    if straight && flush {
        if let BossAbility::SilenceSuit(silenced_suit) = stats.active_ability {
            if hand[0].suit != silenced_suit {
                return HandRank::StraightFlush;
            }
        } else {
            return HandRank::StraightFlush;
        }
    }

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
    if flush {
        if let BossAbility::SilenceSuit(silenced_suit) = stats.active_ability {
            if hand[0].suit != silenced_suit {
                return HandRank::Flush;
            }
        } else {
            return HandRank::Flush;
        }
    }
    if straight { return HandRank::Straight; }
    if threes == 1 { return HandRank::ThreeOfAKind; }
    if pairs == 2 { return HandRank::TwoPair; }
    if pairs == 1 { return HandRank::Pair; }

    HandRank::HighCard
}
pub fn apply_relic_bonuses(stats: &mut BaseModifiers, hand: &[Card]) {
    let rank = get_hand_rank(hand, stats);

    // Clone relics to avoid immutable borrow of stats while mutating it
    let relics = stats.equipped_relics.clone();

    for relic in relics {
        match relic.data.id.as_str() {
            "j_joker" => {
                stats.mult += 4;
            },
            "j_greedy" => {
                // Suit 1 is Diamonds (0=Heart, 1=Diamond, 2=Spade, 3=Club)
                let diamonds = hand.iter().filter(|c| c.suit == 1).count();
                if diamonds > 0 {
                    stats.chips += (diamonds as i32) * 10;
                }
            },
            "j_duo" => {
                if rank == HandRank::Pair { stats.mult *= 2; }
            },
            "j_trio" => {
                if rank == HandRank::ThreeOfAKind { stats.mult *= 3; }
            },
            "j_family" => {
                if rank == HandRank::FourOfAKind { stats.mult *= 4; }
            },
            "relic_twin_daggers" => {
                if rank == HandRank::Pair || rank == HandRank::TwoPair {
                    stats.mult += 1;
                }
            },
            "relic_fading_torch" => {
                stats.mult += 20;
            },
            _ => {}
        }
    }
}
// NEW: Helper to identify which cards actully contribute to the hand
pub fn get_scoring_ids(hand: &[Card], stats: &BaseModifiers) -> Vec<i32> {
    let rank = get_hand_rank(hand, stats);
    let counts = get_counts(hand);
    let mut ids = Vec::new();

    match rank {
        HandRank::StraightFlush | HandRank::Flush => {
            // For flush, we take the cards matching the dominant suit (should be all if is_flush is true)
            // But strict Balatro rules: Top 5 scoring cards if > 5.
            // Simplified: If it's a flush, all selected matching suit count.
            if let Some(suit) = hand.first().map(|c| c.suit) {
                // Sort by value desc to pick top 5 if we implemented >5 card selection later
                let mut flush_cards: Vec<&Card> = hand.iter().filter(|c| c.suit == suit).collect();
                flush_cards.sort_by(|a, b| b.value.cmp(&a.value));
                for card in flush_cards.iter().take(5) {
                    ids.push(card.id);
                }
            }
        }
        HandRank::Straight => {
            // Find the 5 cards making the straight
            let mut values: Vec<i32> = hand.iter().map(|c| c.value).collect();
            values.sort();
            values.dedup();

            // Logic to find the specific straight sequence
            // (Simplified: if it is a straight, taking the straight cards)
            // Ideally we iterate windows.
            // For One Night Balatro MVP: If straight, take all unique cards involved.
            for card in hand {
                ids.push(card.id); // In a 5-card straight select, all valid.
            }
        }
        HandRank::FourOfAKind => {
            for card in hand {
                if *counts.get(&card.value).unwrap_or(&0) == 4 {
                    ids.push(card.id);
                }
            }
        }
        HandRank::FullHouse => {
            for card in hand {
                let c = *counts.get(&card.value).unwrap_or(&0);
                if c == 3 || c == 2 {
                    ids.push(card.id);
                }
            }
        }
        HandRank::ThreeOfAKind => {
            for card in hand {
                if *counts.get(&card.value).unwrap_or(&0) == 3 {
                    ids.push(card.id);
                }
            }
        }
        HandRank::TwoPair => {
            for card in hand {
                if *counts.get(&card.value).unwrap_or(&0) == 2 {
                    ids.push(card.id);
                }
            }
        }
        HandRank::Pair => {
            for card in hand {
                if *counts.get(&card.value).unwrap_or(&0) == 2 {
                    ids.push(card.id);
                }
            }
        }
        HandRank::HighCard => {
            // Only the single highest card scores
            if let Some(max_card) = hand.iter().max_by_key(|c| c.value) {
                ids.push(max_card.id);
            }
        }
    }
    ids
}