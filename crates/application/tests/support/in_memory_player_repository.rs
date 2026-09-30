use application::ports::outbound::{PlayerRepository, RepositoryError};
use domain::entities::Player;
use domain::value_objects::PlayerId;
use std::collections::HashMap;
use std::sync::Mutex;

/// Stores players in memory so a lookup answers whatever was saved last.
#[derive(Debug, Default)]
pub struct InMemoryPlayerRepository {
    players: Mutex<HashMap<u64, Player>>,
}

impl InMemoryPlayerRepository {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn holding(player: Player) -> Self {
        let repository = Self::default();
        repository
            .players
            .lock()
            .unwrap()
            .insert(player.id().current(), player);
        repository
    }

    pub fn stored_player(&self, id: u64) -> Option<Player> {
        self.players.lock().unwrap().get(&id).cloned()
    }
}

impl PlayerRepository for InMemoryPlayerRepository {
    async fn find_by_id(&self, id: &PlayerId) -> Result<Option<Player>, RepositoryError> {
        Ok(self.players.lock().unwrap().get(&id.current()).cloned())
    }

    async fn save(&self, player: &Player) -> Result<(), RepositoryError> {
        self.players
            .lock()
            .unwrap()
            .insert(player.id().current(), player.clone());
        Ok(())
    }
}
