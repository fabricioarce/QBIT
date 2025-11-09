# Contributing to Bot Olim P-Code

Thank you for your interest in contributing to Bot Olim P-Code! This document provides guidelines and information for contributors to help make the process smooth and effective.

## 🎯 Project Overview

Bot Olim P-Code is a Discord bot designed for competitive programming communities, featuring:
- **Codeforces Integration** - Profile linking, problem tracking, and statistics
- **Gamification System** - XP, levels, achievements, and progression tracking
- **Virtual Economy** - Coins, shop system, and rewards
- **Community Features** - Leaderboards, daily problems, and social interaction
- **Moderation Tools** - Automated and manual moderation capabilities

## 🤝 How to Contribute

There are many ways to contribute to this project:

### 🐛 Bug Reports
- Report bugs through [GitHub Issues](https://github.com/your-username/bot-olim-p-code/issues)
- Use the bug report template
- Include steps to reproduce, expected vs actual behavior
- Provide relevant logs and system information

### 💡 Feature Requests
- Suggest new features through [GitHub Issues](https://github.com/your-username/bot-olim-p-code/issues)
- Use the feature request template
- Explain the use case and benefits
- Consider implementation complexity and maintenance

### 📝 Documentation
- Improve existing documentation
- Add examples and tutorials
- Translate documentation
- Fix typos and clarify confusing sections

### 💻 Code Contributions
- Fix bugs and implement features
- Improve performance and efficiency
- Add tests and improve test coverage
- Refactor and clean up code

### 🎨 Design & UX
- Improve Discord embed designs
- Suggest better user interaction flows
- Design bot avatars, logos, or promotional materials

## 🚀 Getting Started

### Prerequisites

Before you begin, ensure you have:
- [Rust](https://rustup.rs/) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/download/) (version 12+)
- [Git](https://git-scm.com/)
- A [Discord Bot Token](https://discord.com/developers/applications)
- Basic knowledge of Rust and Discord bots

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/your-username/bot-olim-p-code.git
   cd bot-olim-p-code
   ```

2. **Install Dependencies**
   ```bash
   # Install SQLx CLI for database management
   cargo install sqlx-cli --no-default-features --features postgres
   
   # Install development tools
   cargo install cargo-watch
   ```

3. **Environment Setup**
   ```bash
   # Copy environment template
   cp .env.example .env
   
   # Edit .env with your configuration
   nano .env
   ```

   Required environment variables:
   ```env
   DISCORD_TOKEN=your_discord_bot_token
   DATABASE_URL=postgresql://username:password@localhost:5432/bot_database
   RUST_LOG=debug
   ```

4. **Database Setup**
   ```bash
   # Create database
   sqlx database create
   
   # Run migrations
   sqlx migrate run
   ```

5. **Build and Test**
   ```bash
   # Build the project
   cargo build
   
   # Run tests
   cargo test
   
   # Run the bot
   cargo run
   ```

6. **Development Workflow**
   ```bash
   # Use cargo-watch for automatic rebuilds during development
   cargo watch -x run
   ```

### Project Structure

Familiarize yourself with the project structure:
- **`src/commands/`** - Discord command implementations
- **`src/services/`** - Business logic and external API integrations
- **`src/database/`** - Database models and queries
- **`src/bot/`** - Discord bot core functionality
- **`docs/`** - Project documentation
- **`tests/`** - Test suites

For detailed information, see [Project Structure Guide](docs/development/function_files.md).

## 📋 Contribution Guidelines

### Code Standards

#### Rust Code Style
- **Formatting**: Use `cargo fmt` to format all code
- **Linting**: Address all `cargo clippy` warnings
- **Naming**: Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- **Documentation**: Document all public APIs with rustdoc comments
- **Error Handling**: Use proper error handling with `Result` types

#### Code Quality
```bash
# Before submitting, ensure code passes all checks:
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

#### Example Code Style
```rust
/// Calculate coin rewards for solving a problem.
/// 
/// # Arguments
/// * `problem_rating` - The difficulty rating of the problem
/// * `user_streak` - Current solving streak of the user
/// 
/// # Returns
/// The number of coins to award
pub fn calculate_problem_reward(problem_rating: Option<u32>, user_streak: u32) -> u32 {
    let base_reward = match problem_rating {
        Some(rating) if rating < 1200 => 1,
        Some(rating) if rating < 1600 => 2,
        Some(rating) if rating < 2000 => 3,
        Some(_) => 4,
        None => 1,
    };
    
    let streak_bonus = (user_streak / 7).min(5);
    base_reward + streak_bonus
}
```

### Database Changes

#### Migrations
- **Never modify existing migrations** - always create new ones
- **Test migrations** thoroughly, including rollbacks
- **Document changes** in migration comments

```sql
-- Migration: 006_add_achievements.sql
-- Description: Add achievements table for user badge system

CREATE TABLE achievements (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL REFERENCES guilds(guild_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL,
    achievement_type TEXT NOT NULL,
    achieved_at TIMESTAMP DEFAULT NOW(),
    
    UNIQUE(guild_id, user_id, achievement_type)
);

CREATE INDEX idx_achievements_user ON achievements(guild_id, user_id);
```

#### Query Guidelines
- **Use prepared statements** with SQLx for type safety
- **Optimize for performance** - add appropriate indexes
- **Handle errors gracefully** - don't panic on DB errors
- **Use transactions** for multi-step operations

### Testing Requirements

#### Test Coverage
- **Unit tests** for all business logic functions
- **Integration tests** for command flows
- **Database tests** for query functions
- **Mock external APIs** in tests

#### Test Organization
```
tests/
├── unit/
│   ├── commands/
│   ├── services/
│   └── database/
└── integration/
    ├── command_flows/
    └── api_integration/
```

#### Example Test
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_problem_reward() {
        assert_eq!(calculate_problem_reward(Some(800), 0), 1);
        assert_eq!(calculate_problem_reward(Some(1500), 7), 3);
        assert_eq!(calculate_problem_reward(None, 14), 3);
    }
    
    #[tokio::test]
    async fn test_mark_problem_solved() {
        let pool = setup_test_db().await;
        let result = mark_problem_solved(&pool, 123, 456, "1000A").await;
        assert!(result.is_ok());
    }
}
```

### Documentation

#### Code Documentation
- **Public APIs** must have rustdoc comments
- **Complex logic** should have inline comments
- **Examples** should be included for non-trivial functions

#### User Documentation
- **Command docs** in `docs/commands/`
- **Feature guides** for new functionality
- **Update README** for major changes
- **Include examples** in documentation

### Commit Messages

Use conventional commit format:
```
type(scope): brief description

Detailed explanation if needed

Fixes #issue_number
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

**Examples**:
```
feat(economy): add coin transfer between users

Allow users to send coins to other server members using the !gift command.
Includes transaction logging and balance validation.

Fixes #42
```

```
fix(codeforces): handle API rate limiting properly

Add exponential backoff retry logic for Codeforces API requests
to prevent failures during high traffic periods.

Fixes #89
```

## 🔄 Pull Request Process

### Before Submitting
1. **Create an issue** for significant changes
2. **Fork the repository** and create a feature branch
3. **Write tests** for your changes
4. **Update documentation** as needed
5. **Run all checks**: `cargo fmt`, `cargo clippy`, `cargo test`

### PR Guidelines
1. **Use descriptive title** following conventional commits
2. **Fill out PR template** completely
3. **Link related issues** using keywords (Fixes #123)
4. **Add screenshots** for UI changes
5. **Request appropriate reviewers**

### PR Template Checklist
- [ ] Tests added/updated and passing
- [ ] Documentation updated
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] Commits follow conventional format
- [ ] Breaking changes documented

### Review Process
1. **Automated checks** must pass
2. **Code review** by maintainer(s)
3. **Testing** on development environment
4. **Approval** required before merge
5. **Squash and merge** preferred for feature branches

## 🐛 Issue Reporting

### Bug Reports
When reporting bugs, include:
- **Clear title** describing the issue
- **Steps to reproduce** the problem
- **Expected behavior** vs actual behavior
- **Environment details** (OS, Rust version, etc.)
- **Relevant logs** and error messages
- **Screenshots** if applicable

### Feature Requests
For feature requests, provide:
- **Clear description** of the proposed feature
- **Use case** and benefits
- **Potential implementation** approach
- **Alternatives considered**
- **Additional context** or examples

## 🏷️ Issue Labels

We use labels to organize and prioritize issues:

### Type Labels
- `bug` - Something isn't working correctly
- `feature` - New functionality request
- `enhancement` - Improvement to existing feature
- `documentation` - Documentation related
- `question` - General questions or discussions

### Priority Labels
- `critical` - Critical bugs or security issues
- `high` - High priority items
- `medium` - Medium priority items
- `low` - Low priority items

### Status Labels
- `help wanted` - Community contributions welcome
- `good first issue` - Suitable for new contributors
- `in progress` - Currently being worked on
- `blocked` - Waiting on dependencies

### Component Labels
- `commands` - Discord command related
- `database` - Database related
- `codeforces` - Codeforces integration
- `economy` - Virtual economy system
- `moderation` - Moderation features

## 👥 Community Guidelines

### Code of Conduct
This project adheres to a code of conduct adapted from the [Contributor Covenant](https://www.contributor-covenant.org/). By participating, you agree to:

- **Be respectful** and inclusive to all contributors
- **Use welcoming language** and be considerate
- **Accept constructive criticism** gracefully
- **Focus on what's best** for the community
- **Show empathy** towards other community members

### Communication
- **Discord Server**: [Join our development Discord](link-to-server)
- **GitHub Discussions**: For design discussions and questions
- **Issues**: For bug reports and feature requests
- **Email**: maintainer@example.com for sensitive matters

### Getting Help
- **New contributors**: Look for `good first issue` labels
- **Documentation**: Check existing docs first
- **Ask questions**: Use GitHub Discussions or Discord
- **Mentoring**: Experienced contributors can request mentorship

## 🎉 Recognition

### Contributors
All contributors are recognized in:
- **Contributors section** of README.md
- **Release notes** for significant contributions
- **Special thanks** in project announcements

### Maintainers
Current maintainers:
- **@username1** - Project lead, architecture
- **@username2** - Codeforces integration, testing
- **@username3** - Documentation, community management

## 📚 Additional Resources

### Learning Resources
- **[The Rust Book](https://doc.rust-lang.org/book/)** - Learn Rust fundamentals
- **[Serenity Documentation](https://docs.rs/serenity/)** - Discord bot framework
- **[SQLx Documentation](https://docs.rs/sqlx/)** - Database toolkit
- **[Tokio Tutorial](https://tokio.rs/tokio/tutorial)** - Async programming

### Tools
- **[Rust Analyzer](https://rust-analyzer.github.io/)** - IDE support
- **[Clippy](https://doc.rust-lang.org/clippy/)** - Linting tool
- **[Rustfmt](https://github.com/rust-lang/rustfmt)** - Code formatting
- **[Cargo Watch](https://github.com/watchexec/cargo-watch)** - Auto-rebuild

### External APIs
- **[Codeforces API](https://codeforces.com/apiHelp)** - Competitive programming platform
- **[Discord API](https://discord.com/developers/docs/intro)** - Discord integration
- **[PostgreSQL Docs](https://www.postgresql.org/docs/)** - Database documentation

## ❓ FAQ

**Q: I'm new to Rust. Can I still contribute?**
A: Absolutely! Look for issues labeled `good first issue` and don't hesitate to ask questions.

**Q: How do I test my changes with Discord?**
A: Create a test Discord server and invite your bot there. Use the development environment for testing.

**Q: Can I work on multiple issues simultaneously?**
A: It's better to focus on one issue at a time to avoid conflicts and ensure quality.

**Q: How long does the review process take?**
A: Usually 2-7 days, depending on the complexity of changes and maintainer availability.

**Q: What if my PR is rejected?**
A: Don't worry! We'll provide feedback. Address the concerns and resubmit or discuss alternatives.

## 📞 Contact

- **Project Issues**: [GitHub Issues](https://github.com/your-username/bot-olim-p-code/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/bot-olim-p-code/discussions)
- **Discord**: [Development Server](link-to-discord)
- **Email**: maintainer@example.com

---

Thank you for contributing to Bot Olim P-Code! Your efforts help make competitive programming more accessible and enjoyable for Discord communities worldwide. 🚀

*Last updated: November 2025*