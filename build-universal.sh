#!/bin/bash

# Build frontend
npm run build

# Build for Intel
echo "Building for x86_64..."
npm run tauri build -- --target x86_64-apple-darwin

# Build for Apple Silicon
echo "Building for aarch64..."
npm run tauri build -- --target aarch64-apple-darwin

# Create universal binary directory
mkdir -p src-tauri/target/universal-apple-darwin/release

# Create universal binary
echo "Creating universal binary..."
lipo "src-tauri/target/x86_64-apple-darwin/release/macos-task-manager" \
     "src-tauri/target/aarch64-apple-darwin/release/macos-task-manager" \
     -create -output "src-tauri/target/universal-apple-darwin/release/macos-task-manager"

# Create universal app bundle
echo "Creating universal app bundle..."
cp -r "src-tauri/target/x86_64-apple-darwin/release/bundle/dmg/NeoHtop.app" \
      "src-tauri/target/universal-apple-darwin/release/"

# Replace the binary in the app bundle
cp "src-tauri/target/universal-apple-darwin/release/macos-task-manager" \
   "src-tauri/target/universal-apple-darwin/release/NeoHtop.app/Contents/MacOS/"

echo "Universal build complete!"
