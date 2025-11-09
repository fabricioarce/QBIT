# Bot Olim P-Code

A Discord bot built with Rust for competitive programming communities, featuring Codeforces integration, gamification, and community engagement tools.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=for-the-badge&logo=discord&logoColor=white)](https://discord.com/)
[![PostgreSQL](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)](https://www.postgresql.org/)

## ✨ Features

### 🎯 Competitive Programming
- **Codeforces Integration**: Sync user profiles and track problem-solving progress
- **Daily Problems**: Automated daily problem distribution to encourage consistent practice
- **Problem Recommendations**: Smart problem suggestions based on user skill level
- **Progress Tracking**: Comprehensive statistics and performance analytics

### 🎮 Gamification System
- **Experience Points (XP)**: Users earn XP for server participation and problem solving
- **Level System**: Progressive leveling based on accumulated XP
- **Achievement Badges**: Unlockable achievements for various milestones
- **Streak Tracking**: Daily problem-solving streak counter

### 💰 Virtual Economy
- **Virtual Currency**: Earn and spend points through various activities
- **Shop System**: Purchase roles, privileges, and special items
- **Transaction History**: Complete audit trail of all economic activities
- **Reward System**: Dynamic rewards based on difficulty and performance

### 🛡️ Moderation Tools
- **Warning System**: Progressive discipline with automated actions
- **Activity Logging**: Comprehensive logging of all moderation actions
- **Auto-moderation**: Spam detection and automated responses
- **Role Management**: Automatic role assignment based on activity and level

### 📊 Analytics & Rankings
- **Leaderboards**: Server-wide and time-based ranking systems
- **User Statistics**: Detailed individual and comparative statistics
- **Progress Visualization**: Rich embeds showing user growth and achievements
- **Community Insights**: Server-wide analytics and trends

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/download/) (version 12+)
- [Discord Bot Token](https://discord.com/developers/applications)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-username/bot-olim-p-code.git
   cd bot-olim-p-code
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Configure your environment**
   ```env
   DISCORD_TOKEN=your_discord_bot_token_here
   DATABASE_URL=postgresql://username:password@localhost:5432/bot_database
   RUST_LOG=info
   ```

4. **Set up the database**
   ```bash
   # Install SQLx CLI if you haven't already
   cargo install sqlx-cli --no-default-features --features postgres

   # Create database and run migrations
   sqlx database create
   sqlx migrate run
   ```

5. **Build and run**
   ```bash
   cargo run --release
   ```

## 🐳 Docker Deployment

### Local Development
```bash
docker-compose up --build
```

### Production Deployment
See our comprehensive deployment guides:
- [Docker Deployment Guide](docs/deployment/DOCKER-README.md)
- [Coolify Quick Start](docs/deployment/COOLIFY-QUICKSTART.md)
- [Deployment Checklist](docs/deployment/DEPLOYMENT-CHECKLIST.md)

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `DISCORD_TOKEN` | Discord bot token | ✅ | - |
| `DATABASE_URL` | PostgreSQL connection string | ✅ | - |
| `RUST_LOG` | Log level (trace, debug, info, warn, error) | ❌ | `info` |
| `RUST_BACKTRACE` | Enable backtraces (0, 1, full) | ❌ | `0` |

### Database Schema

The bot uses PostgreSQL with the following main tables:
- `guilds` - Server configuration and settings
- `user_data` - Per-server user information (XP, level, coins)
- `economy_transactions` - Transaction history
- `moderation_logs` - Moderation action logs
- `auto_roles` - Automatic role assignment rules
- `codeforces_profiles` - Linked Codeforces accounts

## 🎮 Usage

### Basic Commands

| Command | Description | Example |
|---------|-------------|---------|
| `!help` | Show available commands | `!help` |
| `!ping` | Check bot responsiveness | `!ping` |
| `!link <handle>` | Link Codeforces profile | `!link tourist` |
| `!stats [user]` | View user statistics | `!stats @username` |
| `!problem [difficulty]` | Get random problem | `!problem 1500` |
| `!solved <problem_id>` | Mark problem as solved | `!solved 1234A` |
| `!ranking [period]` | View leaderboard | `!ranking weekly` |
| `!balance` | Check your points | `!balance` |
| `!shop` | View available items | `!shop` |

### Admin Commands

| Command | Description | Permission |
|---------|-------------|------------|
| `!give_points <user> <amount>` | Award points to user | Admin |
| `!warn <user> <reason>` | Issue warning | Moderator |
| `!server_stats` | Server statistics | Admin |

## 🏗️ Architecture

The bot follows a modular architecture:

```
src/
├── main.rs              # Application entry point
├── bot/                 # Bot configuration and event handling
├── commands/            # Command implementations
│   ├── codeforces/      # Codeforces-related commands
│   ├── economy/         # Economy system commands
│   ├── moderation/      # Moderation commands
│   └── utility/         # General utility commands
├── services/            # External service integrations
├── database/            # Database connection and queries
├── tasks/               # Scheduled tasks and background jobs
└── utils/               # Shared utilities and helpers
```

For detailed architecture documentation, see [ARCHITECTURE.md](docs/development/ARCHITECTURE.md).

## 📚 Documentation

- **[Commands Reference](docs/commands/)** - Detailed command documentation
- **[Development Guide](docs/development/)** - Architecture and development setup
- **[Deployment Guides](docs/deployment/)** - Production deployment instructions
- **[Database Schema](docs/development/Database_idea.md)** - Database design and structure

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run tests: `cargo test`
5. Run clippy: `cargo clippy`
6. Format code: `cargo fmt`
7. Commit changes: `git commit -m 'Add amazing feature'`
8. Push to branch: `git push origin feature/amazing-feature`
9. Open a Pull Request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Ensure all clippy warnings are addressed (`cargo clippy`)
- Write descriptive commit messages
- Add documentation for public APIs
- Include tests for new functionality

## 🐛 Troubleshooting

### Common Issues

**Bot doesn't respond to commands**
- Verify the bot has proper permissions in your Discord server
- Check that the bot is online and connected
- Ensure the command prefix is correct

**Database connection errors**
- Verify PostgreSQL is running and accessible
- Check DATABASE_URL format and credentials
- Ensure the database exists and migrations have run

**Build failures**
- Update Rust to the latest stable version: `rustup update`
- Clean and rebuild: `cargo clean && cargo build`
- Check for missing system dependencies

For more detailed troubleshooting, see our [deployment documentation](docs/deployment/).

## 📈 Roadmap

- [ ] **v1.0** - Core features (MVP)
  - [x] Basic Discord bot functionality
  - [x] Codeforces integration
  - [x] User tracking and statistics
  - [ ] Daily problem distribution
  - [ ] Basic gamification

- [ ] **v1.1** - Enhanced Features
  - [ ] Advanced shop system
  - [ ] Achievement badges
  - [ ] Contest integration
  - [ ] Web dashboard

- [ ] **v2.0** - Community Features
  - [ ] Team competitions
  - [ ] Mentorship system
  - [ ] Advanced analytics
  - [ ] Multi-platform support (AtCoder, LeetCode)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Serenity](https://github.com/serenity-rs/serenity) - Discord API library for Rust
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit for Rust
- [Codeforces API](https://codeforces.com/apiHelp) - Competitive programming platform API
- [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust

## 📞 Support

- **Documentation**: Check our [docs](docs/) directory
- **Issues**: [GitHub Issues](https://github.com/your-username/bot-olim-p-code/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/bot-olim-p-code/discussions)

---

<div align="center">

**[Documentation](docs/) • [Contributing](CONTRIBUTING.md) • [Changelog](CHANGELOG.md)**

Made with ❤️ by the competitive programming community

</div>