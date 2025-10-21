
## Quick orientation for AI coding agents

This is a small Bevy-based 2D sprite demo / infra for state+direction driven animations.
Keep instructions short and actionable: follow existing patterns in `src/` and asset layout under `assets/`.

Current project status
- Very early prototype: there is currently only one character's worth of sprite sheets under `assets/textures/test_char/`.
- Sprites cover all 8 cardinal directions and a few character states/transitions. New characters should follow the same per-direction, per-state atlas layout.

Key files to read first
- `src/main.rs` — app construction (plugins, resources, startup/update systems).
- `src/rendering/sprite_render.rs` — loading atlases, animation loop, visibility rules (read `setup`, `process_atlases`, `execute_animations`).
- `src/rendering/sprite_state.rs` — `SpriteState` enum and `CurrentSpriteState` resource.
- `src/game/input.rs` — directional input -> `CurrentDirection` and `CurrentCharacterState`.

Big-picture architecture
- Single Bevy App. Global mutable state is stored as Resources named `Current*` (e.g. `CurrentDirection`, `CurrentSpriteState`, `CurrentCharacterState`).
- Sprites are organized by Direction × SpriteState. On startup `setup` loads images from `assets/textures/test_char/` and spawns temporary `ImageAtlasBundle` entries which `process_atlases` converts into Sprite + TextureAtlas entities.
- Animation lifecycle: `execute_animations` advances frames using an `AnimationConfig` Timer. The code only changes the global sprite state when a full animation loop completes (or if the current state is interruptible).

Important conventions and patterns (do not deviate without reason)
- Naming: `CurrentXxx` resources hold the single authoritative value for direction/state.
- Enums that represent entity properties are often Components (e.g. `Direction8`, `SpriteState`, `CharacterState`).
- Texture naming: files follow `textures/test_char/{direction}_{state}.png` or `{direction}_{state}_{cols}x{rows}.png`. See `parse_grid_from_filename` and `find_existing_texture` in `sprite_render.rs` for exact rules.
- Atlas construction: if filename encodes grid `NxM`, `process_atlases` will create a TextureAtlas using (image.width/cols, image.height/rows).
- Animation indices: `AnimationConfig` uses first/last indices and a repeating Timer; frame advancement is index-based.

Bevy version and docs note
- The project pins `bevy = "0.16.1"` in `Cargo.toml`. Bevy's API changes frequently; many online examples target older versions and can be misleading.
- Prefer reading the local source (`src/`) and Bevy's v0.16 docs or the crate source when in doubt. Avoid applying copy-paste fixes from tutorials without verifying against the pinned crate.

Build / lint / run (Windows PowerShell)
- Quick run from repo root:

```powershell
cargo run
```

- Recommended checks to keep the repo warning-free (run locally and in CI):

```powershell
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings  # run clippy and fail on warnings
```

 Verbose runtime info (asset logs):

```powershell
$env:RUST_LOG = 'info'; cargo run
```

Integration points worth checking
- Asset layout: `assets/textures/test_char/` — adding a new direction/state sprite is usually just adding appropriately named PNGs; code auto-loads combinations for all `Direction8::all()` × `SpriteState::all()`.
- Input wiring: `add_direction_input_systems(&mut app)` (in `src/game/input.rs`) adds the arrow-key handling; reuse this pattern to add other isolated input subsystems.

Edge cases and gotchas discovered in code
- An image may not exist; `find_existing_texture` searches `assets` and falls back to `{base}.png` or the first matching `{base}_NxM.png`. If no file exists, textures remain unloaded until added.
- Sprite state transitions only occur after a full animation loop (see `loop_completed` logic). To change this behavior, update `execute_animations` carefully—tests should cover expected transition timing.

Proactive small improvements to consider (safe, low-risk)
- Add unit tests for `parse_grid_from_filename` and `find_existing_texture` (pure functions). These help guard refactors and Bevy upgrades.
- Add CI steps that run `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo build` with `RUSTFLAGS='-D warnings'` to keep the repo warning-free.

How to extend
- New SpriteState: add variant to `src/rendering/sprite_state.rs`, include it in `all()` and add matching textures under `assets/textures/test_char/`.
- New character: add textures for all directions/states and optionally copy the commented spawn example in `src/game/spawn.rs`.

If anything in this file is unclear or you want the agent to emphasize different parts (tests, CI, or editor tooling), tell me which area and I'll expand the instructions.
