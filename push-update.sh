#!/bin/bash

# === CONFIGURATION ===
BASE_URL="https://dev.leclasseur.ca"
TARGET_DIR="/z/static/extension"
VERSION_FILE="$TARGET_DIR/version.json"

# Function to increment version (x.y.z -> x.y.z+1)
increment_version() {
  local v=$1
  if [[ $v =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
    local major=${BASH_REMATCH[1]}
    local minor=${BASH_REMATCH[2]}
    local patch=${BASH_REMATCH[3]}
    echo "$major.$minor.$((patch + 1))"
  else
    echo "1.0.0"
  fi
}

# 1. Determine new version
if [ ! -z "$1" ]; then
    # Manual override if argument provided
    NEW_VERSION=$1
    echo "🔧 Manual override: Using version $NEW_VERSION"
elif [ -f "$VERSION_FILE" ]; then
    # Auto-detect from Z: and increment
    # Extract version using grep/tr (simple JSON parsing without jq)
    CURRENT_VERSION=$(grep -o '"version": *"[^"]*"' "$VERSION_FILE" | cut -d'"' -f4)
    
    if [ -z "$CURRENT_VERSION" ]; then
        NEW_VERSION="1.0.0"
        echo "⚠️  Could not read version from Z:. Defaulting to $NEW_VERSION"
    else
        NEW_VERSION=$(increment_version "$CURRENT_VERSION")
        echo "📈 Found version $CURRENT_VERSION on server. Auto-incrementing to $NEW_VERSION"
    fi
else
    NEW_VERSION="1.0.0"
    echo "✨ No existing version found. Starting at $NEW_VERSION"
fi

echo "🚀 Packaging version $NEW_VERSION..."

# 2. Clean previous builds
rm -f extension.zip version.json

# 3. Zip the extension (excluding dev files)
zip -r extension.zip . -x "*.git*" "node_modules/*" "*.DS_Store" "push-update.sh"

# 4. Generate version.json
cat > version.json <<EOF
{
    "version": "$NEW_VERSION",
    "download_url": "${BASE_URL}/static/extension/extension.zip"
}
EOF

# 5. Deploy to Z: drive
echo "📂 Deploying to $TARGET_DIR..."
cp extension.zip "$TARGET_DIR/extension.zip"
cp version.json "$TARGET_DIR/version.json"

echo "✅ Success! Version $NEW_VERSION is now live."
