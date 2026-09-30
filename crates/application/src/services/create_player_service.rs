use crate::dto::{CreatePlayerRequest, CreatePlayerResponse};
use crate::errors::CreatePlayerError;
use crate::ports::inbound::UseCase;
use crate::ports::outbound::PlayerRepository;
use crate::services::create_player_request_validator::validated_name;
use domain::entities::Player;
use domain::value_objects::PlayerId;

/// Creates a player with a starting deck and hand, then persists it.
///
/// Rejects an empty name, sizes beyond the domain limits, and an identifier
/// already taken, and reports any storage failure raised by the repository.
#[derive(Debug, Clone)]
pub struct CreatePlayerService<Repository: PlayerRepository> {
    players: Repository,
}

impl<Repository: PlayerRepository> CreatePlayerService<Repository> {
    /// Builds the service around the repository holding every player.
    pub fn new(players: Repository) -> Self {
        Self { players }
    }

    async fn reject_taken_identifier(&self, id: &PlayerId) -> Result<(), CreatePlayerError> {
        self.players.find_by_id(id).await?.map_or(Ok(()), |taken| {
            Err(CreatePlayerError::PlayerAlreadyExists(taken.id().current()))
        })
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
        let name = validated_name(&request)?;
        let id = PlayerId::new(request.player_id());
        self.reject_taken_identifier(&id).await?;

        let player = Player::new(id, name, request.deck_size(), request.hand_size());
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
