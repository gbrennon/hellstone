/// Primitive snapshot of the player right after playing a card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayCardResponse {
    card_id: u64,
    remaining_mana: u8,
    hand_size: usize,
}

impl PlayCardResponse {
    pub fn new(card_id: u64, remaining_mana: u8, hand_size: usize) -> Self {
        Self {
            card_id,
            remaining_mana,
            hand_size,
        }
    }

    pub fn card_id(&self) -> u64 {
        self.card_id
    }

    pub fn remaining_mana(&self) -> u8 {
        self.remaining_mana
    }

    pub fn hand_size(&self) -> usize {
        self.hand_size
    }
}
