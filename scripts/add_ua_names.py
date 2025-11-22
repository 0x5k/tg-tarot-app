#!/usr/bin/env python3
"""Add Ukrainian card names to cards_ua.json"""

import json

# Ukrainian translations for card name parts
NUMBERS = {
    "ace": "Туз",
    "two": "Двійка",
    "three": "Трійка",
    "four": "Четвірка",
    "five": "П'ятірка",
    "six": "Шістка",
    "seven": "Сімка",
    "eight": "Вісімка",
    "nine": "Дев'ятка",
    "ten": "Десятка",
    "page": "Паж",
    "knight": "Лицар",
    "queen": "Королева",
    "king": "Король",
}

SUITS = {
    "cups": "Кубків",
    "pentacles": "Пентаклів",
    "swords": "Мечів",
    "wands": "Жезлів",
}

MAJOR_ARCANA = {
    "the-fool": "Блазень",
    "the-magician": "Маг",
    "the-high-priestess": "Верховна Жриця",
    "the-empress": "Імператриця",
    "the-emperor": "Імператор",
    "the-hierophant": "Ієрофант",
    "the-lovers": "Закохані",
    "the-chariot": "Колісниця",
    "strength": "Сила",
    "the-hermit": "Відлюдник",
    "wheel-of-fortune": "Колесо Фортуни",
    "justice": "Справедливість",
    "the-hanged-man": "Повішений",
    "death": "Смерть",
    "temperance": "Поміркованість",
    "the-devil": "Диявол",
    "the-tower": "Вежа",
    "the-star": "Зірка",
    "the-moon": "Місяць",
    "the-sun": "Сонце",
    "judgement": "Суд",
    "the-world": "Світ",
}

def get_ua_name(slug: str) -> str:
    # Check major arcana first
    if slug in MAJOR_ARCANA:
        return MAJOR_ARCANA[slug]

    # Parse minor arcana: "ace-of-cups" -> ["ace", "of", "cups"]
    parts = slug.split("-")
    if len(parts) == 3 and parts[1] == "of":
        number = NUMBERS.get(parts[0], parts[0])
        suit = SUITS.get(parts[2], parts[2])
        return f"{number} {suit}"

    return slug  # fallback

def main():
    with open("translations/cards_ua.json", "r", encoding="utf-8") as f:
        cards = json.load(f)

    for slug in cards:
        cards[slug]["name"] = get_ua_name(slug)

    with open("translations/cards_ua.json", "w", encoding="utf-8") as f:
        json.dump(cards, f, ensure_ascii=False, indent=2)

    print(f"Added Ukrainian names to {len(cards)} cards")

if __name__ == "__main__":
    main()
