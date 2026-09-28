const KEY_BINDINGS = {
  ArrowLeft: { x: -1, y: 0 },
  ArrowRight: { x: 1, y: 0 },
  ArrowUp: { x: 0, y: -1 },
  ArrowDown: { x: 0, y: 1 },
  KeyA: { x: -1, y: 0 },
  KeyD: { x: 1, y: 0 },
  KeyW: { x: 0, y: -1 },
  KeyS: { x: 0, y: 1 },
};

const ACTION_BINDINGS = new Set(["Space", "Enter"]);

export class KeyboardInput {
  #pressed = new Set();

  constructor() {
    window.addEventListener("keydown", (event) => {
      if (!KEY_BINDINGS[event.code] && !ACTION_BINDINGS.has(event.code)) {
        return;
      }

      event.preventDefault();
      this.#pressed.add(event.code);
    });

    window.addEventListener("keyup", (event) => {
      if (!KEY_BINDINGS[event.code] && !ACTION_BINDINGS.has(event.code)) {
        return;
      }

      event.preventDefault();
      this.#pressed.delete(event.code);
    });

    window.addEventListener("blur", () => {
      this.#pressed.clear();
    });
  }

  axis() {
    let x = 0;
    let y = 0;

    for (const code of this.#pressed) {
      const binding = KEY_BINDINGS[code];

      if (!binding) {
        continue;
      }

      x += binding.x;
      y += binding.y;
    }

    return { x, y };
  }

  shooting() {
    return this.#pressed.has("Space");
  }

  restarting() {
    return this.#pressed.has("Enter");
  }
}
