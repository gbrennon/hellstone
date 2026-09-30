/// Primitive description of the card that moved from deck to hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawCardResponse {
    card_id: u64,
    card_name: String,
    hand_size: usize,
    deck_size: usize,
}

impl DrawCardResponse {
    pub fn new(card_id: u64, card_name: String, hand_size: usize, deck_size: usize) -> Self {
        Self {
            card_id,
            card_name,
            hand_size,
            deck_size,
        }
    }

    pub fn card_id(&self) -> u64 {
        self.card_id
    }

    pub fn card_name(&self) -> &str {
        &self.card_name
    }

    pub fn hand_size(&self) -> usize {
        self.hand_size
    }

    pub fn deck_size(&self) -> usize {
        self.deck_size
    }
}
