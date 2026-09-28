# Rust WebAssembly Shooter Demo

Minimal browser-based game prototype written in Rust and compiled to WebAssembly.

The current demo renders a fullscreen arena where the player moves with arrow keys or WASD and shoots with Space. Enemies spawn from the edges of the screen and disappear when hit by bullets. Touching an enemy triggers game over, stores the best score, and Enter restarts the run. The code is intentionally split like a larger game project:

- `src/engine`: reusable engine-style Rust modules.
- `src/engine/physics.rs`: simple movement integration and bounds clamping.
- `src/game`: game-specific world, entities, spawning, shooting and collisions.
- `src/lib.rs`: WebAssembly export boundary.
- `web/platform`: browser platform concerns such as input and Wasm loading.
- `web/rendering`: canvas renderer.
- `web/game-runtime.js`: browser-side game loop.

## Controls

- Move: arrow keys or WASD
- Shoot: Space
- Restart after game over: Enter

## Requirements

- Rust toolchain with `cargo`
- `wasm32-unknown-unknown` target
- A local HTTP server

## Build

If `cargo` is not available in a new terminal yet, load Rust's environment first:

```bash
source "$HOME/.cargo/env"
```

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

## Run

```bash
python3 -m http.server 8081
```

Then open:

```text
http://localhost:8081/web/
```

## How This Project Should Grow

The rule of thumb: keep the game logic independent from the browser. Rust should own gameplay, physics, entities, scores, rules and progression. JavaScript should only handle browser concerns: loading Wasm, keyboard input, canvas rendering, audio playback, browser storage and platform APIs.

Think of the project as four layers:

```text
web/                 Browser shell: input, render, audio, storage, Wasm loading
src/lib.rs           Wasm boundary: exported functions JS can call
src/game/            Game rules: player, enemies, bullets, score, levels, states
src/engine/          Reusable low-level systems: math, physics, collision, time
assets/              Game content: sprites, sounds, maps, config/data
```

The important dependency direction:

```text
web -> wasm exports -> game -> engine
```

Avoid this:

```text
game -> web
engine -> game
physics -> rendering
```

That separation is what later lets the same game run in browser, native desktop, Steam, or another renderer.

## Suggested Folder Structure

Current and planned structure:

```text
.
├── assets/
│   ├── audio/
│   │   ├── music/
│   │   └── sfx/
│   ├── data/
│   ├── maps/
│   ├── raw/
│   └── sprites/
├── src/
│   ├── engine/
│   │   ├── input.rs
│   │   ├── math.rs
│   │   └── physics.rs
│   ├── game/
│   │   ├── entities/
│   │   │   ├── bullet.rs
│   │   │   ├── enemy.rs
│   │   │   └── player.rs
│   │   └── world.rs
│   └── lib.rs
└── web/
    ├── platform/
    ├── rendering/
    ├── bootstrap.js
    ├── game-runtime.js
    └── index.html
```

When a domain grows, turn the single file into a folder. For example:

```text
src/game/entities/player.rs
```

can become:

```text
src/game/player/
├── mod.rs
├── movement.rs
├── shooting.rs
├── animation.rs
└── stats.rs
```

Do that only when the file starts becoming hard to scan. Small files are fine. A folder per tiny idea is also noise.

## Where To Put Gameplay Code

Use `src/game` for anything specific to this game.

Examples:

- Player behavior: `src/game/entities/player.rs`
- Enemy behavior: `src/game/entities/enemy.rs`
- Bullet behavior: `src/game/entities/bullet.rs`
- Score rules: `src/game/world.rs` now, later `src/game/scoring.rs`
- Spawn rules: `src/game/world.rs` now, later `src/game/spawning.rs`
- Game states: later `src/game/state.rs`
- Levels/waves: later `src/game/waves.rs` or `src/game/levels.rs`

Use `src/engine` only for reusable concepts that do not know what this game is.

Good `engine` modules:

- `math`: vectors, interpolation, random helpers.
- `physics`: bodies, integration, bounds, collision primitives.
- `time`: timers, cooldowns, fixed timestep helpers.
- `animation`: generic animation state/timers if kept renderer-agnostic.

Bad `engine` modules:

- `enemy_ai`: enemies belong to the game.
- `player_stats`: player belongs to the game.
- `score`: scoring rules belong to the game.

## Where To Put Rendering Code

Right now rendering lives in JavaScript:

```text
web/rendering/canvas-renderer.js
```

This file should draw the current game state, not decide game rules. It can ask Wasm for positions, radii, score and state, then draw them.

Good renderer responsibilities:

- Clear the screen.
- Draw background/grid.
- Draw player, bullets, enemies.
- Draw HUD.
- Draw game over overlay.
- Later: draw sprite frames instead of circles.

Bad renderer responsibilities:

- Deciding whether a bullet hit an enemy.
- Moving enemies.
- Updating score.
- Spawning waves.

Later, if rendering gets complex, split it:

```text
web/rendering/
├── canvas-renderer.js
├── draw-background.js
├── draw-entities.js
├── draw-hud.js
└── sprite-atlas.js
```

If the project moves to Bevy rendering, the same separation still applies: rendering systems draw state, gameplay systems decide rules.

## Where To Put Assets

Use `assets` for game content.

Recommended layout:

```text
assets/
├── raw/
│   ├── sprites/
│   ├── audio/
│   └── references/
├── sprites/
│   ├── player/
│   ├── enemies/
│   ├── bullets/
│   ├── effects/
│   └── ui/
├── audio/
│   ├── music/
│   └── sfx/
├── maps/
└── data/
```

Use `assets/raw` for editable source files:

- `.aseprite`
- `.psd`
- `.kra`
- `.blend`
- uncompressed audio sessions
- reference images

Use `assets/sprites`, `assets/audio`, `assets/maps` and `assets/data` for files the game actually loads.

Examples:

```text
assets/sprites/player/idle.png
assets/sprites/player/run.png
assets/sprites/enemies/basic.png
assets/audio/sfx/shoot.wav
assets/audio/sfx/hit.wav
assets/audio/music/arena-loop.ogg
assets/data/waves.json
```

For small assets, commit them to Git. For large source art/audio, consider Git LFS later.

## Where To Put Sound

Put sound files in:

```text
assets/audio/sfx/
assets/audio/music/
```

Use `sfx` for short sound effects:

- shooting
- enemy hit
- player damage
- game over
- menu select

Use `music` for longer loops:

- arena theme
- menu theme
- boss theme

Browser audio usually needs a user gesture before it can play, so audio should be started after a key press or click. In this project, JavaScript should own the browser audio API at first:

```text
web/audio/
├── audio-system.js
└── sound-bank.js
```

Rust can expose events like:

```text
shot fired
enemy killed
player died
```

Then JavaScript plays the actual sound. This keeps browser-specific audio details out of the game rules.

Later, for native builds, the platform audio implementation can change while the game events stay the same.

## How To Handle Animations

Start simple. Do not build a full animation engine on day one.

For sprites, use sprite sheets or separate frame images:

```text
assets/sprites/player/run_right.png
assets/sprites/player/run_left.png
assets/sprites/player/shoot_right.png
assets/sprites/player/shoot_left.png
```

A simple animation model:

```text
AnimationClip
- name
- frames
- frame_duration
- loops

Animator
- current_clip
- elapsed_time
- current_frame
```

Put game animation state near the domain:

```text
src/game/player/animation.rs
src/game/enemy/animation.rs
```

Put generic frame-timer code in:

```text
src/engine/animation.rs
```

Rendering should only ask: "which frame should I draw now?" It should not decide if the player is walking, shooting or dead. That decision belongs to the game/domain layer.

For this demo, a good next animation step would be:

1. Replace the player circle with a sprite.
2. Add `idle_left` and `idle_right`.
3. Add `run_left` and `run_right`.
4. Choose animation from velocity/facing.
5. Add a muzzle flash sprite when shooting.

## Do We Need A Database?

Not yet.

Most games do not start with a database. For this kind of browser game, use the lightest storage that solves the current problem.

Use in-memory state for things that reset when the page reloads:

- current score
- enemy positions
- bullets
- current run state

Use `localStorage` for tiny persistent values:

- best score
- settings
- volume
- unlocked debug flags

Use `IndexedDB` when save data becomes larger or structured:

- save slots
- inventory
- level progress
- generated maps
- replay data

Use a real backend/database only when you need online features:

- accounts
- cloud saves
- leaderboards
- multiplayer
- anti-cheat validation
- purchases outside a store platform

For native PC/Steam later, persistence usually becomes local files:

```text
save.json
settings.json
profile.dat
```

Steam Cloud can sync those files if configured. The game should have a storage abstraction so browser storage and native file storage can differ without rewriting gameplay.

Recommended future structure:

```text
web/storage/browser-storage.js
src/game/save_data.rs
src/game/settings.rs
```

For now, the best score is stored in Wasm memory for the current page session. The next step would be persisting it through `localStorage`.

## Game State

As the project grows, avoid scattered booleans like `is_dead`, `is_paused`, `is_menu`, `is_loading` everywhere. Prefer a single game state enum in Rust:

```rust
enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
```

Then the update loop becomes easier to reason about:

```text
MainMenu -> wait for start
Playing -> update player, enemies, bullets, collisions
Paused -> skip gameplay update, draw pause overlay
GameOver -> skip gameplay update, show score, wait for restart
```

The current demo already has the beginning of this with `game_over`. When more states appear, replace that boolean with a real enum.

## Data-Driven Content

Hardcoding numbers is fine for the first demo. As the game grows, move tunable values into data files.

Examples:

```text
assets/data/player.json
assets/data/enemies.json
assets/data/waves.json
```

Possible data:

```json
{
  "player": {
    "speed": 260,
    "radius": 18,
    "shootCooldown": 0.16
  }
}
```

Do this when values change often. Do not add a data pipeline before there is real content to tune.

## Suggested Next Milestones

1. Persist best score with `localStorage`.
2. Add player lives or health.
3. Add enemy waves instead of constant random spawn.
4. Replace circles with sprite assets.
5. Add shooting/hit/game-over sounds.
6. Add simple animation clips.
7. Add a start menu and pause state.
8. Add difficulty scaling over time.
9. Package a native desktop build.
10. Later, migrate rendering/game loop to Bevy if the prototype proves fun.

## Architecture Rules

- Gameplay rules live in Rust.
- Browser APIs live in `web`.
- Rendering reads state; it does not decide rules.
- Physics does not know about sprites.
- Audio reacts to game events; it does not decide rules.
- Assets are content, not logic.
- Keep the first version simple, but put files where a larger version would expect them.
