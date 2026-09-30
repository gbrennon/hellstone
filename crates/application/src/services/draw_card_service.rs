use crate::dto::{DrawCardRequest, DrawCardResponse};
use crate::errors::DrawCardError;
use crate::ports::inbound::UseCase;
use crate::ports::outbound::PlayerRepository;
use domain::value_objects::PlayerId;

/// Moves the top card of a player's deck into their hand and persists it.
///
/// Reports a missing player, an empty deck, a full hand, or a storage failure
/// raised by the injected repository.
#[derive(Debug, Clone)]
pub struct DrawCardService<Repository: PlayerRepository> {
    players: Repository,
}

impl<Repository: PlayerRepository> DrawCardService<Repository> {
    pub fn new(players: Repository) -> Self {
        Self { players }
    }
}

impl<Repository: PlayerRepository> UseCase for DrawCardService<Repository> {
    type Request = DrawCardRequest;
    type Response = DrawCardResponse;
    type Error = DrawCardError;

    async fn execute(&self, request: DrawCardRequest) -> Result<DrawCardResponse, DrawCardError> {
        let id = PlayerId::new(request.player_id());
        let mut player = self
            .players
            .find_by_id(&id)
            .await?
            .ok_or(DrawCardError::PlayerNotFound(request.player_id()))?;

        let drawn_card = player.draw_card()?;
        self.players.save(&player).await?;

        Ok(DrawCardResponse::new(
            drawn_card.id().current(),
            drawn_card.name().to_string(),
            player.hand().len(),
            player.deck().len(),
        ))
    }
}
