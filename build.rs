use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const LOWERCASE_WORDS: &[&str] = &["of", "the", "and", "in"];

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");

    let metadata = load_metadata()?;

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let dest_path = out_dir.join("generated_cards.rs");
    let mut output = File::create(dest_path)?;

    let cards = discover_cards(&metadata)?;
    write_cards(&mut output, &cards)?;

    Ok(())
}

fn load_metadata() -> Result<HashMap<String, CardMetadata>, Box<dyn Error>> {
    let path = Path::new("cards_data.json");
    if !path.exists() {
        return Ok(HashMap::new());
    }

    println!("cargo:rerun-if-changed=cards_data.json");
    let contents = fs::read_to_string(path)?;
    let entries: Vec<CardMetadata> = serde_json::from_str(&contents)?;

    let mut map = HashMap::new();
    for entry in entries {
        map.insert(entry.slug.clone(), entry);
    }

    Ok(map)
}

fn discover_cards(
    metadata: &HashMap<String, CardMetadata>,
) -> Result<Vec<CardDescriptor>, Box<dyn Error>> {
    let mut cards = Vec::new();

    if let Ok(entries) = fs::read_dir("assets") {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() || !is_webp(&path) {
                continue;
            }

            if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
                println!("cargo:rerun-if-changed=assets/{filename}");
                if let Some((slug, fallback_name)) = parse_filename(filename) {
                    cards.push(build_descriptor(slug, fallback_name, metadata));
                }
            }
        }
    }

    cards.sort_by(|a, b| a.slug.cmp(&b.slug));
    cards.dedup_by(|a, b| a.slug == b.slug);
    Ok(cards)
}

fn build_descriptor(
    slug: String,
    fallback_name: String,
    metadata: &HashMap<String, CardMetadata>,
) -> CardDescriptor {
    if let Some(meta) = metadata.get(&slug) {
        let name = meta.name.clone().unwrap_or_else(|| fallback_name.clone());
        let upright = meta
            .upright
            .clone()
            .unwrap_or_else(|| default_upright(&name));
        let reversed = meta
            .reversed
            .clone()
            .unwrap_or_else(|| default_reversed(&name));
        let keywords = meta.keywords.clone().unwrap_or_default();

        CardDescriptor {
            slug,
            display_name: name,
            upright,
            reversed,
            keywords,
        }
    } else {
        let name = fallback_name;
        CardDescriptor {
            slug,
            display_name: name.clone(),
            upright: default_upright(&name),
            reversed: default_reversed(&name),
            keywords: Vec::new(),
        }
    }
}

fn write_cards(writer: &mut File, cards: &[CardDescriptor]) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "pub const AUTO_CARDS: &[TarotCard] = &[")?;
    for card in cards {
        let keywords_literal = if card.keywords.is_empty() {
            "AUTO_KEYWORDS".to_string()
        } else {
            let list = card
                .keywords
                .iter()
                .map(|kw| format!("{kw:?}"))
                .collect::<Vec<_>>()
                .join(", ");
            format!("&[{list}]")
        };

        writeln!(
            writer,
            "    TarotCard {{ slug: {slug:?}, name: {name:?}, upright: {upright:?}, reversed: {reversed:?}, keywords: {keywords} }},",
            slug = card.slug,
            name = card.display_name,
            upright = card.upright,
            reversed = card.reversed,
            keywords = keywords_literal,
        )?;
    }
    writeln!(writer, "];")?;
    Ok(())
}

fn parse_filename(filename: &str) -> Option<(String, String)> {
    let stem = filename.split('.').next()?.trim();
    if stem.is_empty() {
        return None;
    }

    let display_name = prettify_card_slug(stem).unwrap_or_else(|| fallback_display_name(stem));
    Some((stem.to_string(), display_name))
}

fn prettify_card_slug(slug: &str) -> Option<String> {
    let trimmed = slug
        .trim_start_matches("_images_")
        .trim_start_matches("tarot_");
    let parts: Vec<String> = trimmed
        .split(|c| c == '-' || c == '_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_lowercase())
        .collect();

    if parts.is_empty() {
        return None;
    }

    let mut formatted = Vec::with_capacity(parts.len());
    for (index, word) in parts.into_iter().enumerate() {
        if index > 0 && LOWERCASE_WORDS.contains(&word.as_str()) {
            formatted.push(word);
        } else {
            formatted.push(title_case(&word));
        }
    }

    Some(formatted.join(" "))
}

fn fallback_display_name(slug: &str) -> String {
    let cleaned = slug.replace('-', " ").replace('_', " ");
    title_case(&cleaned)
}

fn title_case(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn is_webp(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("webp")),
        Some(true)
    )
}

fn default_upright(name: &str) -> String {
    format!(
        "{name} invites you to interpret this pull in the context of your question.",
        name = name
    )
}

fn default_reversed(name: &str) -> String {
    format!(
        "{name} reversed highlights the hidden dynamics around your question.",
        name = name
    )
}

struct CardDescriptor {
    slug: String,
    display_name: String,
    upright: String,
    reversed: String,
    keywords: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct CardMetadata {
    slug: String,
    name: Option<String>,
    upright: Option<String>,
    reversed: Option<String>,
    keywords: Option<Vec<String>>,
}
