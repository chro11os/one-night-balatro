#[cfg(test)]
mod tests {
    use super::*; // Import from the outer module

    // Import necessary components from the main crate
    use crate::score_manager::ScoreManager;
    use crate::structures::stats::{BaseModifiers, RelicData};
    use crate::structures::card::Card;
    use crate::structures::hand::HandRank;

    #[test]
    fn test_two_pair_with_relics() {
        let mut stats = BaseModifiers::default();
        stats.chips = 0; // Ensure starting chips are zero for calculation
        stats.mult = 0; // Ensure starting mult is zero for calculation

        // Simulate a Two Pair hand: e.g., Two Kings, Two Queens
        let mut played_cards_for_test = vec![
            Card::new(0, 0.0, 0.0), // Placeholder
            Card::new(1, 0.0, 0.0), // Placeholder
            Card::new(2, 0.0, 0.0), // Placeholder
            Card::new(3, 0.0, 0.0), // Placeholder
            Card::new(4, 0.0, 0.0), // Placeholder
        ];
        
        // Manually set card values for the test
        played_cards_for_test[0].value = 13; played_cards_for_test[0].suit = 0; // King
        played_cards_for_test[1].value = 13; played_cards_for_test[1].suit = 1; // King
        played_cards_for_test[2].value = 12; played_cards_for_test[2].suit = 2; // Queen
        played_cards_for_test[3].value = 12; played_cards_for_test[3].suit = 3; // Queen
        played_cards_for_test[4].value = 14; played_cards_for_test[4].suit = 0; // Ace (kicker)

        let hand_rank = HandRank::TwoPair; // Two Pair base score: 20 chips, 2 mult (as per Balatro values)

        // Create relics
        let relic_mult_add = RelicData {
            id: "relic_mult_add".to_string(),
            name: "PlusFourMult".to_string(), // "+4 Mult" Relic
            description: "Adds 4 to multiplier.".to_string(),
            value: 0,
        };
        let relic_mult_x = RelicData {
            id: "relic_mult_x".to_string(),
            name: "TimesTwoMult".to_string(), // "x2 Mult" Relic
            description: "Multiplies multiplier by 2.".to_string(),
            value: 0,
        };
        let equipped_relics = vec![relic_mult_add, relic_mult_x];

        // Expected Calculation:
        // Base for Two Pair: 20 Chips, 2 Mult (from poker::get_hand_base_score for TwoPair)
        // Relic 1 (PlusFourMult): Mult becomes 2 + 4 = 6
        // Relic 2 (TimesTwoMult): Mult becomes 6 * 2 = 12
        // Final Score: Chips * Mult = 20 * 12 = 240

        ScoreManager::calculate_hand_score(
            &mut stats,
            &played_cards_for_test,
            &[], // No held cards for this test
            hand_rank,
            &equipped_relics,
        );

        // Assertions
        // In the poker.rs file, the get_hand_base_score function for HandRank::TwoPair returns (20, 2)
        assert_eq!(stats.chips, 20, "Final chips should be 20");
        assert_eq!(stats.mult, 12, "Final mult should be 12");
        assert_eq!(stats.total_score, 240, "Final total score should be 240");
        assert_eq!(stats.round_score, 240, "Final round score should be 240");
    }
}