# Zafkiel

A modern, cross-platform anime watching application built with SvelteKit and Tauri.

## Features

- 🎬 Browse trending and popular anime
- 🔍 Search anime with AniList integration
- 👤 Sync your anime list with AniList account
- ⚡ Smart caching with TanStack Query
- 🎨 Clean, responsive UI with shadcn-svelte
- 🔒 Secure OAuth authentication
- ⚙️ Configurable UI scaling

## Prerequisites

- [Bun](https://bun.sh/) - Fast JavaScript runtime
- [Rust](https://www.rust-lang.org/) - For Tauri backend
- [AniList Account](https://anilist.co/signup) - For authentication

## Quick Start

### 1. Install Dependencies

```bash
bun install
```

### 2. Set Up OAuth (Required)

Create a `.env` file with your AniList OAuth credentials:

```bash
cp .env.example .env
```

Then follow the [OAuth Setup Guide](docs/OAUTH_SETUP.md) to get your credentials.

### 3. Run Development Server

```bash
bun dev
```

The app will open automatically. You'll be prompted to sign in with AniList.

## Documentation

- [OAuth Setup Guide](docs/OAUTH_SETUP.md) - How to set up AniList authentication
- [AniList Integration](docs/ANILIST_INTEGRATION.md) - API integration details
- [Authentication Flow](docs/AUTHENTICATION.md) - OAuth implementation
- [Client Architecture](docs/ANILIST_CLIENT_ARCHITECTURE.md) - Backend architecture

## Tech Stack

- **Frontend**: SvelteKit, TypeScript, TanStack Query, shadcn-svelte
- **Backend**: Tauri, Rust, anilist_moe
- **Styling**: Tailwind CSS, Iconify (Solar icons)
- **Database**: SQLite (via Drizzle ORM)

## Project Structure

```
zafkiel/
├── src/                    # Frontend code
│   ├── lib/
│   │   ├── components/    # UI components
│   │   ├── hooks/         # TanStack Query hooks
│   │   ├── services/      # API clients
│   │   ├── stores/        # Svelte stores
│   │   └── types/         # TypeScript types
│   └── routes/            # SvelteKit pages
├── src-tauri/             # Backend code
│   └── src/
│       ├── anilist.rs     # AniList service
│       ├── auth.rs        # OAuth logic
│       └── config/        # Configuration management
└── docs/                  # Documentation
```

## Building

To create a production build:

```bash
bun run build
```

This will create a distributable Tauri application in `src-tauri/target/release/`.

## Contributing

Contributions are welcome! Please read the [Copilot Instructions](/.github/copilot-instructions.md) for development guidelines.

## License

[Your License Here]
