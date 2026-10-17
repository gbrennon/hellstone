# Domain Layer

This crate contains the core business logic of HellStone.

## Structure
- `entities/`: domain models (Player, Card)
- `value_objects/`: immutable values with behavior (CardId, Health, Mana)
- `repository/`: data access interfaces and implementations

All public APIs are exported via `lib.rs`.