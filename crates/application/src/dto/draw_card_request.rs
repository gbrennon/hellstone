/// Primitive input naming the player drawing a card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawCardRequest {
    player_id: u64,
}

impl DrawCardRequest {
    pub fn new(player_id: u64) -> Self {
        Self { player_id }
    }

    pub fn player_id(&self) -> u64 {
        self.player_id
    }
}
