# Telegram Tarot Mini App

A beginner-friendly Telegram Mini App built with [Yew](https://github.com/yewstack/yew).
Users can draw 1, 3, or 5 tarot cards, see upright/reversed meanings, and the
interface automatically adapts to Telegram's dark or light theme. The codebase
is intentionally tiny and well-commented so you can explore Rust, Yew, and the
Telegram WebApp SDK step by step.

## Quick start

1. Install the Rust toolchain and the WebAssembly target.
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Install [Trunk](https://trunkrs.dev/) to compile and serve the Yew app.
   ```bash
   cargo install trunk
   ```
3. Drop your 78 `.webp` tarot images inside the `assets/` directory. Follow the
   slug-based naming convention: `ace-of-cups.webp`, `the-fool.webp`, etc. The
   slug values live in [`src/deck/cards.rs`](src/deck/cards.rs).
4. Start the development server.
   ```bash
   trunk serve --open
   ```

Trunk compiles the Rust code to WebAssembly, serves the `index.html`, and
watches the project for live reloads.

## Project structure

- [`src/app.rs`](src/app.rs) – the root Yew component with a tiny `Reading`
  state machine and Telegram theme binding.
- [`src/deck/`](src/deck) – split into `mod.rs` (types & helpers) and
  `cards.rs` (static data). Update the list to add or edit cards.
- [`src/ui/`](src/ui) – small, well-documented UI components (`DrawControls`
  and `CardGrid`).
- [`src/telegram.rs`](src/telegram.rs) – glue around the
  [`telegram-webapp-sdk`](https://crates.io/crates/telegram-webapp-sdk) crate
  that initialises the Mini App context, mirrors Telegram theme tokens, and
  exposes `use_main_button`/`use_back_button` hooks for native controls.
- [`static/styles.css`](static/styles.css) – global styling shared across the
  app (copied into the `dist/` folder by Trunk).
- [`Trunk.toml`](Trunk.toml) & [`index.html`](index.html) – Trunk build config
  and the HTML shell that loads the generated WASM bundle.
- [`PLAN.md`](PLAN.md) – the original learning roadmap and TODOs.

## Telegram setup checklist

1. Create a bot with BotFather, then add the `/tarot` command.
2. Host the built app (or tunnel `trunk serve`) behind an HTTPS URL and assign
   it as the bot's WebApp URL.
3. Telegram injects theme colors through `window.Telegram.WebApp`; the app calls
   `expand()` and `ready()` so the webview behaves like a native screen.
4. (Optional) explore [`telegram-webapp-sdk`](https://github.com/RAprogramm/telegram-webapp-sdk)
   for advanced features such as sending results back to the bot.

## Deployment

### Docker

Build and run the Docker image locally (it compiles the WASM bundle with Trunk
and serves the static `dist/` directory from Nginx):

```bash
docker build -t tg-tarot .
docker run -p 8080:8080 tg-tarot
```

### Vercel (static site)

Vercel can deploy the static `dist/` directory directly. The repo ships with
`vercel-build.sh` plus `vercel.json`, so the platform knows how to:

1. Install Rust/Trunk inside the build container.
2. Run `trunk build --release --public-url .`.
3. Skip Vercel’s own `wasm-opt` pass via the `VERCEL_SKIP_WASM_BINARYEN=1`
   environment variable (this avoids the “error parsing wasm” failure).

**Quick deployment**

1. Push this repo to GitHub (or connect via the Vercel CLI).
2. Import the project in Vercel; leave “Framework” as **Other**.
3. Vercel reads `vercel.json`, executes `bash vercel-build.sh`, and serves `dist/`.

**Manual override (if you change settings later)**

- **Build Command:** `bash vercel-build.sh`
- **Output Directory:** `dist`
- **Environment Variable:** `VERCEL_SKIP_WASM_BINARYEN=1`

Telegram Mini Apps expect HTTPS, so a Vercel deployment works out of the box
once the project is connected. After deployment, use your Vercel URL as the
WebApp URL in BotFather.
If you change your mind later and want the Telegram MainButton back, just toggle the visible flag inside src/app.rs:88-96.

## Next steps

- Flesh out the remaining card entries in [`src/deck/cards.rs`](src/deck/cards.rs)
  with intuitive upright/reversed meanings (the major arcana and a few minor
  cards ship as examples).
- Replace the placeholder front-card face with a custom design or animation.
- Persist the last reading locally so returning users can revisit it.
- Add internationalisation if you plan to support multiple languages.

## Further reading

- [Yew documentation and examples](https://github.com/yewstack/yew)
- [telegram-webapp-sdk crate repository](https://github.com/RAprogramm/telegram-webapp-sdk)

Enjoy experimenting – the codebase is intentionally compact so you can focus on
learning Yew and the Telegram WebApp platform at your own pace.
