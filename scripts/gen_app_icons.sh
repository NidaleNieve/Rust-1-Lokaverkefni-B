#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if [ ! -f assets/app_icon.jpg ]; then
  echo "Missing assets/app_icon.jpg" >&2
  exit 1
fi

echo "Generating macOS .icns..."
mkdir -p assets/app.iconset
sips -z 16 16     assets/app_icon.jpg --out assets/app.iconset/icon_16x16.png
sips -z 32 32     assets/app_icon.jpg --out assets/app.iconset/icon_16x16@2x.png
sips -z 32 32     assets/app_icon.jpg --out assets/app.iconset/icon_32x32.png
sips -z 64 64     assets/app_icon.jpg --out assets/app.iconset/icon_32x32@2x.png
sips -z 128 128   assets/app_icon.jpg --out assets/app.iconset/icon_128x128.png
sips -z 256 256   assets/app_icon.jpg --out assets/app.iconset/icon_128x128@2x.png
sips -z 256 256   assets/app_icon.jpg --out assets/app.iconset/icon_256x256.png
sips -z 512 512   assets/app_icon.jpg --out assets/app.iconset/icon_256x256@2x.png
sips -z 512 512   assets/app_icon.jpg --out assets/app.iconset/icon_512x512.png
cp assets/app_icon.jpg assets/app.iconset/icon_512x512@2x.png
iconutil -c icns assets/app.iconset -o assets/app_icon.icns
rm -rf assets/app.iconset

if command -v convert >/dev/null 2>&1; then
  echo "Generating Windows .ico with ImageMagick..."
  convert assets/app_icon.jpg \
    -resize 256x256 assets/app_icon_256.png \
    -resize 128x128 assets/app_icon_128.png \
    -resize 64x64   assets/app_icon_64.png  \
    -resize 32x32   assets/app_icon_32.png  \
    -resize 16x16   assets/app_icon_16.png  \
    assets/app_icon.ico
  rm -f assets/app_icon_*.png
else
  echo "ImageMagick 'convert' not found; skipping .ico generation."
fi

echo "Done."