# tg-tarot-app – Future Work

1. **UI polish** – refine card layout for foldables, add swipe gestures, and animate the flip.
2. **LLM descriptions** – hook `CardGrid` into a backend that fetches GPT-generated prose per card/orientation, with caching.
3. **Persistence** – store the last reading (LocalStorage or Telegram cloud storage) so users can revisit spreads.
4. **Telemetry** – send read-only draw stats to a backend for insight (counts, spreads, session length).
5. **Internationalisation** – add a language switcher and extract strings for translations.
6. **Bot round-trip** – let users send readings back to the bot via `sendData` and surface inline results.
