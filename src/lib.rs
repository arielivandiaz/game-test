mod engine;
mod game;

use core::cell::UnsafeCell;

use engine::input::InputAxis;
use game::world::GameWorld;

struct GameStorage(UnsafeCell<GameWorld>);

unsafe impl Sync for GameStorage {}

static GAME: GameStorage = GameStorage(UnsafeCell::new(GameWorld::new()));

fn game() -> &'static mut GameWorld {
    unsafe { &mut *GAME.0.get() }
}

#[no_mangle]
pub extern "C" fn game_reset() {
    *game() = GameWorld::new();
}

#[no_mangle]
pub extern "C" fn game_restart() {
    game().reset_run();
}

#[no_mangle]
pub extern "C" fn game_set_world_size(width: f32, height: f32) {
    game().resize(width, height);
}

#[no_mangle]
pub extern "C" fn game_update(delta_seconds: f32, axis_x: f32, axis_y: f32, shooting: i32) {
    let input = InputAxis::new(axis_x, axis_y);
    game().update(delta_seconds, input, shooting != 0);
}

#[no_mangle]
pub extern "C" fn game_world_width() -> f32 {
    game().width()
}

#[no_mangle]
pub extern "C" fn game_world_height() -> f32 {
    game().height()
}

#[no_mangle]
pub extern "C" fn player_x() -> f32 {
    game().player().position.x
}

#[no_mangle]
pub extern "C" fn player_y() -> f32 {
    game().player().position.y
}

#[no_mangle]
pub extern "C" fn player_radius() -> f32 {
    game().player().radius
}

#[no_mangle]
pub extern "C" fn player_facing() -> f32 {
    game().player_facing()
}

#[no_mangle]
pub extern "C" fn bullet_capacity() -> usize {
    game::world::MAX_BULLETS
}

#[no_mangle]
pub extern "C" fn bullet_active(index: usize) -> i32 {
    game().bullet(index).active() as i32
}

#[no_mangle]
pub extern "C" fn bullet_x(index: usize) -> f32 {
    game().bullet(index).body().position.x
}

#[no_mangle]
pub extern "C" fn bullet_y(index: usize) -> f32 {
    game().bullet(index).body().position.y
}

#[no_mangle]
pub extern "C" fn bullet_radius(index: usize) -> f32 {
    game().bullet(index).body().radius
}

#[no_mangle]
pub extern "C" fn enemy_capacity() -> usize {
    game::world::MAX_ENEMIES
}

#[no_mangle]
pub extern "C" fn enemy_active(index: usize) -> i32 {
    game().enemy(index).active() as i32
}

#[no_mangle]
pub extern "C" fn enemy_x(index: usize) -> f32 {
    game().enemy(index).body().position.x
}

#[no_mangle]
pub extern "C" fn enemy_y(index: usize) -> f32 {
    game().enemy(index).body().position.y
}

#[no_mangle]
pub extern "C" fn enemy_radius(index: usize) -> f32 {
    game().enemy(index).body().radius
}

#[no_mangle]
pub extern "C" fn score() -> u32 {
    game().score()
}

#[no_mangle]
pub extern "C" fn best_score() -> u32 {
    game().best_score()
}

#[no_mangle]
pub extern "C" fn game_over() -> i32 {
    game().game_over() as i32
}
