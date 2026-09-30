use crate::dto::{CreatePlayerRequest, CreatePlayerResponse};
use crate::errors::CreatePlayerError;
use crate::ports::inbound::UseCase;
use crate::ports::outbound::PlayerRepository;
use domain::entities::Player;
use domain::value_objects::PlayerId;

/// Creates a player with a starting deck and hand, then persists it.
///
/// Rejects an empty name and an identifier already taken, and reports any
/// storage failure raised by the injected repository.
#[derive(Debug, Clone)]
pub struct CreatePlayerService<Repository: PlayerRepository> {
    players: Repository,
}

impl<Repository: PlayerRepository> CreatePlayerService<Repository> {
    pub fn new(players: Repository) -> Self {
        Self { players }
    }
}

impl<Repository: PlayerRepository> UseCase for CreatePlayerService<Repository> {
    type Request = CreatePlayerRequest;
    type Response = CreatePlayerResponse;
    type Error = CreatePlayerError;

    async fn execute(
        &self,
        request: CreatePlayerRequest,
    ) -> Result<CreatePlayerResponse, CreatePlayerError> {
        if request.name().trim().is_empty() {
            return Err(CreatePlayerError::EmptyName);
        }

        let id = PlayerId::new(request.player_id());
        if self.players.find_by_id(&id).await?.is_some() {
            return Err(CreatePlayerError::PlayerAlreadyExists(request.player_id()));
        }

        let player = Player::new(id, request.name(), request.deck_size(), request.hand_size());
        self.players.save(&player).await?;

        Ok(CreatePlayerResponse::new(
            request.player_id(),
            player.name().to_string(),
            player.health().current(),
            player.deck().len(),
            player.hand().len(),
        ))
    }
}
