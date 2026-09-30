use crate::dto::{PlayCardRequest, PlayCardResponse};
use crate::errors::PlayCardError;
use crate::ports::inbound::UseCase;
use crate::ports::outbound::PlayerRepository;
use domain::entities::PlayerError;
use domain::value_objects::{CardId, PlayerId};

/// Plays a card from a player's hand, paying its mana cost, then persists it.
///
/// Reports a missing player, a card absent from hand, insufficient mana, or a
/// storage failure raised by the injected repository.
#[derive(Debug, Clone)]
pub struct PlayCardService<Repository: PlayerRepository> {
    players: Repository,
}

impl<Repository: PlayerRepository> PlayCardService<Repository> {
    pub fn new(players: Repository) -> Self {
        Self { players }
    }
}

impl<Repository: PlayerRepository> UseCase for PlayCardService<Repository> {
    type Request = PlayCardRequest;
    type Response = PlayCardResponse;
    type Error = PlayCardError;

    async fn execute(&self, request: PlayCardRequest) -> Result<PlayCardResponse, PlayCardError> {
        let player_id = PlayerId::new(request.player_id());
        let mut player = self
            .players
            .find_by_id(&player_id)
            .await?
            .ok_or(PlayCardError::PlayerNotFound(request.player_id()))?;

        player
            .play_card_from_hand(CardId::new(request.card_id()))
            .map_err(|error| translate_play_failure(error, request.card_id()))?;
        self.players.save(&player).await?;

        Ok(PlayCardResponse::new(
            request.card_id(),
            player.current_mana(),
            player.hand().len(),
        ))
    }
}

fn translate_play_failure(error: PlayerError, card_id: u64) -> PlayCardError {
    match error {
        PlayerError::InsufficientMana {
            available,
            requested,
        } => PlayCardError::InsufficientMana {
            available,
            requested,
        },
        _ => PlayCardError::CardNotInHand(card_id),
    }
}
