/// Primitive input describing the player a caller wants created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePlayerRequest {
    player_id: u64,
    name: String,
    deck_size: usize,
    hand_size: usize,
}

impl CreatePlayerRequest {
    pub fn new(player_id: u64, name: String, deck_size: usize, hand_size: usize) -> Self {
        Self {
            player_id,
            name,
            deck_size,
            hand_size,
        }
    }

    pub fn player_id(&self) -> u64 {
        self.player_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn deck_size(&self) -> usize {
        self.deck_size
    }

    pub fn hand_size(&self) -> usize {
        self.hand_size
    }
}
