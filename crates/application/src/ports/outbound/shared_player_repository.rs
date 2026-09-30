use crate::ports::outbound::player_repository::PlayerRepository;
use crate::ports::outbound::repository_error::RepositoryError;
use domain::entities::Player;
use domain::value_objects::PlayerId;
use std::future::Future;
use std::sync::Arc;

impl<Shared: PlayerRepository> PlayerRepository for Arc<Shared> {
    fn find_by_id(
        &self,
        id: &PlayerId,
    ) -> impl Future<Output = Result<Option<Player>, RepositoryError>> + Send {
        (**self).find_by_id(id)
    }

    fn save(&self, player: &Player) -> impl Future<Output = Result<(), RepositoryError>> + Send {
        (**self).save(player)
    }
}
