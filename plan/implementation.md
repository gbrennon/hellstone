# Implementation Plan: Replace Damagable with Expressive Domain Capabilities

## Goal
Replace the generic `Damagable` trait with three expressive, game-domain-specific capabilities:
- `CanBeAttacked`
- `ImmuneToDamage`
- `HasAttackPower`

These will improve clarity, expressiveness, and alignment with real card game mechanics in HellStone.

## Scope
- **Files affected**:
  - `crates/domain/src/value_objects/mod.rs`
  - `crates/domain/src/entities/card.rs`
  - `crates/domain/src/entities/player.rs`
  - `crates/domain/src/entities/creature.rs`
  - `crates/domain/src/traits/damagable.rs` → removed
  - Tests in `tests/` that depend on `Damagable`

- **No changes to infrastructure or TUI layer**
- **No breaking changes to public API**, except internal refactoring of trait usage

## Step-by-Step Execution
### Phase 1: Prepare Environment
1. Confirm current state via `cargo build --all`
2. Create backup branch: `git checkout -b feat/refactor-capabilities`
3. Ensure all tests pass before starting

### Phase 2: Define New Traits
1. Create new files under `crates/domain/src/traits/`:
   - `can_be_attacked.rs`
   - `immune_to_damage.rs`
   - `has_attack_power.rs`
2. Implement each as a marker trait with clear doc comments.
3. Add re-export in `lib.rs`.

### Phase 3: Apply Traits to Entities
1. In `card.rs`, add appropriate traits based on type:
   - Creatures are `CanBeAttacked`, possibly `ImmuneToDamage`, and have `HasAttackPower`
   - Players may be `CanBeAttacked` but not typically attackable (unless taunted)
2. Update constructors and methods accordingly.
3. Remove any references to old `damagable` logic.

### Phase 4: Clean Up & Verify
1. Delete obsolete file: `crates/domain/src/traits/damagable.rs`
2. Run `cargo check` and `cargo test` to verify no regressions
3. Rebuild and run end-to-end integration test if available
4. Commit with conventional message: `feat(domain): replace damagable with expressive capabilities`
5. Push to remote and open PR for review

## Acceptance Criteria
- [ ] No compilation errors after implementation
- [ ] All unit and integration tests pass
- [ ] Code is clean, well-documented, follows existing conventions
- [ ] Old `damagable` trait is completely removed
- [ ] New traits are used consistently across relevant entity types
- [ ] Documentation updated where needed (e.g., module-level docs)

## Risks & Mitigations
| Risk | Mitigation |
|------|------------|
| Breaking change in combat logic | Test coverage must include edge cases |
| Overlapping or conflicting trait behavior | Use composition rules; avoid inheritance |
| Naming confusion | Review with team; use consistent naming pattern |
| Tests fail due to missing mock data | Update fixtures and fakes in test suite |

## Next Steps After Completion
- Review PR with team
- Merge into main after approval
- Document the design decision in `docs/design.md`