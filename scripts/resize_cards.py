#!/usr/bin/env python3
"""Resize all card images to consistent dimensions."""

from pathlib import Path
from PIL import Image

TARGET_WIDTH = 600
TARGET_HEIGHT = 1040
ASSETS_DIR = Path(__file__).parent.parent / "assets"

def resize_card(path: Path):
    """Resize a single card image to target dimensions with padding."""
    img = Image.open(path)

    # Skip if already correct size
    if img.size == (TARGET_WIDTH, TARGET_HEIGHT):
        print(f"  {path.name}: already correct size")
        return

    original_size = img.size

    # Calculate scaling to fit within target while maintaining aspect ratio
    ratio = min(TARGET_WIDTH / img.width, TARGET_HEIGHT / img.height)
    new_size = (int(img.width * ratio), int(img.height * ratio))

    # Resize image
    img = img.resize(new_size, Image.Resampling.LANCZOS)

    # Create new image with target size and dark background
    result = Image.new("RGBA", (TARGET_WIDTH, TARGET_HEIGHT), (10, 6, 18, 255))

    # Paste resized image centered
    offset = ((TARGET_WIDTH - new_size[0]) // 2, (TARGET_HEIGHT - new_size[1]) // 2)
    result.paste(img, offset)

    # Save as webp
    result.save(path, "WEBP", quality=90)
    print(f"  {path.name}: {original_size} -> {TARGET_WIDTH}x{TARGET_HEIGHT}")

def main():
    print(f"Resizing cards to {TARGET_WIDTH}x{TARGET_HEIGHT}...")
    print(f"Assets directory: {ASSETS_DIR}")

    cards = list(ASSETS_DIR.glob("*.webp"))
    print(f"Found {len(cards)} cards\n")

    for card_path in sorted(cards):
        resize_card(card_path)

    print(f"\nDone! Resized {len(cards)} cards.")

if __name__ == "__main__":
    main()
