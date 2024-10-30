# Tauri + SvelteKit

This template should help get you started developing with Tauri and SvelteKit in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

<div align="center">
  <img src="app-icon.png" alt="NeoHtop Logo" width="120" />
  <h1>NeoHtop</h1>
  <p>A modern, cross-platform system monitor built with Tauri, Rust, and Svelte.</p>
</div>

<div align="center">
  <img src="screenshot.jpg" alt="NeoHtop Screenshot" width="800" />
</div>

## Features

- 🚀 Real-time process monitoring
- 💻 CPU and Memory usage tracking
- 🎨 Beautiful, modern UI with dark/light themes
- 🔍 Process search and filtering
- 📌 Pin important processes
- 🛠 Process management (kill processes)
- 🎯 Sort by any column
- 🔄 Auto-refresh system stats

## Tech Stack

- **Frontend**: SvelteKit, TypeScript
- **Backend**: Rust, Tauri
- **Styling**: CSS Variables for theming
- **Icons**: FontAwesome

## Development

### Prerequisites

- Node.js (v16 or later)
- Rust (latest stable)
- Xcode Command Line Tools (for macOS)

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build

# Build universal macOS binary
./build-universal.sh