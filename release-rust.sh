#!/usr/bin/env bash
set -euo pipefail

FILE="Cargo.toml"

read -r -p "Insert the version: " version

if [[ -z "${version// }" ]]; then
  echo "❌ Version cannot be empty."
  exit 1
fi

if [[ ! -f "$FILE" ]]; then
  echo "❌ $FILE not found in the current directory."
  exit 1
fi

tmp="$(mktemp)"
awk -v ver="$version" '
  BEGIN { in_pkg=0; done=0 }
  /^\[package\][[:space:]]*$/ { in_pkg=1; print; next }
  /^\[/ { in_pkg=0; print; next }

  in_pkg && !done && $0 ~ /^[[:space:]]*version[[:space:]]*=/ {
    print "version = \"" ver "\""
    done=1
    next
  }

  { print }
' "$FILE" > "$tmp"
mv "$tmp" "$FILE"

echo "✅ Updated [package] version to \"$version\" in $FILE"

tag="v${version}"

git tag "$tag"
mise changelog
git tag -d "$tag"

echo "🔨 Running cargo build..."
cargo build

git add .
git commit -m "Prepare version to v${version}"
git push

echo "✅ All done with success"