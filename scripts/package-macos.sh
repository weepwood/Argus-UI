#!/usr/bin/env bash
# Builds a universal macOS .app, then distributes only a drag-to-Applications DMG.
set -euo pipefail

version="${1:?Usage: bash scripts/package-macos.sh <version>}"
if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-.][0-9A-Za-z.-]+)?$ ]]; then
  echo "Invalid package version: $version" >&2
  exit 1
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

cargo build -p argus-ui --release --locked --target aarch64-apple-darwin
cargo build -p argus-ui --release --locked --target x86_64-apple-darwin

app="dist/Argus UI.app"
stage="dist/dmg-root"
dmg="dist/argus-ui-${version}-macos-universal.dmg"
rm -rf "$app" "$stage"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources" "$stage"
lipo -create \
  target/aarch64-apple-darwin/release/argus-ui \
  target/x86_64-apple-darwin/release/argus-ui \
  -output "$app/Contents/MacOS/argus-ui"
chmod +x "$app/Contents/MacOS/argus-ui"
lipo -verify_arch arm64 x86_64 "$app/Contents/MacOS/argus-ui"

cat > "$app/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>Argus UI</string>
  <key>CFBundleDisplayName</key><string>Argus UI</string>
  <key>CFBundleIdentifier</key><string>dev.weepwood.argusui</string>
  <key>CFBundleExecutable</key><string>argus-ui</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$version</string>
  <key>CFBundleVersion</key><string>$version</string>
  <key>LSMinimumSystemVersion</key><string>12.0</string>
  <key>NSHighResolutionCapable</key><true/>
  <key>NSSupportsAutomaticGraphicsSwitching</key><true/>
</dict>
</plist>
EOF
plutil -lint "$app/Contents/Info.plist"
cp LICENSE CHANGELOG.md "$app/Contents/Resources/"

# Ad-hoc signing satisfies arm64's signature requirement; this is not notarization.
codesign --force --deep --sign - "$app"
codesign --verify --deep --strict --verbose=2 "$app"

ditto "$app" "$stage/Argus UI.app"
ln -s /Applications "$stage/Applications"
hdiutil create -volname "Argus UI" -srcfolder "$stage" -ov -format UDZO "$dmg"
hdiutil verify "$dmg"
test -s "$dmg"
rm -rf "$app" "$stage"
echo "Built $dmg (universal arm64 + x86_64)"
