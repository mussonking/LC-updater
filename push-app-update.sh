#!/bin/bash
# ============================================================
# push-app-update.sh
# Deploys LeClasseur-Updater app update artifacts to the server.
#
# Usage:
#   ./push-app-update.sh [version]
#
# If no version is provided, it reads from tauri.conf.json.
#
# This script auto-detects which platform artifacts exist and
# includes all found platforms in the update.json.
#
# Prerequisites:
#   - Build with TAURI_SIGNING_PRIVATE_KEY set, then npm run tauri build
#   - Run this script once per platform, or after building on all platforms
# ============================================================

# === CONFIGURATION ===
BASE_URL="https://dev.leclasseur.ca"
TARGET_DIR="/z/static/app-update"
BUILD_DIR="src-tauri/target/release/bundle"

# 1. Determine version
if [ ! -z "$1" ]; then
    VERSION=$1
    echo "🔧 Manual override: Using version $VERSION"
else
    VERSION=$(grep -o '"version": *"[^"]*"' src-tauri/tauri.conf.json | head -1 | cut -d'"' -f4)
    echo "📦 Using version from tauri.conf.json: $VERSION"
fi

# 2. Ensure target directory exists
mkdir -p "$TARGET_DIR"

# 3. Detect and collect platform artifacts
PLATFORMS_JSON=""

# --- Windows (NSIS or MSI) ---
NSIS_DIR="$BUILD_DIR/nsis"
MSI_DIR="$BUILD_DIR/msi"

if [ -d "$NSIS_DIR" ]; then
    WIN_INSTALLER=$(find "$NSIS_DIR" -name "*.exe" -not -name "*.sig" 2>/dev/null | head -1)
    WIN_SIGNATURE=$(find "$NSIS_DIR" -name "*.exe.sig" 2>/dev/null | head -1)
elif [ -d "$MSI_DIR" ]; then
    WIN_INSTALLER=$(find "$MSI_DIR" -name "*.msi" -not -name "*.sig" 2>/dev/null | head -1)
    WIN_SIGNATURE=$(find "$MSI_DIR" -name "*.msi.sig" 2>/dev/null | head -1)
fi

if [ ! -z "$WIN_INSTALLER" ] && [ ! -z "$WIN_SIGNATURE" ]; then
    WIN_NAME=$(basename "$WIN_INSTALLER")
    WIN_SIG=$(cat "$WIN_SIGNATURE")
    cp "$WIN_INSTALLER" "$TARGET_DIR/$WIN_NAME"
    echo "✅ Windows: $WIN_NAME"
    PLATFORMS_JSON="$PLATFORMS_JSON
        \"windows-x86_64\": {
            \"signature\": \"$WIN_SIG\",
            \"url\": \"${BASE_URL}/static/app-update/$WIN_NAME\"
        }"
fi

# --- macOS (app.tar.gz) ---
MACOS_DIR="$BUILD_DIR/macos"

if [ -d "$MACOS_DIR" ]; then
    MAC_BUNDLE=$(find "$MACOS_DIR" -name "*.app.tar.gz" -not -name "*.sig" 2>/dev/null | head -1)
    MAC_SIGNATURE=$(find "$MACOS_DIR" -name "*.app.tar.gz.sig" 2>/dev/null | head -1)

    if [ ! -z "$MAC_BUNDLE" ] && [ ! -z "$MAC_SIGNATURE" ]; then
        MAC_NAME=$(basename "$MAC_BUNDLE")
        MAC_SIG=$(cat "$MAC_SIGNATURE")
        cp "$MAC_BUNDLE" "$TARGET_DIR/$MAC_NAME"
        echo "✅ macOS: $MAC_NAME"

        # Add comma if windows was already added
        if [ ! -z "$PLATFORMS_JSON" ]; then
            PLATFORMS_JSON="$PLATFORMS_JSON,"
        fi

        # macOS supports both Intel and Apple Silicon via universal binary
        PLATFORMS_JSON="$PLATFORMS_JSON
        \"darwin-x86_64\": {
            \"signature\": \"$MAC_SIG\",
            \"url\": \"${BASE_URL}/static/app-update/$MAC_NAME\"
        },
        \"darwin-aarch64\": {
            \"signature\": \"$MAC_SIG\",
            \"url\": \"${BASE_URL}/static/app-update/$MAC_NAME\"
        }"
    fi
fi

# 4. Check we found at least one platform
if [ -z "$PLATFORMS_JSON" ]; then
    echo "❌ No build artifacts found for any platform."
    echo "   Run 'npm run tauri build' first (with TAURI_SIGNING_PRIVATE_KEY set)."
    exit 1
fi

# Add comma after Windows entry if both platforms exist
# (handled above in the macOS section)

# 5. If an existing update.json exists, merge platforms
EXISTING_JSON="$TARGET_DIR/update.json"
if [ -f "$EXISTING_JSON" ]; then
    EXISTING_VERSION=$(grep -o '"version": *"[^"]*"' "$EXISTING_JSON" | head -1 | cut -d'"' -f4)
    if [ "$EXISTING_VERSION" = "$VERSION" ]; then
        echo "📋 Merging with existing update.json (same version $VERSION)..."
        
        # Extract existing platform entries we DON'T have
        if [ -z "$WIN_INSTALLER" ]; then
            # We don't have Windows artifacts - keep existing Windows entry
            EXISTING_WIN=$(python3 -c "
import json, sys
with open('$EXISTING_JSON') as f:
    d = json.load(f)
p = d.get('platforms', {})
if 'windows-x86_64' in p:
    print(json.dumps({'windows-x86_64': p['windows-x86_64']}))" 2>/dev/null)
            if [ ! -z "$EXISTING_WIN" ] && [ "$EXISTING_WIN" != "None" ]; then
                WIN_SIG=$(echo "$EXISTING_WIN" | python3 -c "import json,sys; print(json.load(sys.stdin)['windows-x86_64']['signature'])")
                WIN_URL=$(echo "$EXISTING_WIN" | python3 -c "import json,sys; print(json.load(sys.stdin)['windows-x86_64']['url'])")
                PLATFORMS_JSON="
        \"windows-x86_64\": {
            \"signature\": \"$WIN_SIG\",
            \"url\": \"$WIN_URL\"
        },$PLATFORMS_JSON"
            fi
        fi

        if [ -z "$MAC_BUNDLE" ]; then
            # We don't have Mac artifacts - keep existing Mac entries
            EXISTING_MAC=$(python3 -c "
import json, sys
with open('$EXISTING_JSON') as f:
    d = json.load(f)
p = d.get('platforms', {})
if 'darwin-x86_64' in p:
    print(json.dumps({'darwin-x86_64': p['darwin-x86_64'], 'darwin-aarch64': p.get('darwin-aarch64', p['darwin-x86_64'])}))" 2>/dev/null)
            if [ ! -z "$EXISTING_MAC" ] && [ "$EXISTING_MAC" != "None" ]; then
                MAC_SIG=$(echo "$EXISTING_MAC" | python3 -c "import json,sys; print(json.load(sys.stdin)['darwin-x86_64']['signature'])")
                MAC_URL=$(echo "$EXISTING_MAC" | python3 -c "import json,sys; print(json.load(sys.stdin)['darwin-x86_64']['url'])")
                if [ ! -z "$PLATFORMS_JSON" ]; then
                    PLATFORMS_JSON="$PLATFORMS_JSON,"
                fi
                PLATFORMS_JSON="$PLATFORMS_JSON
        \"darwin-x86_64\": {
            \"signature\": \"$MAC_SIG\",
            \"url\": \"$MAC_URL\"
        },
        \"darwin-aarch64\": {
            \"signature\": \"$MAC_SIG\",
            \"url\": \"$MAC_URL\"
        }"
            fi
        fi
    fi
fi

# 6. Generate update.json
PUB_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

cat > "$TARGET_DIR/update.json" <<EOF
{
    "version": "$VERSION",
    "notes": "LeClasseur Updater v$VERSION",
    "pub_date": "$PUB_DATE",
    "platforms": {
$PLATFORMS_JSON
    }
}
EOF

echo ""
echo "✅ Generated update.json"
echo "🚀 App update v$VERSION is now live!"
echo "   Endpoint: ${BASE_URL}/static/app-update/update.json"
