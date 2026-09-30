use application::dto::DrawCardRequest;
use application::errors::DrawCardError;
use application::ports::inbound::UseCase;
use application::services::DrawCardService;
use domain::entities::Player;
use domain::value_objects::PlayerId;
use std::sync::Arc;
#[path = "support/block_on.rs"]
mod block_on;
#[path = "support/in_memory_player_repository.rs"]
mod in_memory_player_repository;
#[path = "support/read_only_player_repository.rs"]
mod read_only_player_repository;
#[path = "support/unavailable_player_repository.rs"]
mod unavailable_player_repository;

use block_on::block_on;
use in_memory_player_repository::InMemoryPlayerRepository;
use read_only_player_repository::ReadOnlyPlayerRepository;
use unavailable_player_repository::UnavailablePlayerRepository;

fn player_with(deck_size: usize, hand_size: usize) -> Player {
    Player::new(PlayerId::new(7), "Jaina", deck_size, hand_size)
}

fn service_for(player: Player) -> DrawCardService<Arc<InMemoryPlayerRepository>> {
    DrawCardService::new(Arc::new(InMemoryPlayerRepository::holding(player)))
}

#[test]
fn moves_a_card_from_deck_to_hand() {
    let service = service_for(player_with(5, 2));

    let response = block_on(service.execute(DrawCardRequest::new(7))).unwrap();

    assert_eq!(response.hand_size(), 3);
}

#[test]
fn removes_the_drawn_card_from_the_deck() {
    let service = service_for(player_with(5, 2));

    let response = block_on(service.execute(DrawCardRequest::new(7))).unwrap();

    assert_eq!(response.deck_size(), 2);
}

#[test]
fn answers_the_drawn_card_name() {
    let service = service_for(player_with(5, 2));

    let response = block_on(service.execute(DrawCardRequest::new(7))).unwrap();

    assert_eq!(response.card_name(), "Recruit");
}

#[test]
fn persists_the_player_after_drawing() {
    let repository = Arc::new(InMemoryPlayerRepository::holding(player_with(5, 2)));
    let service = DrawCardService::new(Arc::clone(&repository));

    block_on(service.execute(DrawCardRequest::new(7))).unwrap();

    assert_eq!(repository.stored_player(7).unwrap().hand().len(), 3);
}

#[test]
fn reports_unknown_player() {
    let service = DrawCardService::new(Arc::new(InMemoryPlayerRepository::empty()));

    let result = block_on(service.execute(DrawCardRequest::new(7)));

    assert_eq!(result.unwrap_err(), DrawCardError::PlayerNotFound(7));
}

#[test]
fn reports_empty_deck() {
    let service = service_for(player_with(0, 0));

    let result = block_on(service.execute(DrawCardRequest::new(7)));

    assert_eq!(result.unwrap_err(), DrawCardError::DeckEmpty);
}

#[test]
fn reports_full_hand() {
    let service = service_for(player_with(12, 10));

    let result = block_on(service.execute(DrawCardRequest::new(7)));

    assert_eq!(result.unwrap_err(), DrawCardError::HandFull);
}

#[test]
fn reports_storage_failure() {
    let service = DrawCardService::new(UnavailablePlayerRepository);

    let result = block_on(service.execute(DrawCardRequest::new(7)));

    assert_eq!(
        result.unwrap_err(),
        DrawCardError::Repository(UnavailablePlayerRepository::failure())
    );
}

#[test]
fn reports_failure_to_persist_the_drawn_card() {
    let service = DrawCardService::new(ReadOnlyPlayerRepository::holding(player_with(5, 2)));

    let result = block_on(service.execute(DrawCardRequest::new(7)));

    assert_eq!(
        result.unwrap_err(),
        DrawCardError::Repository(ReadOnlyPlayerRepository::failure())
    );
}
