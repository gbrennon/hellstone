use crate::entities::card::Card;
use crate::traits::CanBeAttacked;
use crate::value_objects::{
    Attack, CardId, CardRarity, CardType, Health, HealthError, Mana, PlayerId,
};
use std::fmt;

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

impl CanBeAttacked for Player {}

impl Player {
    pub fn new(
        id: PlayerId,
        name: &str,
        starting_deck_size: usize,
        starting_hand_size: usize,
    ) -> Self {
        let mut player = Self {
            id,
            name: name.to_string(),
            health: Health::default(),
            mana: Mana::new(BASE_MAX_MANA),
            deck: vec![],
            hand: vec![],
            current_mana: 0,
            max_mana_limit: BASE_MAX_MANA,
        };

        // Initialize deck and hand
        for _ in 0..starting_deck_size {
            player.deck.push(Card::new(
                CardId::new(42 + player.deck.len() as u64),
                "Test Card",
                1,
                CardType::Minion,
                CardRarity::Common,
                Some(Attack::new(1).unwrap()),
                Some(Health::new(1).unwrap()),
                ""
            ).unwrap());
        }

        for _ in 0..starting_hand_size {
            if !player.deck.is_empty() {
                let card = player.deck.pop().unwrap();
                player.hand.push(card);
            }
        }

        player.replenish_mana();
        player
    }

    // Getters
    pub fn id(&self) -> &PlayerId { &self.id }
    pub fn name(&self) -> &String { &self.name }
    pub fn health(&self) -> &Health { &self.health }
    pub fn mana(&self) -> &Mana { &self.mana }
    pub fn deck(&self) -> &[Card] { &self.deck }
    pub fn hand(&self) -> &[Card] { &self.hand }
    pub fn current_mana(&self) -> u8 { self.current_mana }
    pub fn max_mana_limit(&self) -> u8 { self.max_mana_limit }

    // Mutators
    pub fn replenish_mana(&mut self) {
        self.current_mana = self.mana.current();
    }

    pub fn draw_card(&mut self) -> Result<Card, PlayerError> {
        if self.deck.is_empty() {
            return Err(PlayerError::DeckEmpty);
        }
        let card = self.deck.pop().ok_or(PlayerError::DeckEmpty)?;
        if self.hand.len() >= MAX_HAND_SIZE {
            return Err(PlayerError::HandFull);
        }
        self.hand.push(card);
        Ok(self.hand.last().unwrap().clone())
    }

    pub fn play_card_from_hand(&mut self, card_id: CardId) -> Result<(), PlayerError> {
        match self.hand.iter().position(|c| c.id() == &card_id) {
            Some(i) => {
                let card = self.hand.remove(i);
                let cost = card.mana_cost();
                if !self.can_spend(cost) {
                    return Err(PlayerError::InsufficientMana {
                        available: self.current_mana(),
                        requested: cost,
                    });
                }
                self.spend_mana(cost);
                Ok(())
            }
            None => Err(PlayerError::CardNotInHand(card_id)),
        }
    }

    pub fn can_spend(&self, amount: u8) -> bool {
        self.current_mana >= amount
    }

    pub fn spend_mana(&mut self, amount: u8) {
        self.current_mana -= amount;
    }

    pub fn take_damage(&mut self, damage: u32) -> Result<u32, HealthError> {
        self.health.take_damage(damage)
    }

    pub fn is_dead(&self) -> bool {
        self.health.is_dead()
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
        write!(f, "{} (Health: {}, Mana: {})",
               self.name,
               self.health.current(),
               self.current_mana
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerError {
    DeckEmpty,
    CardNotInHand(CardId),
    CardNotInDeck(CardId),
    DeckFull,
    HandFull,
    PlayerDead,
    InsufficientMana { available: u8, requested: u8 },
}

impl From<HealthError> for PlayerError {
    fn from(error: HealthError) -> Self {
        match error {
            HealthError::AlreadyDead => Self::PlayerDead,
            _ => panic!("Unexpected health error: {:?}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_new() {
        let player = Player::new(
            PlayerId::new(1),
            "Test Player",
            STARTING_DECK_SIZE,
            STARTING_HAND_SIZE,
        );
        assert_eq!(player.hand.len(), STARTING_HAND_SIZE);
        assert_eq!(player.deck.len(), STARTING_DECK_SIZE - STARTING_HAND_SIZE);
        assert_eq!(player.mana().current(), BASE_MAX_MANA);
        assert_eq!(player.current_mana(), BASE_MAX_MANA);
    }

    #[test]
    fn test_player_draw_card_success() {
        let mut player = Player::new(
            PlayerId::new(1),
            "Test Player",
            5,
            3,
        );
        assert_eq!(player.hand.len(), 3);
        let card = player.draw_card().unwrap();
        assert_eq!(card.name(), "Test Card");
        assert_eq!(player.hand.len(), 4);
    }

    #[test]
    fn test_player_draw_card_empty_deck() {
        let mut player = Player::new(
            PlayerId::new(1),
            "Test Player",
            0,
            0,
        );
        let result = player.draw_card();
        assert!(result.is_err());
    }

    #[test]
    fn test_player_play_card_from_hand() {
        let mut player = Player::new(
            PlayerId::new(1),
            "Test Player",
            2,
            2,
        );
        let card_id = player.hand[0].id();
        let result = player.play_card_from_hand(*card_id);
        assert!(result.is_ok());
        assert_eq!(player.hand.len(), 1);
    }

    #[test]
    fn test_player_play_card_not_in_hand() {
        let mut player = Player::new(
            PlayerId::new(1),
            "Test Player",
            2,
            2,
        );
        let invalid_id = CardId::new(999);
        let result = player.play_card_from_hand(invalid_id);
        assert!(result.is_err());
    }
}