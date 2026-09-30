use crate::dto::CreatePlayerRequest;
use crate::errors::CreatePlayerError;
use domain::entities::{MAX_DECK_SIZE, MAX_HAND_SIZE};

/// Answers the trimmed name of a request whose sizes fit the domain limits.
///
/// Rejects a blank name, a deck above the domain maximum, and a hand larger
/// than either the domain maximum or the requested deck.
pub fn validated_name(request: &CreatePlayerRequest) -> Result<&str, CreatePlayerError> {
    let name = request.name().trim();
    reject_blank_name(name)?;
    reject_oversized_deck(request)?;
    reject_oversized_hand(request)?;

    Ok(name)
}

fn reject_blank_name(name: &str) -> Result<(), CreatePlayerError> {
    match name.is_empty() {
        true => Err(CreatePlayerError::EmptyName),
        false => Ok(()),
    }
}

fn reject_oversized_deck(request: &CreatePlayerRequest) -> Result<(), CreatePlayerError> {
    match request.deck_size() > MAX_DECK_SIZE {
        true => Err(CreatePlayerError::DeckSizeOutOfRange {
            maximum: MAX_DECK_SIZE,
            requested: request.deck_size(),
        }),
        false => Ok(()),
    }
}

fn reject_oversized_hand(request: &CreatePlayerRequest) -> Result<(), CreatePlayerError> {
    let maximum = MAX_HAND_SIZE.min(request.deck_size());

    match request.hand_size() > maximum {
        true => Err(CreatePlayerError::HandSizeOutOfRange {
            maximum,
            requested: request.hand_size(),
        }),
        false => Ok(()),
    }
}
