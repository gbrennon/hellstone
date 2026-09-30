use application::dto::CreatePlayerRequest;
use application::errors::CreatePlayerError;
use application::ports::inbound::UseCase;
use application::services::CreatePlayerService;
use domain::entities::{Player, MAX_DECK_SIZE, MAX_HAND_SIZE};
use domain::value_objects::PlayerId;
use std::sync::Arc;
#[path = "support/block_on.rs"]
mod block_on;
#[path = "support/in_memory_player_repository.rs"]
mod in_memory_player_repository;
#[path = "support/unavailable_player_repository.rs"]
mod unavailable_player_repository;
#[path = "support/write_rejecting_player_repository.rs"]
mod write_rejecting_player_repository;

use block_on::block_on;
use in_memory_player_repository::InMemoryPlayerRepository;
use unavailable_player_repository::UnavailablePlayerRepository;
use write_rejecting_player_repository::WriteRejectingPlayerRepository;

fn request_for(name: &str) -> CreatePlayerRequest {
    CreatePlayerRequest::new(7, name.to_string(), 5, 2)
}

#[test]
fn creates_player_with_requested_identity() {
    let service = CreatePlayerService::new(InMemoryPlayerRepository::empty());

    let response = block_on(service.execute(request_for("Jaina"))).unwrap();

    assert_eq!(response.name(), "Jaina");
}

#[test]
fn creates_player_with_requested_hand_size() {
    let service = CreatePlayerService::new(InMemoryPlayerRepository::empty());

    let response = block_on(service.execute(request_for("Jaina"))).unwrap();

    assert_eq!(response.hand_size(), 2);
}

#[test]
fn creates_player_with_full_starting_health() {
    let service = CreatePlayerService::new(InMemoryPlayerRepository::empty());

    let response = block_on(service.execute(request_for("Jaina"))).unwrap();

    assert_eq!(response.health(), 30);
}

#[test]
fn persists_the_created_player() {
    let repository = Arc::new(InMemoryPlayerRepository::empty());
    let service = CreatePlayerService::new(Arc::clone(&repository));

    block_on(service.execute(request_for("Jaina"))).unwrap();

    assert!(repository.stored_player(7).is_some());
}

#[test]
fn rejects_blank_name() {
    let service = CreatePlayerService::new(InMemoryPlayerRepository::empty());

    let result = block_on(service.execute(request_for("   ")));

    assert_eq!(result.unwrap_err(), CreatePlayerError::EmptyName);
}

#[test]
fn rejects_identifier_already_taken() {
    let existing = Player::new(PlayerId::new(7), "Anduin", 5, 2);
    let service = CreatePlayerService::new(InMemoryPlayerRepository::holding(existing));

    let result = block_on(service.execute(request_for("Jaina")));

    assert_eq!(
        result.unwrap_err(),
        CreatePlayerError::PlayerAlreadyExists(7)
    );
}

#[test]
fn reports_storage_failure() {
    let service = CreatePlayerService::new(UnavailablePlayerRepository);

    let result = block_on(service.execute(request_for("Jaina")));

    assert_eq!(
        result.unwrap_err(),
        CreatePlayerError::Repository(UnavailablePlayerRepository::failure())
    );
}

#[test]
fn rejects_deck_larger_than_the_domain_limit() {
    let service = CreatePlayerService::new(InMemoryPlayerRepository::empty());
    let request = CreatePlayerRequest::new(7, "Jaina".to_string(), MAX_DECK_SIZE + 1, 2);

    let result = block_on(service.execute(request));

    assert_eq!(
        result.unwrap_err(),
        CreatePlayerError::DeckSizeOutOfRange {
            maximum: MAX_DECK_SIZE,
            requested: MAX_DECK_SIZE + 1
        }
    );
}

#[test]
fn rejects_hand_larger_than_the_domain_limit() {
    let service = CreatePlayerService::new(InMemoryPlayerRepository::empty());
    let request =
        CreatePlayerRequest::new(7, "Jaina".to_string(), MAX_DECK_SIZE, MAX_HAND_SIZE + 1);

    let result = block_on(service.execute(request));

    assert_eq!(
        result.unwrap_err(),
        CreatePlayerError::HandSizeOutOfRange {
            maximum: MAX_HAND_SIZE,
            requested: MAX_HAND_SIZE + 1
        }
    );
}

#[test]
fn rejects_hand_larger_than_the_requested_deck() {
    let service = CreatePlayerService::new(InMemoryPlayerRepository::empty());
    let request = CreatePlayerRequest::new(7, "Jaina".to_string(), 3, 5);

    let result = block_on(service.execute(request));

    assert_eq!(
        result.unwrap_err(),
        CreatePlayerError::HandSizeOutOfRange {
            maximum: 3,
            requested: 5
        }
    );
}

#[test]
fn stores_the_name_without_surrounding_blanks() {
    let repository = Arc::new(InMemoryPlayerRepository::empty());
    let service = CreatePlayerService::new(Arc::clone(&repository));
    let request = CreatePlayerRequest::new(7, "  Jaina  ".to_string(), 5, 2);

    let response = block_on(service.execute(request)).unwrap();

    assert_eq!(response.name(), "Jaina");
}

#[test]
fn reports_failure_to_persist_the_new_player() {
    let service = CreatePlayerService::new(WriteRejectingPlayerRepository);

    let result = block_on(service.execute(request_for("Jaina")));

    assert_eq!(
        result.unwrap_err(),
        CreatePlayerError::Repository(WriteRejectingPlayerRepository::failure())
    );
}
