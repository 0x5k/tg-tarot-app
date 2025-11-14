use super::TarotCard;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Auto-generated keywords for any cards detected purely from the assets folder.
pub const AUTO_KEYWORDS: &[&str] = &["intuitive", "auto-generated"];

/// Standard tarot deck data. The list includes the 22 Major Arcana plus a
/// sample of the Minor Arcana so newcomers can see how to extend it.
///
/// Feel free to replace the text or add the remaining suits – the build script
/// will automatically pick up any new `.webp` cards as long as the filename
/// matches the slug (e.g. `the-fool.webp`).
#[rustfmt::skip]
const MANUAL_CARDS: &[TarotCard] = &[
    TarotCard { slug: "the-fool", name: "The Fool", upright: "Leap into the new with curiosity and trust the journey.", reversed: "Check your footing before you jump; an impulsive move needs a pause.", keywords: &["beginnings", "wonder", "faith"] },
    TarotCard { slug: "the-magician", name: "The Magician", upright: "Every tool you need is within reach – act with focused intent.", reversed: "Scattered attention or doubt is blurring the spell.", keywords: &["skill", "willpower", "manifestation"] },
    TarotCard { slug: "the-high-priestess", name: "The High Priestess", upright: "Your quiet inner voice already knows the answer.", reversed: "Secrets or second guessing are muffling your intuition.", keywords: &["intuition", "mystery", "stillness"] },
    TarotCard { slug: "the-empress", name: "The Empress", upright: "Nurture ideas with warmth and watch abundance bloom.", reversed: "Creative energy feels blocked – offer yourself gentle care.", keywords: &["creation", "care", "fertility"] },
    TarotCard { slug: "the-emperor", name: "The Emperor", upright: "Lead with calm structure and grounded confidence.", reversed: "Rigidity is choking growth – loosen the rules.", keywords: &["authority", "stability", "boundaries"] },
    TarotCard { slug: "the-hierophant", name: "The Hierophant", upright: "Tradition or mentorship lights the next step on your path.", reversed: "Rewrite the rulebook – dogma is holding you back.", keywords: &["wisdom", "ritual", "learning"] },
    TarotCard { slug: "the-lovers", name: "The Lovers", upright: "Aligned values create magnetic connection.", reversed: "Mixed signals ask for honest conversation.", keywords: &["union", "choice", "harmony"] },
    TarotCard { slug: "the-chariot", name: "The Chariot", upright: "Harness your momentum – drive forward with intention.", reversed: "Split focus causes wheel spin; regain your direction.", keywords: &["determination", "motion", "victory"] },
    TarotCard { slug: "strength", name: "Strength", upright: "Gentle courage tames the wildest storm.", reversed: "Self-doubt is louder than your heart – offer yourself compassion.", keywords: &["courage", "patience", "resilience"] },
    TarotCard { slug: "the-hermit", name: "The Hermit", upright: "Seek solitude to let the inner lantern glow brighter.", reversed: "Isolation has gone too far – open the door a little.", keywords: &["reflection", "guidance", "stillness"] },
    TarotCard { slug: "wheel-of-fortune", name: "Wheel of Fortune", upright: "Life is turning; ride the wave of change with faith.", reversed: "Clinging too tightly slows the wheel – adapt and release.", keywords: &["cycles", "destiny", "timing"] },
    TarotCard { slug: "justice", name: "Justice", upright: "Look at every angle – fairness comes from clarity.", reversed: "Hidden facts or denial keep the scales uneven.", keywords: &["truth", "balance", "accountability"] },
    TarotCard { slug: "the-hanged-man", name: "The Hanged Man", upright: "A fresh perspective appears when you surrender control.", reversed: "Stagnation lingers; take a conscious step forward.", keywords: &["pause", "insight", "release"] },
    TarotCard { slug: "death", name: "Death", upright: "An ending clears space for a profound rebirth.", reversed: "Resistance to change is draining your energy.", keywords: &["transformation", "closure", "renewal"] },
    TarotCard { slug: "temperance", name: "Temperance", upright: "Blend patience with purpose to find your sweet spot.", reversed: "Imbalance shows up as burnout – restore your rhythm.", keywords: &["moderation", "alchemy", "flow"] },
    TarotCard { slug: "the-devil", name: "The Devil", upright: "Name the chain and you can choose to remove it.", reversed: "A release is underway – keep untangling from old habits.", keywords: &["shadow", "attachment", "temptation"] },
    TarotCard { slug: "the-tower", name: "The Tower", upright: "Sudden change shakes loose what was never stable.", reversed: "Avoidance delays the inevitable rebuild – begin now.", keywords: &["upheaval", "awakening", "liberation"] },
    TarotCard { slug: "the-star", name: "The Star", upright: "Hope returns – pour light back into your dreams.", reversed: "Tend the spark; cynicism is dimming your shine.", keywords: &["healing", "optimism", "guidance"] },
    TarotCard { slug: "the-moon", name: "The Moon", upright: "Listen to your dreams; intuition speaks in symbols tonight.", reversed: "Foggy fears fade when you ground in reality.", keywords: &["intuition", "mystery", "emotion"] },
    TarotCard { slug: "the-sun", name: "The Sun", upright: "Joy and clarity radiate – share your light freely.", reversed: "A cloud passes overhead, but the warmth remains.", keywords: &["vitality", "success", "confidence"] },
    TarotCard { slug: "judgement", name: "Judgement", upright: "Answer the call – your next chapter is ready.", reversed: "Self-criticism is muting the trumpet. Forgive and rise.", keywords: &["awakening", "purpose", "evaluation"] },
    TarotCard { slug: "the-world", name: "The World", upright: "Celebrate completion – a cycle is gracefully closing.", reversed: "Tie up loose threads before you move on.", keywords: &["wholeness", "achievement", "integration"] },
    // Minor Arcana sampler -------------------------------------------------
    TarotCard { slug: "ace-of-cups", name: "Ace of Cups", upright: "Love and inspiration overflow – receive the blessing.", reversed: "Check in with your heart; it needs a refill.", keywords: &["emotion", "intuition", "connection"] },
    TarotCard { slug: "two-of-cups", name: "Two of Cups", upright: "Mutual respect creates a beautiful exchange.", reversed: "Realign expectations; a bond needs honest tending.", keywords: &["partnership", "trust", "balance"] },
    TarotCard { slug: "ace-of-wands", name: "Ace of Wands", upright: "Ignite the idea – action turns spark into flame.", reversed: "Restlessness scatters the fire; choose one direction.", keywords: &["passion", "drive", "creation"] },
    TarotCard { slug: "ace-of-swords", name: "Ace of Swords", upright: "Truth slices through confusion – speak with clarity.", reversed: "Doubt fogs the insight; ground your thoughts.", keywords: &["clarity", "logic", "communication"] },
    TarotCard { slug: "ace-of-pentacles", name: "Ace of Pentacles", upright: "A practical opportunity is ready to plant.", reversed: "Tidy the foundation before investing more energy.", keywords: &["stability", "resources", "new beginning"] },
];

include!(concat!(env!("OUT_DIR"), "/generated_cards.rs"));

pub static CARDS: Lazy<&'static [TarotCard]> = Lazy::new(|| {
    let mut by_slug: HashMap<&'static str, TarotCard> = HashMap::new();

    for card in MANUAL_CARDS {
        by_slug.insert(card.slug, *card);
    }

    for card in AUTO_CARDS {
        by_slug.insert(card.slug, *card);
    }

    let mut merged: Vec<TarotCard> = by_slug.values().copied().collect();
    merged.sort_by(|a, b| a.name.cmp(b.name));
    Box::leak(merged.into_boxed_slice())
});
