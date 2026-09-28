export class CanvasRenderer {
  #canvas;
  #context;

  constructor(canvas) {
    this.#canvas = canvas;
    this.#context = canvas.getContext("2d");
  }

  width() {
    return this.#canvas.width;
  }

  height() {
    return this.#canvas.height;
  }

  resizeToDisplay() {
    const pixelRatio = window.devicePixelRatio || 1;
    const width = Math.floor(this.#canvas.clientWidth * pixelRatio);
    const height = Math.floor(this.#canvas.clientHeight * pixelRatio);

    if (this.#canvas.width === width && this.#canvas.height === height) {
      return;
    }

    this.#canvas.width = width;
    this.#canvas.height = height;
  }

  render({ world, player, bullets, enemies, score, bestScore, gameOver }) {
    this.#drawBackground(world);
    this.#drawGrid(world);
    this.#drawBullets(bullets);
    this.#drawEnemies(enemies);
    this.#drawPlayer(player);
    this.#drawHud(score, bestScore);

    if (gameOver) {
      this.#drawGameOver(world, score, bestScore);
    }
  }

  #drawBackground(world) {
    const ctx = this.#context;
    ctx.clearRect(0, 0, world.width, world.height);
    ctx.fillStyle = "#111318";
    ctx.fillRect(0, 0, world.width, world.height);
  }

  #drawGrid(world) {
    const ctx = this.#context;
    const step = 48;

    ctx.save();
    ctx.strokeStyle = "rgba(244, 240, 232, 0.08)";
    ctx.lineWidth = 1;

    for (let x = 0; x <= world.width; x += step) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, world.height);
      ctx.stroke();
    }

    for (let y = 0; y <= world.height; y += step) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(world.width, y);
      ctx.stroke();
    }

    ctx.restore();
  }

  #drawPlayer(player) {
    const ctx = this.#context;

    ctx.save();
    ctx.shadowColor = "rgba(159, 195, 255, 0.45)";
    ctx.shadowBlur = 18;
    ctx.fillStyle = "#9fc3ff";
    ctx.beginPath();
    ctx.arc(player.x, player.y, player.radius, 0, Math.PI * 2);
    ctx.fill();

    ctx.shadowBlur = 0;
    ctx.strokeStyle = "#f4f0e8";
    ctx.lineWidth = 3;
    ctx.stroke();

    ctx.fillStyle = "#f4f0e8";
    ctx.beginPath();
    ctx.moveTo(player.x + player.facing * (player.radius + 16), player.y);
    ctx.lineTo(player.x + player.facing * (player.radius + 3), player.y - 7);
    ctx.lineTo(player.x + player.facing * (player.radius + 3), player.y + 7);
    ctx.closePath();
    ctx.fill();
    ctx.restore();
  }

  #drawBullets(bullets) {
    const ctx = this.#context;

    ctx.save();
    ctx.fillStyle = "#f8d66d";
    ctx.shadowColor = "rgba(248, 214, 109, 0.5)";
    ctx.shadowBlur = 10;

    for (const bullet of bullets) {
      ctx.beginPath();
      ctx.arc(bullet.x, bullet.y, bullet.radius, 0, Math.PI * 2);
      ctx.fill();
    }

    ctx.restore();
  }

  #drawEnemies(enemies) {
    const ctx = this.#context;

    ctx.save();
    ctx.fillStyle = "#ff6b6b";
    ctx.strokeStyle = "#3b161a";
    ctx.lineWidth = 2;

    for (const enemy of enemies) {
      ctx.beginPath();
      ctx.arc(enemy.x, enemy.y, enemy.radius, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
    }

    ctx.restore();
  }

  #drawHud(score, bestScore) {
    const ctx = this.#context;

    ctx.save();
    ctx.fillStyle = "rgba(17, 19, 24, 0.68)";
    ctx.fillRect(20, 20, 190, 68);
    ctx.fillStyle = "#f4f0e8";
    ctx.font = "700 18px system-ui, sans-serif";
    ctx.textBaseline = "middle";
    ctx.fillText(`Score ${score}`, 36, 42);
    ctx.fillStyle = "#9fc3ff";
    ctx.font = "700 14px system-ui, sans-serif";
    ctx.fillText(`Best ${bestScore}`, 36, 68);
    ctx.restore();
  }

  #drawGameOver(world, score, bestScore) {
    const ctx = this.#context;
    const centerX = world.width / 2;
    const centerY = world.height / 2;

    ctx.save();
    ctx.fillStyle = "rgba(17, 19, 24, 0.72)";
    ctx.fillRect(0, 0, world.width, world.height);

    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.fillStyle = "#f4f0e8";
    ctx.font = "800 64px system-ui, sans-serif";
    ctx.fillText("Game Over", centerX, centerY - 70);

    ctx.fillStyle = "#9fc3ff";
    ctx.font = "700 24px system-ui, sans-serif";
    ctx.fillText(`Score ${score}   Best ${bestScore}`, centerX, centerY);

    ctx.fillStyle = "#f4f0e8";
    ctx.font = "700 18px system-ui, sans-serif";
    ctx.fillText("Press Enter to restart", centerX, centerY + 54);
    ctx.restore();
  }
}
