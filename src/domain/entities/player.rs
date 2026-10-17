use std::fmt;

use super::entities::card::Card;
use super::value_objects::{CardId, Health, Mana, PlayerId};

pub const MAX_HAND_SIZE: usize = 10;
pub const MAX_DECK_SIZE: usize = 30;
pub const STARTING_HAND_SIZE: usize = 3;
pub const STARTING_DECK_SIZE: usize = 30;
pub const BASE_MAX_MANA: u8 = 10;

#[derive(Debug, Clone)]
pub struct Player {
    id: PlayerId,
    name: String,
    health: Health,
    mana: Mana,
    deck: Vec<Card>,
    hand: Vec<Card>,
    current_mana: u8,
    max_mana_limit: u8,
}

impl Player {
    pub fn new(id: PlayerId, name: String) -> Self {
        Self {
            id,
            name,
            health: Health::default(),
            mana: Mana::default(),
            deck: Vec::with_capacity(MAX_DECK_SIZE),
            hand: Vec::with_capacity(MAX_HAND_SIZE),
            current_mana: 1,
            max_mana_limit: BASE_MAX_MANA,
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn health(&self) -> Health {
        self.health
    }

    pub fn mana(&self) -> Mana {
        self.mana
    }

    pub fn deck(&self) -> &[Card] {
        &self.deck
    }

    pub fn hand(&self) -> &[Card] {
        &self.hand
    }

    pub fn current_mana(&self) -> u8 {
        self.current_mana
    }

    pub fn max_mana(&self) -> u8 {
        self.max_mana_limit
    }

    pub fn hand_size(&self) -> usize {
        self.hand.len()
    }

    pub fn deck_size(&self) -> usize {
        self.deck.len()
    }

    pub fn is_dead(&self) -> bool {
        self.health.is_dead()
    }

    pub fn is_hand_full(&self) -> bool {
        self.hand.len() >= MAX_HAND_SIZE
    }

    pub fn is_deck_empty(&self) -> bool {
        self.deck.is_empty()
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        if self.hand.len() >= MAX_HAND_SIZE {
            return None;
        }
        self.deck.pop().map(|card| {
            self.hand.push(card.clone());
            card
        })
    }

    pub fn draw_cards(&mut self, count: usize) -> Vec<Card> {
        (0..count).map(|_| self.draw_card()).flatten().collect()
    }

    pub fn play_card(&mut self, card_id: CardId) -> Result<Card, PlayerError> {
        let index = self
            .hand
            .iter()
            .position(|c| c.id() == card_id)
            .ok_or(PlayerError::CardNotInHand(card_id))?;

        let card = self.hand.remove(index);
        Ok(card)
    }

    pub fn add_card_to_deck(&mut self, card: Card) -> Result<(), PlayerError> {
        if self.deck.len() >= MAX_DECK_SIZE {
            return Err(PlayerError::DeckFull);
        }
        self.deck.push(card);
        Ok(())
    }

    pub fn remove_card_from_deck(&mut self, card_id: CardId) -> Result<Card, PlayerError> {
        let index = self
            .deck
            .iter()
            .position(|c| c.id() == card_id)
            .ok_or(PlayerError::CardNotInDeck(card_id))?;
        Ok(self.deck.remove(index))
    }

    pub fn take_damage(&mut self, damage: u32) -> Result<u32, PlayerError> {
        if self.is_dead() {
            return Err(PlayerError::PlayerDead);
        }
        self.health.take_damage(damage).map_err(|e| e.into())
    }

    pub fn heal(&mut self, amount: u32) {
        let _ = self.health.heal(amount);
    }

    pub fn start_turn(&mut self) {
        self.mana.replenish();
        if self.current_mana < self.max_mana_limit {
            self.current_mana += 1;
        }

        for card in &mut self.hand {
            if card.is_minion() {
                card.wake_up();
            }
        }
    }

    pub fn spend_mana(&mut self, amount: u8) -> Result<(), PlayerError> {
        if self.current_mana >= amount {
            self.current_mana -= amount;
            Ok(())
        } else {
            Err(PlayerError::InsufficientMana {
                available: self.current_mana,
                requested: amount,
            })
        }
    }

    pub fn gain_mana(&mut self, amount: u8) {
        self.current_mana = self.current_mana.saturating_add(amount);
    }

    pub fn increase_max_mana(&mut self, amount: u8) {
        self.max_mana_limit = self.max_mana_limit.saturating_add(amount);
    }

    pub fn set_starting_max_mana(&mut self, value: u8) {
        self.max_mana_limit = value;
        self.current_mana = value.min(1);
    }

    pub fn restore_mana(&mut self) {
        self.current_mana = self.max_mana_limit;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [HP: {} | Mana: {}/{} | Hand: {} | Deck: {}]",
            self.name,
            self.health,
            self.current_mana,
            self.max_mana_limit,
            self.hand_size(),
            self.deck_size()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerError {
    CardNotInHand(CardId),
    CardNotInDeck(CardId),
    DeckFull,
    HandFull,
    PlayerDead,
    InsufficientMana { available: u8, requested: u8 },
}

impl From<crate::domain::value_objects::HealthError> for PlayerError {
    fn from(e: crate::domain::value_objects::HealthError) -> Self {
        match e {
            crate::domain::value_objects::HealthError::AlreadyDead => PlayerError::PlayerDead,
            crate::domain::value_objects::HealthError::CannotSetToZero => PlayerError::PlayerDead,
            crate::domain::value_objects::HealthError::AboveMaxHealth { .. } => {
                PlayerError::PlayerDead
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{Attack, CardRarity, CardType, Health};

    fn test_card() -> Card {
        Card::new(
            CardId::new(1),
            "Test Card".to_string(),
            2,
            CardType::Minion,
            CardRarity::Common,
            Some(Attack::new(2).unwrap()),
            Some(Health::new(3).unwrap()),
            "Test".to_string(),
        )
        .unwrap()
    }

    fn create_player() -> Player {
        let mut player = Player::new(PlayerId::new(1), "Test Player".to_string());
        for _ in 0..30 {
            player.add_card_to_deck(test_card()).unwrap();
        }
        player
    }

    #[test]
    fn test_player_equality_by_id() {
        let player1 = Player::new(PlayerId::new(1), "Player1".to_string());
        let player2 = Player::new(PlayerId::new(1), "Different".to_string());
        assert_eq!(player1, player2);
    }

    #[test]
    fn test_player_inequality_by_different_id() {
        let player1 = Player::new(PlayerId::new(1), "Player".to_string());
        let player2 = Player::new(PlayerId::new(2), "Player".to_string());
        assert_ne!(player1, player2);
    }

    #[test]
    fn test_player_new() {
        let player = Player::new(PlayerId::new(1), "Player".to_string());
        assert_eq!(player.name(), "Player");
        assert_eq!(player.health().current(), 30);
        assert_eq!(player.current_mana(), 1);
        assert_eq!(player.max_mana(), 10);
    }

    #[test]
    fn test_player_draw_card() {
        let mut player = Player::new(PlayerId::new(1), "Player".to_string());
        player.add_card_to_deck(test_card()).unwrap();

        let card = player.draw_card();
        assert!(card.is_some());
        assert_eq!(player.hand_size(), 1);
        assert_eq!(player.deck_size(), 0);
    }

    #[test]
    fn test_player_draw_card_hand_full() {
        let mut player = Player::new(PlayerId::new(1), "Player".to_string());
        for _ in 0..MAX_HAND_SIZE {
            player.hand.push(test_card());
        }

        let result = player.draw_card();
        assert!(result.is_none());
    }

    #[test]
    fn test_player_draw_cards() {
        let mut player = Player::new(PlayerId::new(1), "Player".to_string());
        for _ in 0..5 {
            player.add_card_to_deck(test_card()).unwrap();
        }

        let drawn = player.draw_cards(3);
        assert_eq!(drawn.len(), 3);
        assert_eq!(player.hand_size(), 3);
    }

    #[test]
    fn test_player_play_card() {
        let mut player = create_player();
        player.draw_card();

        let card_id = player.hand()[0].id();
        let played = player.play_card(card_id);
        assert!(played.is_ok());
        assert_eq!(player.hand_size(), 0);
    }

    #[test]
    fn test_player_play_card_not_in_hand() {
        let mut player = create_player();

        let result = player.play_card(CardId::new(999));
        assert!(result.is_err());
    }

    #[test]
    fn test_player_add_card_to_deck() {
        let mut player = Player::new(PlayerId::new(1), "Player".to_string());
        let result = player.add_card_to_deck(test_card());
        assert!(result.is_ok());
        assert_eq!(player.deck_size(), 1);
    }

    #[test]
    fn test_player_add_card_to_deck_full() {
        let mut player = create_player();

        let result = player.add_card_to_deck(test_card());
        assert!(result.is_err());
    }

    #[test]
    fn test_player_is_dead() {
        let mut player = create_player();
        assert!(!player.is_dead());

        player.take_damage(30).unwrap();
        assert!(player.is_dead());
    }

    #[test]
    fn test_player_start_turn() {
        let mut player = create_player();

        player.start_turn();
        assert_eq!(player.current_mana(), 2);
    }

    #[test]
    fn test_player_start_turn_max_mana() {
        let mut player = create_player();

        for _ in 0..9 {
            player.start_turn();
        }

        assert_eq!(player.current_mana(), 10);
        player.start_turn();
        assert_eq!(player.current_mana(), 10);
    }

    #[test]
    fn test_player_take_damage() {
        let mut player = create_player();
        let damage = player.take_damage(5);
        assert!(damage.is_ok());
        assert_eq!(damage.unwrap(), 5);
        assert_eq!(player.health().current(), 25);
    }

    #[test]
    fn test_player_spend_mana() {
        let mut player = create_player();
        player.spend_mana(1).unwrap();
        assert_eq!(player.current_mana(), 0);
    }

    #[test]
    fn test_player_gain_mana() {
        let mut player = create_player();
        player.gain_mana(5);
        assert_eq!(player.current_mana(), 6);
    }

    #[test]
    fn test_player_increase_max_mana() {
        let mut player = create_player();
        player.increase_max_mana(5);
        assert_eq!(player.max_mana(), 15);
    }

    #[test]
    fn test_player_set_starting_max_mana() {
        let mut player = Player::new(PlayerId::new(1), "Player".to_string());
        player.set_starting_max_mana(12);
        assert_eq!(player.max_mana(), 12);
    }

    #[test]
    fn test_player_display() {
        let player = create_player();
        assert_eq!(
            format!("{}", player),
            "Test Player [HP: 30 | Mana: 1/10 | Hand: 0 | Deck: 30]"
        );
    }
}
