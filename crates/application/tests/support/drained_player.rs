use domain::entities::Player;
use domain::value_objects::PlayerId;

/// Builds a player whose mana is fully spent and who still holds one card.
///
/// Plays the whole opening hand to burn every mana crystal, then draws one
/// more card so a further play can only fail for lack of mana.
pub fn drained_player_with_one_card(id: u64) -> Player {
    let mut player = Player::new(PlayerId::new(id), "Jaina", 30, 10);

    while let Some(card_id) = player.hand().first().map(|card| *card.id()) {
        if player.play_card_from_hand(card_id).is_err() {
            break;
        }
    }
    player.draw_card().unwrap();

    player
}
