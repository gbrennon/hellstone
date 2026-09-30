use application::ports::outbound::{PlayerRepository, RepositoryError};
use domain::entities::Player;
use domain::value_objects::PlayerId;

/// Fails every operation as if the storage mechanism were unreachable.
#[derive(Debug, Default)]
pub struct UnavailablePlayerRepository;

impl UnavailablePlayerRepository {
    pub fn failure() -> RepositoryError {
        RepositoryError::Unavailable("connection refused".to_string())
    }
}

impl PlayerRepository for UnavailablePlayerRepository {
    async fn find_by_id(&self, _id: &PlayerId) -> Result<Option<Player>, RepositoryError> {
        Err(Self::failure())
    }

    async fn save(&self, _player: &Player) -> Result<(), RepositoryError> {
        Err(Self::failure())
    }
}
