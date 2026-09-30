use application::ports::outbound::{PlayerRepository, RepositoryError};
use domain::entities::Player;
use domain::value_objects::PlayerId;

/// Holds no player and rejects every write.
#[derive(Debug, Default)]
pub struct WriteRejectingPlayerRepository;

impl WriteRejectingPlayerRepository {
    pub fn failure() -> RepositoryError {
        RepositoryError::Unavailable("writes rejected".to_string())
    }
}

impl PlayerRepository for WriteRejectingPlayerRepository {
    async fn find_by_id(&self, _id: &PlayerId) -> Result<Option<Player>, RepositoryError> {
        Ok(None)
    }

    async fn save(&self, _player: &Player) -> Result<(), RepositoryError> {
        Err(Self::failure())
    }
}
