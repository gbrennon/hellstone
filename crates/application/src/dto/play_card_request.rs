/// Primitive input naming the player and the card being played.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayCardRequest {
    player_id: u64,
    card_id: u64,
}

impl PlayCardRequest {
    pub fn new(player_id: u64, card_id: u64) -> Self {
        Self { player_id, card_id }
    }

    pub fn player_id(&self) -> u64 {
        self.player_id
    }

    pub fn card_id(&self) -> u64 {
        self.card_id
    }
}
