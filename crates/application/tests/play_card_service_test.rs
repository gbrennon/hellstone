mod support;

use application::dto::PlayCardRequest;
use application::errors::PlayCardError;
use application::ports::inbound::UseCase;
use application::services::PlayCardService;
use domain::entities::Player;
use domain::value_objects::PlayerId;
use std::sync::Arc;
use support::{block_on, InMemoryPlayerRepository, UnavailablePlayerRepository};

fn player_holding_cards() -> Player {
    Player::new(PlayerId::new(7), "Jaina", 5, 2)
}

fn card_in_hand_of(player: &Player) -> u64 {
    player.hand()[0].id().current()
}

fn service_for(player: Player) -> PlayCardService<Arc<InMemoryPlayerRepository>> {
    PlayCardService::new(Arc::new(InMemoryPlayerRepository::holding(player)))
}

#[test]
fn removes_the_played_card_from_hand() {
    let player = player_holding_cards();
    let card_id = card_in_hand_of(&player);
    let service = service_for(player);

    let response = block_on(service.execute(PlayCardRequest::new(7, card_id))).unwrap();

    assert_eq!(response.hand_size(), 1);
}

#[test]
fn charges_the_card_mana_cost() {
    let player = player_holding_cards();
    let card_id = card_in_hand_of(&player);
    let service = service_for(player);

    let response = block_on(service.execute(PlayCardRequest::new(7, card_id))).unwrap();

    assert_eq!(response.remaining_mana(), 9);
}

#[test]
fn persists_the_player_after_playing() {
    let player = player_holding_cards();
    let card_id = card_in_hand_of(&player);
    let repository = Arc::new(InMemoryPlayerRepository::holding(player));
    let service = PlayCardService::new(Arc::clone(&repository));

    block_on(service.execute(PlayCardRequest::new(7, card_id))).unwrap();

    assert_eq!(repository.stored_player(7).unwrap().hand().len(), 1);
}

#[test]
fn reports_unknown_player() {
    let service = PlayCardService::new(Arc::new(InMemoryPlayerRepository::empty()));

    let result = block_on(service.execute(PlayCardRequest::new(7, 1)));

    assert_eq!(result.unwrap_err(), PlayCardError::PlayerNotFound(7));
}

#[test]
fn reports_card_absent_from_hand() {
    let service = service_for(player_holding_cards());

    let result = block_on(service.execute(PlayCardRequest::new(7, 999)));

    assert_eq!(result.unwrap_err(), PlayCardError::CardNotInHand(999));
}

#[test]
fn reports_storage_failure() {
    let service = PlayCardService::new(UnavailablePlayerRepository);

    let result = block_on(service.execute(PlayCardRequest::new(7, 1)));

    assert_eq!(
        result.unwrap_err(),
        PlayCardError::Repository(UnavailablePlayerRepository::failure())
    );
}
