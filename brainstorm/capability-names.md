# Brainstorm: Expressive Domain Capabilities for HellStone

## Goal
Replace the generic `damagable` trait with more expressive, game-domain-specific capabilities that reflect actual gameplay mechanics in a TUI card game.

## Context
- **HellStone** is a turn-based card game played in a terminal UI.
- Core entities: `Card`, `Player`, `Creature`
- Current model includes a `Damagable` trait, but its semantics are vague and not aligned with typical card game abstractions.
- The goal is to define capabilities that are:
  - **Expressive**: clearly convey purpose in code and documentation
  - **Domain-driven**: rooted in actual game rules and interactions
  - **Composable**: allow flexible combinations across different entity types

## Research & Inspiration
### Real Card Game Mechanics (Magic: The Gathering, Hearthstone)
- **Targetable**: can be selected as an attack target (e.g., enemy creature, player)
- **Castable**: can be played from hand onto battlefield (requires mana cost)
- **Summoned**: enters play via casting; may have summoning sickness
- **Taunt**: forces opponent to attack first if possible
- **Battlecry**: triggers effect when summoned
- **Deathrattle**: triggers when destroyed
- **Stealth**: cannot be targeted by attacks until revealed
- **Divine Shield**: immune to damage once per turn
- **Windfury**: can attack multiple times per turn
- **Lifesteal**: gain health when dealing damage
- **Deathtouch**: destroys creatures with 1 or less toughness

### Common Abstractions in Game Engines
- `HasManaCost`: provides ability to pay mana cost to activate
- `CanBeAttacked`: determines whether an entity can be attacked
- `HasHealth`: tracks current HP and death state
- `IsAlive`: boolean flag indicating survival status
- `OnDamageReceived`: event handler for incoming damage
- `OnAttackPerformed`: event handler for attacking other units

## Refined Capability Names (Selected)
| Name | Purpose | Example Use Case |
|------|--------|------------------|
| `CanBeAttacked` | Determines eligibility for being attacked (e.g., Taunted creatures) | Enemy players and creatures are targetable; friendly ones usually aren't |
| `ImmuneToDamage` | Prevents all damage taken (like Divine Shield) | Temporary protection during battle |
| `HasAttackPower` | Provides base attack value used in combat calculations | Combat strength comparison between attackers and defenders |

## Additional Considerations
- These traits should be implemented on specific entities (`Card`, `Player`, `Creature`) rather than shared globally.
- They should support composition: e.g., a creature might be both `CanBeAttacked` and `ImmuneToDamage`.
- No need for complex inheritance hierarchies—simple marker traits suffice.
- Avoid over-engineering: keep interfaces minimal and focused.

## Next Steps
1. Define trait signatures based on the three selected names.
2. Map each capability to applicable entity types.
3. Remove the old `damagable.rs` file and replace it with these new, expressive traits.
4. Update related code paths (e.g., combat logic, targeting UI).
5. Run tests to ensure no regressions.