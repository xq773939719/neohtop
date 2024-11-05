#!/bin/bash
# source .env for notarization credentials
echo "Sourcing .env..."
if [ -f .env ]; then
    source .env
    # Verify that required environment variables are set
    if [ -z "$APPLE_ID" ] || [ -z "$APPLE_PASSWORD" ] || [ -z "$APPLE_TEAM_ID" ]; then
        echo "Error: Required environment variables are not set"
        echo "Please ensure your .env file contains:"
        echo "APPLE_ID=your.email@example.com"
        echo "APPLE_PASSWORD=your-app-specific-password"
        echo "APPLE_TEAM_ID=your-team-id"
        exit 1
    fi
    # Export the variables so they are available to child processes
    export APPLE_ID
    export APPLE_PASSWORD
    export APPLE_TEAM_ID
else
    echo "Error: .env file not found"
    echo "Please create a .env file with the following variables:"
    echo "APPLE_ID=your.email@example.com"
    echo "APPLE_PASSWORD=your-app-specific-password"
    echo "APPLE_TEAM_ID=your-team-id"
    exit 1
fi

# Build frontend
npm run build

# Build for Intel
echo "Building for x86_64..."
npm run tauri build -- --target x86_64-apple-darwin --bundles dmg

# Build for Apple Silicon
echo "Building for aarch64..."
npm run tauri build -- --target aarch64-apple-darwin --bundles dmg

