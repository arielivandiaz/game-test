import { GameRuntime } from "./game-runtime.js";
import { KeyboardInput } from "./platform/keyboard-input.js";
import { loadWasm } from "./platform/wasm-loader.js";
import { CanvasRenderer } from "./rendering/canvas-renderer.js";

const canvas = document.querySelector("#game");
const wasm = await loadWasm("../target/wasm32-unknown-unknown/release/wasm_game.wasm");

const input = new KeyboardInput();
const renderer = new CanvasRenderer(canvas);
const runtime = new GameRuntime({ wasm, input, renderer });

runtime.start();
