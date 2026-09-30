mod support;

use application::dto::CreatePlayerRequest;
use application::errors::CreatePlayerError;
use application::ports::inbound::UseCase;
use application::services::CreatePlayerService;
use domain::entities::Player;
use domain::value_objects::PlayerId;
use std::sync::Arc;
use support::{block_on, InMemoryPlayerRepository, UnavailablePlayerRepository};

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
