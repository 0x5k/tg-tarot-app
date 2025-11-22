#!/bin/bash
# Resize all card images to consistent dimensions using macOS sips

TARGET_WIDTH=600
TARGET_HEIGHT=1040
ASSETS_DIR="$(dirname "$0")/../assets"

echo "Resizing cards to ${TARGET_WIDTH}x${TARGET_HEIGHT}..."
echo "Assets directory: $ASSETS_DIR"

count=0
for img in "$ASSETS_DIR"/*.webp; do
    if [ -f "$img" ]; then
        name=$(basename "$img")

        # Get current dimensions
        current_height=$(sips -g pixelHeight "$img" 2>/dev/null | awk '/pixelHeight/{print $2}')
        current_width=$(sips -g pixelWidth "$img" 2>/dev/null | awk '/pixelWidth/{print $2}')

        if [ "$current_width" = "$TARGET_WIDTH" ] && [ "$current_height" = "$TARGET_HEIGHT" ]; then
            echo "  $name: already correct size"
        else
            # Resize to fit within target, maintaining aspect ratio
            sips --resampleHeightWidth $TARGET_HEIGHT $TARGET_WIDTH "$img" --out "$img" >/dev/null 2>&1

            # Pad to exact dimensions with dark background
            sips --padToHeightWidth $TARGET_HEIGHT $TARGET_WIDTH --padColor 0A0612 "$img" --out "$img" >/dev/null 2>&1

            echo "  $name: ${current_width}x${current_height} -> ${TARGET_WIDTH}x${TARGET_HEIGHT}"
        fi
        ((count++))
    fi
done

echo ""
echo "Done! Processed $count cards."
