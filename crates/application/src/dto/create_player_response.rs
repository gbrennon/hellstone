/// Primitive snapshot of the player a caller just created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePlayerResponse {
    player_id: u64,
    name: String,
    health: u32,
    deck_size: usize,
    hand_size: usize,
}

impl CreatePlayerResponse {
    pub fn new(
        player_id: u64,
        name: String,
        health: u32,
        deck_size: usize,
        hand_size: usize,
    ) -> Self {
        Self {
            player_id,
            name,
            health,
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

    pub fn health(&self) -> u32 {
        self.health
    }

    pub fn deck_size(&self) -> usize {
        self.deck_size
    }

    pub fn hand_size(&self) -> usize {
        self.hand_size
    }
}
