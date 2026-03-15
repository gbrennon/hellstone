use std::fmt;

use super::{Card, CardId, Health, Mana};

pub const MAX_HAND_SIZE: usize = 10;
pub const MAX_DECK_SIZE: usize = 30;
pub const STARTING_HAND_SIZE: usize = 3;
pub const STARTING_DECK_SIZE: usize = 30;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    id: PlayerId,
    name: String,
    health: Health,
    mana: Mana,
    deck: Vec<Card>,
    hand: Vec<Card>,
    max_mana: u8,
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
            max_mana: 1,
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

    pub fn max_mana(&self) -> u8 {
        self.max_mana
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

    pub fn start_turn(&mut self) {
        self.mana.replenish();
        if self.max_mana < 10 {
            self.max_mana += 1;
        }
    }

    pub fn spend_mana(&mut self, amount: u8) -> Result<(), PlayerError> {
        self.mana.spend(amount).map_err(|e| e.into())
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [HP: {} | Mana: {}/{} | Hand: {} | Deck: {}]",
            self.name,
            self.health,
            self.mana,
            self.max_mana,
            self.hand_size(),
            self.deck_size()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(u64);

impl PlayerId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn current(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlayerId({})", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerError {
    CardNotInHand(CardId),
    CardNotInDeck(CardId),
    DeckFull,
    HandFull,
    PlayerDead,
}

impl From<super::ManaError> for PlayerError {
    fn from(e: super::ManaError) -> Self {
        match e {
            super::ManaError::InsufficientMana { .. } => PlayerError::PlayerDead,
        }
    }
}

impl From<super::HealthError> for PlayerError {
    fn from(e: super::HealthError) -> Self {
        match e {
            super::HealthError::AlreadyDead => PlayerError::PlayerDead,
            super::HealthError::CannotSetToZero => PlayerError::PlayerDead,
            super::HealthError::AboveMaxHealth { .. } => PlayerError::PlayerDead,
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
        for i in 0..30 {
            let mut card = test_card();
            player.add_card_to_deck(card).unwrap();
        }
        player
    }

    #[test]
    fn test_player_new() {
        let player = Player::new(PlayerId::new(1), "Player".to_string());
        assert_eq!(player.name(), "Player");
        assert_eq!(player.health().current(), 30);
        assert_eq!(player.mana().current(), 1);
        assert_eq!(player.max_mana(), 1);
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
        for i in 0..MAX_HAND_SIZE {
            let mut card = test_card();
            player.hand.push(card);
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
        assert_eq!(
            result.unwrap_err(),
            PlayerError::CardNotInHand(CardId::new(999))
        );
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
        assert_eq!(result.unwrap_err(), PlayerError::DeckFull);
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
        assert_eq!(player.mana().current(), 2);
        assert_eq!(player.max_mana(), 2);
    }

    #[test]
    fn test_player_start_turn_max_mana() {
        let mut player = create_player();
        player.start_turn();
        player.start_turn();
        player.start_turn();
        player.start_turn();
        player.start_turn();
        player.start_turn();
        player.start_turn();
        player.start_turn();
        player.start_turn();
        player.start_turn();

        player.start_turn();
        assert_eq!(player.max_mana(), 10);
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
        assert_eq!(player.mana().current(), 0);
    }

    #[test]
    fn test_player_display() {
        let player = create_player();
        assert_eq!(
            format!("{}", player),
            "Test Player [HP: 30 | Mana: 1/1 | Hand: 0 | Deck: 30]"
        );
    }

    #[test]
    fn test_player_id_display() {
        assert_eq!(format!("{}", PlayerId::new(42)), "PlayerId(42)");
    }
}
