use application::ports::outbound::{PlayerRepository, RepositoryError};
use domain::entities::Player;
use domain::value_objects::PlayerId;

/// Answers lookups from a single held player and rejects every write.
#[derive(Debug)]
pub struct ReadOnlyPlayerRepository {
    player: Player,
}

impl ReadOnlyPlayerRepository {
    pub fn holding(player: Player) -> Self {
        Self { player }
    }

    pub fn failure() -> RepositoryError {
        RepositoryError::Unavailable("writes rejected".to_string())
    }
}

impl PlayerRepository for ReadOnlyPlayerRepository {
    async fn find_by_id(&self, _id: &PlayerId) -> Result<Option<Player>, RepositoryError> {
        Ok(Some(self.player.clone()))
    }

    async fn save(&self, _id: &Player) -> Result<(), RepositoryError> {
        Err(Self::failure())
    }
}
