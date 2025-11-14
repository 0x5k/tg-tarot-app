# Build Plan

1. **Tooling checklist**
   - Install Rust stable and add the WebAssembly target:
     `rustup target add wasm32-unknown-unknown`.
   - Install [Trunk](https://trunkrs.dev/) for live-reloading builds:
     `cargo install trunk`.
   - Optional: install `wasm-bindgen-cli` if you want to inspect generated
     bindings, but it is not required.

2. **Project tour**
   - `src/app.rs` – root component with the small `Reading` state machine and
     draw flow.
   - `src/deck/` – `mod.rs` exposes the `Deck`, `TarotCard`, and `DrawCount`
     helpers, while `cards.rs` holds the actual data.
   - `src/ui/` – UI building blocks (`DrawControls` and `CardGrid`).
   - `src/telegram.rs` – Telegram WebApp bootstrap + theme extraction.
   - `static/styles.css` – styling and animations copied by Trunk.
   - `assets/` – place your 78 `.webp` images here (slug-based names like
     `ace-of-cups.webp`).

3. **Populate the deck**
   - Expand the `CARDS` array in `src/deck/cards.rs` so all 78 tarot cards have
     upright/reversed meanings and keyword lists.
   - Keep the slugs in sync with the filenames you drop into `assets/`.
   - Feel free to split the data into separate modules if it grows too large.

4. **Run it locally**
   - Copy images into `assets/` (the repo ships with an empty folder so the
     first run works even without assets).
   - Start the dev server with `trunk serve --open`.
   - Update `index.html` or add new styles and Trunk will hot-reload.

5. **Telegram integration**
   - The current bridge calls `expand()` and `ready()` plus applies theme colors.
   - For richer functionality (sending data back to the bot, handling closing
     events, etc.), layer in the official
     [`telegram-webapp-sdk`](https://github.com/RAprogramm/telegram-webapp-sdk).
   - When testing inside Telegram, tunnel your local server (e.g. `ngrok http 8080`)
     and configure the HTTPS URL in BotFather.

6. **Polish ideas**
   - Animate the card fronts or add particle effects when revealing spreads.
   - Let users toggle between upright-only readings and the upright/reversed mix.
   - Persist recent draws in local storage to build a history view.
   - Add localisation support or multiple decks.

7. **Deploy**
   - Produce a production build with `trunk build --release`.
   - Upload the `dist/` directory to a static host (GitHub Pages, Netlify, etc.).
   - Point your bot's WebApp URL to the hosted build and celebrate! 🎉

Use this plan as a learning guide—each step is bite-sized so you can iterate
comfortably as you explore Rust, Yew, and the Telegram platform.
