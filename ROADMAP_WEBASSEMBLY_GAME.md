# Roadmap para crear un videojuego 2.5D en WebAssembly con Rust y Bevy

## Objetivo

Crear un videojuego que corra en navegador usando WebAssembly como tecnología central, desarrollado en **Rust** con **Bevy**, con una dirección visual **2.5D**: mundo, cámara y profundidad 3D, pero personajes, props, efectos y buena parte del arte construidos con sprites/assets 2D.

La meta inicial no debería ser "hacer el motor perfecto", sino llegar rápido a un prototipo jugable en navegador y después endurecer la arquitectura.

## Visión del juego

El concepto técnico/artístico base:

- Mundo 3D simple: piso, profundidad, coordenadas `x/y/z`, cámara con perspectiva u ortográfica inclinada.
- Assets 2D: personajes, enemigos, pickups, proyectiles, partículas y UI como sprites.
- Estética 2.5D: sensación espacial, sombras simples, parallax, capas y composición cinematográfica.
- Gameplay inicialmente acotado: una arena, habitación, corredor o pequeño mapa donde se pruebe el loop central.
- Build web desde temprano: el juego tiene que vivirse en navegador, no solo correr localmente.

Inspiraciones posibles:

- Sprites 2D en mundo 3D.
- Dioramas con cámara fija o semi fija.
- Arena shooter con cámara isométrica/perspectiva.
- RPG táctico o exploración ligera.
- Beat'em up con profundidad.
- Estética tipo paper cutout, pixel art con profundidad o billboard sprites.

## Recomendación corta

Para este proyecto, la recomendación queda mucho más clara:

**Rust + Bevy + WebAssembly**

Por qué Bevy encaja:

- ECS natural para separar jugador, enemigos, cámara, sprites, colisiones, estados y efectos.
- Buen modelo mental para mezclar 3D y 2D.
- Render moderno y estructura escalable.
- Rust obliga a diseñar datos y ownership con disciplina, algo muy sano para juegos medianos.
- WebAssembly permite publicar builds jugables sin instalador.

El foco ya no sería comparar engines durante semanas. El foco sería validar rápido una pregunta: **se siente cool este 2.5D en navegador?**

## Qué es WebAssembly en este contexto

WebAssembly no reemplaza al navegador. Es un formato binario portable que permite ejecutar código compilado, normalmente desde Rust, C, C++ u otros lenguajes, con rendimiento cercano al nativo. En juegos web, suele convivir con JavaScript para cargar el módulo, hablar con APIs del navegador y empaquetar assets.

En la práctica vas a trabajar con:

- Un lenguaje fuente: Rust, C, C++, C#, AssemblyScript, etc.
- Un compilador hacia `wasm32`.
- Glue code JavaScript generado o mínimo.
- Canvas, WebGL o WebGPU para gráficos.
- Audio web.
- Un bundler/dev server.
- Un pipeline de assets.

## Comparativa de stacks

| Stack | Cuándo elegirlo | Ventajas | Costos |
| --- | --- | --- | --- |
| Rust + Bevy | Camino recomendado para este proyecto 2.5D | ECS, 2D/3D, arquitectura moderna, buen aprendizaje | Curva mayor, builds web a cuidar |
| Rust + Macroquad | Prototipo 2D rápido | Simple, portable, directo | Menos ideal si quieres mundo 3D con estructura |
| C/C++ + Emscripten + raylib | Vienes de C/C++, quieres control y bajo nivel | Toolchain madura, mentalidad nativa | Integración web y builds pueden ser ásperos |
| C/C++ + SDL2 + Emscripten | Portar o construir algo más clásico | SDL es conocida, portable | Más plumbing |
| Godot Web Export | Quieres terminar un juego con editor visual | Productivo, escenas, editor, assets | Aprendes menos WebAssembly internamente |
| AssemblyScript | Experimentos pequeños con sintaxis TS-like | Familiar si vienes de TS | Ecosistema de game dev menor |
| Unity WebGL | Proyecto grande con tooling comercial | Muy productivo, mucho ecosistema | Builds pesados, menos control fino |

## Stack inicial propuesto

**Rust + Bevy**

Herramientas:

- `rustup`
- `cargo`
- target `wasm32-unknown-unknown`
- `trunk` o workflow recomendado por el engine
- Bevy
- Dev server local
- GitHub Pages, Netlify, Vercel, itch.io o Cloudflare Pages para publicar

Plugins/crates a evaluar cuando el prototipo lo pida:

- `bevy_asset_loader` para estados de carga.
- `bevy_kira_audio` o audio nativo de Bevy, según madurez y necesidades.
- `avian3d` o `rapier3d`, solo si las colisiones simples dejan de alcanzar.
- `bevy_ecs_ldtk` o integración con Tiled/LDtk si el diseño pide niveles authorables.
- `iyes_perf_ui` o herramientas similares para métricas en pantalla.

Decisión importante:

- Evitar física 3D compleja al inicio.
- Usar coordenadas 3D para posición.
- Representar personajes como sprites/billboards en planos 3D.
- Mantener el primer mapa pequeño.
- Priorizar cámara, lectura visual y feel antes que sistemas grandes.

## Decisión recomendada para empezar

Para este proyecto haría esto:

1. Arrancar directamente en **Bevy**.
2. Construir una escena 3D mínima con piso, cámara y un sprite/personaje 2D.
3. Validar el truco 2.5D antes de meter contenido.
4. Mantener el primer juego como arena o escena pequeña.
5. Publicar una build web desde la primera semana.

La publicación temprana importa muchísimo: WebAssembly no es solo compilar, también es carga de assets, tamaño de bundle, compatibilidad, input, audio y hosting.

## Roadmap por fases

### Fase 0: Definir el juego

Duración sugerida: 1 o 2 días.

Entregables:

- Documento de una página con la idea.
- Género.
- Loop principal.
- Controles.
- Una referencia visual.
- Una mecánica central.
- Alcance máximo para el primer prototipo.

Buenas ideas para primer juego:

- Arena shooter 2.5D con sprites que miran a cámara.
- Exploración de una habitación/diorama.
- Beat'em up con profundidad.
- Roguelite de una sala con cámara inclinada.
- Puzzle táctico sobre grilla 3D simple.
- Acción estilo arcade con enemigos billboard.

Evitar al principio:

- Multiplayer online.
- Mundo abierto.
- Editor de niveles completo.
- 3D complejo con modelos, rigs y animación esquelética.
- Física avanzada.
- Inventarios enormes.
- Cámaras dinámicas demasiado inteligentes.
- Muchísimos tipos de sprites o animaciones antes de validar el look.

### Fase 1: Ambiente y primer render

Duración sugerida: 2 o 3 días.

Objetivo:

Lograr que algo se vea en el navegador.

Tareas:

- Instalar Rust con `rustup`.
- Crear repo base.
- Crear proyecto Bevy.
- Crear una escena 3D mínima.
- Agregar un piso/plano.
- Agregar una cámara con ángulo 2.5D.
- Agregar un sprite/personaje 2D dentro del mundo 3D.
- Probar billboard básico: sprite orientado a cámara o a una dirección fija según estilo.
- Compilar a WebAssembly.
- Servir localmente con HTTP, no abrir el archivo directo desde disco.
- Confirmar que corre en Chrome, Firefox y Safari si tienes acceso.

Checklist:

- El juego abre en navegador.
- Hay un canvas visible.
- Hay profundidad visual.
- Se entiende la escala del personaje en el mundo.
- Se puede reconstruir con un solo comando.
- Hay una carpeta de assets.
- Hay README mínimo con comandos.

### Fase 2: Game loop jugable

Duración sugerida: 1 semana.

Objetivo:

Tener una experiencia mínima que ya sea "juego", aunque sea fea.

Tareas:

- Input de teclado y/o mouse.
- Movimiento del jugador en plano 3D.
- Cámara fija, cámara follow o cámara isométrica.
- Sprite del jugador con orientación legible.
- Entidades básicas.
- Colisiones simples.
- Condición de ganar/perder.
- Reinicio rápido.
- HUD mínimo.
- Prueba de profundidad: enemigos delante/detrás, oclusión simple o layering.

Entregable:

- Una build web donde puedas jugar 30 segundos y entender el loop.

Regla útil:

Si el prototipo no es divertido con cuadrados y círculos, los sprites no lo van a salvar.

### Fase 3: Arquitectura mínima

Duración sugerida: 1 semana.

Objetivo:

Ordenar el proyecto antes de que crezca.

Módulos sugeridos:

- `game`: estado global y transiciones.
- `player`: lógica del jugador.
- `enemy`: enemigos o entidades activas.
- `world`: piso, límites, zonas, obstáculos y niveles.
- `camera`: cámara 2.5D, follow, shake y zoom.
- `physics`: colisiones simples.
- `render`: materiales, sprites 3D, billboards y efectos.
- `assets`: carga y referencias.
- `ui`: HUD, menús, pausa.
- `audio`: música y efectos.
- `combat` o `interaction`: daño, hits, pickups o acciones principales.

Estados mínimos:

- `Loading`
- `MainMenu`
- `Playing`
- `Paused`
- `GameOver`

### Fase 4: Pipeline de assets

Duración sugerida: 3 a 5 días.

Objetivo:

No bloquearte por arte, audio o formatos.

Herramientas útiles:

- Aseprite, LibreSprite, Krita o Photoshop para sprites.
- Blender solo para blockouts simples, referencias de cámara o props muy básicos si hiciera falta.
- Tiled, LDtk o escenas Bevy si el mapa nace como grilla/zonas.
- Audacity o Reaper para audio.
- TexturePacker o scripts propios si necesitas atlas.
- `just`, `make` o scripts npm para comandos repetibles.

Decisiones:

- Formato de imágenes: `png` o `webp`.
- Formato de audio: `ogg` y/o `mp3`, según soporte y framework.
- Tamaño base de sprites: por ejemplo 32, 64 o 128 px por unidad visual.
- Estilo de orientación: billboards siempre mirando a cámara, sprites con 4/8 direcciones, o sprites fijos en el mundo.
- Sombras: blobs simples, planos translúcidos o sombras reales si el costo lo permite.
- Convención de nombres.
- Carpeta `assets/raw` y `assets/dist` si vas a procesar assets.

### Fase 5: Contenido vertical slice

Duración sugerida: 2 a 4 semanas.

Objetivo:

Construir una porción terminada del juego.

Debe incluir:

- Una escena o nivel completo.
- Menú inicial.
- UI usable.
- Música o ambientación básica.
- Efectos de sonido.
- Cámara 2.5D ya definida.
- Sprite pipeline definido para jugador y enemigos.
- Una regla clara de profundidad/oclusiones.
- Feedback visual al recibir daño, ganar puntos o interactuar.
- Pantalla de resultado.
- Restart sin recargar la página.

Esto es más importante que tener 20 mecánicas incompletas.

### Fase 6: Rendimiento y tamaño

Duración sugerida: 1 semana.

Objetivo:

Hacer que cargue rápido y corra fluido.

Medir:

- FPS.
- Tiempo de carga inicial.
- Tamaño de `.wasm`.
- Tamaño total de assets.
- Uso de memoria.
- Cantidad de sprites/entidades visibles.
- Costo de materiales, transparencias y orden de render.
- Jank al cargar audio/texturas.

Optimizar:

- Build release.
- Compresión Brotli/Gzip en hosting.
- Reducir assets enormes.
- Lazy loading si aplica.
- Evitar asignaciones por frame.
- Reusar buffers/entidades.
- Reducir overdraw por sprites transparentes si empieza a doler.
- Mantener geometría 3D simple.
- Perfilar en DevTools.

### Fase 7: Publicación web

Duración sugerida: 2 o 3 días.

Opciones:

- itch.io para compartir rápido con comunidad gamer.
- GitHub Pages para builds estáticas simples.
- Netlify/Vercel/Cloudflare Pages para deploy automático.

Checklist:

- Build release.
- MIME types correctos para `.wasm`.
- Compresión activada.
- Pantalla de carga.
- Mensaje si el browser no soporta lo necesario.
- Controles visibles.
- Página simple con canvas centrado.

### Fase 8: Iteración y comunidad

Duración sugerida: continua.

Objetivo:

No desarrollar en una cueva durante meses.

Ritmo sugerido:

- Publicar una build cada semana.
- Pedir feedback concreto: dificultad, claridad, controles.
- Registrar bugs en issues.
- Grabar GIFs cortos.
- Mantener changelog.
- Separar backlog en `Now`, `Next`, `Later`.

## Plan de 8 semanas

| Semana | Objetivo | Entregable |
| --- | --- | --- |
| 1 | Setup Bevy + escena 2.5D mínima | Build web con piso, cámara y sprite en mundo 3D |
| 2 | Movimiento + cámara + colisiones simples | Prototipo jugable con profundidad |
| 3 | Enemigos + loop de combate/interacción | Arena con ganar/perder |
| 4 | Arquitectura + estados | Menú, loading, pausa, game over |
| 5 | Pipeline visual 2D/3D | Sprites, sombras, efectos y lectura visual definidos |
| 6 | Vertical slice | Una escena o nivel completo |
| 7 | Optimización web | Build release liviana y fluida |
| 8 | Publicación y feedback | Versión pública jugable |

## Conocimientos que conviene estudiar

### WebAssembly

- Qué es un módulo `.wasm`.
- Cómo se carga desde JavaScript.
- Diferencia entre `wasm32-unknown-unknown` y targets con Emscripten.
- Costos de cruzar la frontera JS/Wasm.
- Memoria lineal.
- Limitaciones de APIs del navegador.

### Game development

- Game loop.
- Delta time.
- Fixed timestep vs variable timestep.
- Input buffering.
- Colisiones.
- ECS si usas Bevy.
- Coordenadas 3D con gameplay sobre plano.
- Billboards y sprites en mundo 3D.
- Cámara isométrica, ortográfica inclinada o perspectiva controlada.
- Sorting/oclusiones de sprites transparentes.
- Sombras simples para anclar personajes al piso.
- Spatial partitioning si hay muchas entidades.
- Estados de juego.
- Asset management.

### Web

- Canvas.
- WebGL/WebGPU según engine.
- Audio unlock por interacción de usuario.
- Hosting estático.
- Cache headers.
- DevTools performance.
- Compatibilidad mobile.

## Riesgos comunes

- Querer hacer engine antes que juego.
- Convertir el 2.5D en un proyecto 3D completo demasiado pronto.
- Que los sprites "floten" por falta de sombras/contacto con el piso.
- Que la cámara se vea cool pero haga difícil jugar.
- Perder lectura visual por transparencias, escala u oclusión.
- Subestimar audio en browser.
- Cargar assets gigantes.
- No probar Safari hasta el final.
- Depender de APIs web no disponibles en todos los browsers objetivo.
- Cruzar demasiadas llamadas JS/Wasm por frame.
- No tener builds públicas tempranas.

## Primeros comandos orientativos

Para Rust:

```bash
cargo new wasm-game
cd wasm-game
```

Para Bevy:

```bash
cargo add bevy
```

Para preparar target WebAssembly:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

No tomes estos comandos como plantilla final cerrada. El comando exacto de build depende de la versión de Bevy, del empaquetado elegido y de cómo se sirvan los assets.

## Primer milestone recomendado

Hacer un mini juego llamado, por ejemplo, `wasm-25d-arena`:

- Pantalla 16:9.
- Piso 3D simple con grilla o textura.
- Cámara 2.5D fija o follow.
- Jugador como sprite 2D en mundo 3D.
- Movimiento WASD.
- Apuntar con mouse.
- Enemigos como sprites/billboards que persiguen.
- Disparo básico.
- Sombras simples bajo personajes.
- Obstáculos bajos para probar profundidad.
- Score.
- Game over al tocar enemigo.
- Restart con tecla.
- Build web pública.

Este prototipo fuerza las piezas importantes: input, loop, entidades, colisión, cámara 2.5D, sprites en 3D, UI, audio opcional, export y deploy.

## Decisiones técnicas clave

### Cámara

Opciones recomendadas:

- **Ortográfica inclinada:** look táctico/isométrico, lectura muy limpia.
- **Perspectiva suave:** más dramatismo y profundidad, pero puede complicar escala y puntería.
- **Cámara fija por escena:** excelente para un primer vertical slice.
- **Follow camera:** mejor para acción, requiere más pulido.

Para empezar: **ortográfica inclinada o perspectiva muy controlada**.

### Sprites en 3D

Opciones:

- Billboard siempre mirando a cámara.
- Sprite con 4 direcciones.
- Sprite con 8 direcciones.
- Sprite fijo en el mundo para props y decoración.

Para empezar: **billboard para enemigos y 4 direcciones para jugador**, si el scope lo permite. Si no, billboard para todo.

### Colisiones

Empezar simple:

- Posiciones 3D, pero colisiones en `x/z`.
- Círculos o AABB sobre el piso.
- Altura visual separada de la lógica.

Evitar al inicio:

- Rigid bodies 3D complejos.
- Escaleras, rampas y pisos múltiples.
- Hitboxes ultra precisos.

### Arte

Dirección sugerida:

- Pixel art o sprites pintados con alto contraste.
- Paleta limitada, pero no monocromática.
- Sombras/contacto para anclar al piso.
- Efectos 2D: slash, muzzle flash, pickup sparkle, hit flash.
- Props planos con algo de volumen falso.

## Fuentes útiles

- MDN WebAssembly: https://developer.mozilla.org/en-US/docs/WebAssembly
- Rust and WebAssembly Book: https://rustwasm.github.io/docs/book/
- wasm-bindgen Guide: https://wasm-bindgen.github.io/wasm-bindgen/
- Bevy Learn: https://bevy.org/learn/
- Emscripten docs: https://emscripten.org/docs/
- Godot Web export: https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_web.html
- AssemblyScript docs: https://www.assemblyscript.org/
- raylib: https://www.raylib.com/

## Siguiente paso

El siguiente paso concreto:

1. Crear el proyecto Bevy.
2. Lograr build nativa local.
3. Lograr build web.
4. Crear escena con piso, cámara y un sprite.
5. Mover el sprite con WASD.
6. Ajustar cámara hasta que el look 2.5D se sienta bien.

Mi sugerencia: la primera sesión del proyecto debería terminar con una escena mínima pero emocionante: un personaje 2D caminando sobre un mundo 3D simple dentro del navegador. Si eso se siente bien, el proyecto tiene pulso.
