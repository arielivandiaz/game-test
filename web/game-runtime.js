export class GameRuntime {
  #wasm;
  #input;
  #renderer;
  #lastFrameTime = performance.now();

  constructor({ wasm, input, renderer }) {
    this.#wasm = wasm;
    this.#input = input;
    this.#renderer = renderer;

    this.#wasm.game_reset();
  }

  start() {
    requestAnimationFrame(this.#frame);
  }

  #frame = (now) => {
    const deltaSeconds = Math.min((now - this.#lastFrameTime) / 1000, 0.05);
    this.#lastFrameTime = now;
    this.#renderer.resizeToDisplay();
    this.#wasm.game_set_world_size(this.#renderer.width(), this.#renderer.height());

    if (this.#input.restarting()) {
      this.#wasm.game_restart();
    }

    const axis = this.#input.axis();
    this.#wasm.game_update(deltaSeconds, axis.x, axis.y, this.#input.shooting() ? 1 : 0);

    this.#renderer.render({
      world: {
        width: this.#wasm.game_world_width(),
        height: this.#wasm.game_world_height(),
      },
      player: {
        x: this.#wasm.player_x(),
        y: this.#wasm.player_y(),
        radius: this.#wasm.player_radius(),
        facing: this.#wasm.player_facing(),
      },
      bullets: this.#readBullets(),
      enemies: this.#readEnemies(),
      score: this.#wasm.score(),
      bestScore: this.#wasm.best_score(),
      gameOver: Boolean(this.#wasm.game_over()),
    });

    requestAnimationFrame(this.#frame);
  };

  #readBullets() {
    const bullets = [];
    const capacity = this.#wasm.bullet_capacity();

    for (let index = 0; index < capacity; index += 1) {
      if (!this.#wasm.bullet_active(index)) {
        continue;
      }

      bullets.push({
        x: this.#wasm.bullet_x(index),
        y: this.#wasm.bullet_y(index),
        radius: this.#wasm.bullet_radius(index),
      });
    }

    return bullets;
  }

  #readEnemies() {
    const enemies = [];
    const capacity = this.#wasm.enemy_capacity();

    for (let index = 0; index < capacity; index += 1) {
      if (!this.#wasm.enemy_active(index)) {
        continue;
      }

      enemies.push({
        x: this.#wasm.enemy_x(index),
        y: this.#wasm.enemy_y(index),
        radius: this.#wasm.enemy_radius(index),
      });
    }

    return enemies;
  }
}
