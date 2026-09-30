use crate::ports::outbound::repository_error::RepositoryError;
use domain::entities::Player;
use domain::value_objects::PlayerId;
use std::future::Future;

/// Outbound contract for persisting and retrieving players.
///
/// Implementers must store a player so that a later lookup by the same
/// identifier answers the stored state, answer `Ok(None)` when no player
/// carries the requested identifier, and reserve `RepositoryError` for failures
/// of the storage mechanism itself.
pub trait PlayerRepository: Send + Sync {
    /// Answers the stored player carrying `id`, or `None` when none does.
    fn find_by_id(
        &self,
        id: &PlayerId,
    ) -> impl Future<Output = Result<Option<Player>, RepositoryError>> + Send;

    /// Stores `player` so a later lookup by its identifier answers this state.
    fn save(&self, player: &Player) -> impl Future<Output = Result<(), RepositoryError>> + Send;
}
