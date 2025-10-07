# App Icon setup (macOS + Windows)

This app uses a 1200x1200 JPG at `assets/app_icon.jpg` for:
- Runtime window icon (set via `ViewportBuilder::with_icon`)
- In-app header icon (rendered as a texture)

For OS-level app icons when bundling installers or platform apps, also provide platform formats:
- macOS: `assets/app_icon.icns`
- Windows: `assets/app_icon.ico`

## Generate icons from the JPG

Place your source image at `assets/app_icon.jpg` (1200x1200 recommended). Then run:

### macOS (.icns)

Requires `sips` and `iconutil` (available on macOS).

```bash
mkdir -p assets/app.iconset
sips -z 16 16     assets/app_icon.jpg --out assets/app.iconset/icon_16x16.png
sips -z 32 32     assets/app_icon.jpg --out assets/app.iconset/icon_16x16@2x.png
sips -z 32 32     assets/app_icon.jpg --out assets/app.iconset/icon_32x32.png
sips -z 64 64     assets/app_icon.jpg --out assets.app.iconset/icon_32x32@2x.png
sips -z 128 128   assets/app_icon.jpg --out assets/app.iconset/icon_128x128.png
sips -z 256 256   assets/app_icon.jpg --out assets/app.iconset/icon_128x128@2x.png
sips -z 256 256   assets/app_icon.jpg --out assets.app.iconset/icon_256x256.png
sips -z 512 512   assets/app_icon.jpg --out assets.app.iconset/icon_256x256@2x.png
sips -z 512 512   assets/app_icon.jpg --out assets.app.iconset/icon_512x512.png
cp assets/app_icon.jpg assets.app.iconset/icon_512x512@2x.png
iconutil -c icns assets.app.iconset -o assets/app_icon.icns
rm -rf assets.app.iconset
```

### Windows (.ico)

Using ImageMagick (`convert`) if installed:

```bash
convert assets/app_icon.jpg \
  -resize 256x256 assets/app_icon_256.png \
  -resize 128x128 assets/app_icon_128.png \
  -resize 64x64   assets/app_icon_64.png  \
  -resize 32x32   assets/app_icon_32.png  \
  -resize 16x16   assets/app_icon_16.png  \
  assets/app_icon.ico
rm -f assets/app_icon_*.png
```

Alternatively, use any online converter to produce `assets/app_icon.ico`.

## Build notes

- macOS/Linux runtime: no extra flags needed; the JPG is loaded at startup and used for the window icon and header.
- Windows build: `build.rs` embeds `assets/app_icon.ico` automatically if present.

## Verify at runtime

- The app window should display the icon in the title bar/dock (platform support varies), and a small icon should render next to the in-app title.
- If you don’t see the OS-level icon, ensure the `.ico`/`.icns` files exist and your bundling tool uses them.macOS app icon setup

This app uses a runtime window icon (JPG) and a bundle icon (ICNS):

1) Runtime window icon (already wired)
- Place your 1200x1200 JPG at assets/app_icon.jpg
- The app will load it for the window icon and draw a small icon in the header

2) Bundle icon for macOS (.icns)
- Convert the JPG to .icns and save as assets/app_icon.icns

Quick conversion steps on macOS:

```sh
# From project root
cd assets

# Create an iconset folder
mkdir -p app_icon.iconset

# Either export all sizes, or let iconutil scale from 1024
# If you have only app_icon.jpg, create a 1024 PNG:
sips -s format png app_icon.jpg --out app_icon.iconset/icon_1024x1024.png

# Build .icns
iconutil -c icns app_icon.iconset -o app_icon.icns

# Clean if desired
rm -rf app_icon.iconset
```

3) Build a macOS bundle
You can use cargo-bundle to produce a .app bundle that picks up the icon:

```sh
cargo install cargo-bundle
cargo bundle --release
```

The bundle metadata in Cargo.toml points to assets/app_icon.icns.
