use crate::engine::{input::InputAxis, math::Vec2, physics::PhysicsWorld};

use super::entities::{bullet::Bullet, enemy::Enemy, player::Player};

const WORLD_WIDTH: f32 = 960.0;
const WORLD_HEIGHT: f32 = 540.0;
pub const MAX_BULLETS: usize = 64;
pub const MAX_ENEMIES: usize = 48;

pub struct GameWorld {
    physics: PhysicsWorld,
    player: Player,
    bullets: [Bullet; MAX_BULLETS],
    enemies: [Enemy; MAX_ENEMIES],
    shoot_cooldown: f32,
    enemy_spawn_timer: f32,
    random_seed: u32,
    score: u32,
    best_score: u32,
    game_over: bool,
}

impl GameWorld {
    pub const fn new() -> Self {
        Self {
            physics: PhysicsWorld::new(WORLD_WIDTH, WORLD_HEIGHT),
            player: Player::new(Vec2::new(WORLD_WIDTH * 0.5, WORLD_HEIGHT * 0.5)),
            bullets: [Bullet::inactive(); MAX_BULLETS],
            enemies: [Enemy::inactive(); MAX_ENEMIES],
            shoot_cooldown: 0.0,
            enemy_spawn_timer: 0.6,
            random_seed: 0x1234_abcd,
            score: 0,
            best_score: 0,
            game_over: false,
        }
    }

    pub fn reset_run(&mut self) {
        let width = self.width();
        let height = self.height();
        let best_score = self.best_score;

        *self = Self::new();
        self.best_score = best_score;
        self.resize(width, height);
    }

    pub fn update(&mut self, delta_seconds: f32, input: InputAxis, shooting: bool) {
        if self.game_over {
            return;
        }

        self.tick_timers(delta_seconds);
        self.player.update(delta_seconds, input, &self.physics);

        if shooting && self.shoot_cooldown <= 0.0 {
            self.spawn_bullet();
            self.shoot_cooldown = 0.16;
        }

        if self.enemy_spawn_timer <= 0.0 {
            self.spawn_enemy();
            self.enemy_spawn_timer = 0.45;
        }

        for bullet in &mut self.bullets {
            bullet.update(delta_seconds, &self.physics);
        }

        for enemy in &mut self.enemies {
            enemy.update(delta_seconds, &self.physics);
        }

        self.resolve_bullet_enemy_collisions();
        self.resolve_player_enemy_collisions();
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let width = width.max(320.0);
        let height = height.max(240.0);
        self.physics.resize(width, height);
    }

    pub fn width(&self) -> f32 {
        self.physics.bounds().x
    }

    pub fn height(&self) -> f32 {
        self.physics.bounds().y
    }

    pub fn player(&self) -> crate::engine::physics::Body {
        self.player.body()
    }

    pub fn player_facing(&self) -> f32 {
        self.player.facing()
    }

    pub fn bullet(&self, index: usize) -> Bullet {
        self.bullets.get(index).copied().unwrap_or(Bullet::inactive())
    }

    pub fn enemy(&self, index: usize) -> Enemy {
        self.enemies.get(index).copied().unwrap_or(Enemy::inactive())
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn best_score(&self) -> u32 {
        self.best_score
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    fn tick_timers(&mut self, delta_seconds: f32) {
        self.shoot_cooldown = (self.shoot_cooldown - delta_seconds).max(0.0);
        self.enemy_spawn_timer -= delta_seconds;
    }

    fn spawn_bullet(&mut self) {
        let player = self.player.body();
        let direction = Vec2::new(self.player.facing(), 0.0);
        let position = Vec2::new(
            player.position.x + direction.x * (player.radius + 8.0),
            player.position.y,
        );

        if let Some(bullet) = self.bullets.iter_mut().find(|bullet| !bullet.active()) {
            bullet.spawn(position, direction);
        }
    }

    fn spawn_enemy(&mut self) {
        let position = self.random_edge_position();
        let target = self.player.body().position;

        if let Some(enemy) = self.enemies.iter_mut().find(|enemy| !enemy.active()) {
            enemy.spawn(position, target);
        }
    }

    fn random_edge_position(&mut self) -> Vec2 {
        let width = self.width();
        let height = self.height();
        let edge = (self.random_unit() * 4.0) as u32;
        let t = self.random_unit();
        let margin = 32.0;

        match edge {
            0 => Vec2::new(t * width, -margin),
            1 => Vec2::new(width + margin, t * height),
            2 => Vec2::new(t * width, height + margin),
            _ => Vec2::new(-margin, t * height),
        }
    }

    fn random_unit(&mut self) -> f32 {
        self.random_seed = self
            .random_seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);

        ((self.random_seed >> 8) as f32) / ((u32::MAX >> 8) as f32)
    }

    fn resolve_bullet_enemy_collisions(&mut self) {
        for bullet in &mut self.bullets {
            if !bullet.active() {
                continue;
            }

            for enemy in &mut self.enemies {
                if !enemy.active() {
                    continue;
                }

                let bullet_body = bullet.body();
                let enemy_body = enemy.body();
                let radius = bullet_body.radius + enemy_body.radius;

                if bullet_body.position.distance_squared(enemy_body.position) <= radius * radius {
                    bullet.deactivate();
                    enemy.deactivate();
                    self.score += 1;
                    break;
                }
            }
        }
    }

    fn resolve_player_enemy_collisions(&mut self) {
        let player = self.player.body();

        for enemy in &mut self.enemies {
            if !enemy.active() {
                continue;
            }

            let enemy_body = enemy.body();
            let radius = player.radius + enemy_body.radius;

            if player.position.distance_squared(enemy_body.position) <= radius * radius {
                self.game_over = true;
                self.best_score = self.best_score.max(self.score);
                break;
            }
        }
    }
}
