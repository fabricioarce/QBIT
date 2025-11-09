# Commands Reference

A comprehensive guide to all available bot commands for competitive programming and community management.

## 📋 Command Categories

### 🔗 Account Management
Commands for linking and managing your competitive programming profiles.

| Command | Description | Usage | Requirements |
|---------|-------------|-------|--------------|
| `!account <handle>` | Link your Codeforces profile | `!account tourist` | None |
| `!unlink` | Remove account connection | `!unlink` | Linked account |
| `!update` | Refresh profile data | `!update` | Linked account |
| `!profile [user]` | View profile information | `!profile @user` | None |

### 🎯 Problem Solving
Commands for finding, tracking, and verifying problem solutions.

| Command | Description | Usage | Requirements |
|---------|-------------|-------|--------------|
| `!problem [difficulty]` | Get random problem | `!problem 1500` | None |
| `!solved <problem_id>` | Mark problem as solved | `!solved 467B` | Linked account |
| `!hint <problem_id>` | Get problem hint | `!hint 1200A` | Coins |
| `!daily` | Get today's featured problem | `!daily` | None |
| `!unsolved` | List your unsolved attempts | `!unsolved` | Linked account |

### 💰 Economy System
Commands for managing coins, purchases, and virtual economy.

| Command | Description | Usage | Requirements |
|---------|-------------|-------|--------------|
| `!balance` | Check your coin balance | `!balance` | Profile created |
| `!shop` | Browse available items | `!shop` | None |
| `!buy <item>` | Purchase shop item | `!buy hint_pack` | Sufficient coins |
| `!gift @user <amount>` | Send coins to user | `!gift @friend 10` | Sufficient coins |
| `!transactions [limit]` | View transaction history | `!transactions 10` | Profile created |

### 📊 Statistics & Rankings
Commands for viewing progress, statistics, and leaderboards.

| Command | Description | Usage | Requirements |
|---------|-------------|-------|--------------|
| `!stats [user]` | View detailed statistics | `!stats @user` | Linked account |
| `!leaderboard [type]` | Server rankings | `!leaderboard coins` | None |
| `!compare @user` | Compare with another user | `!compare @friend` | Both linked |
| `!progress` | View your improvement over time | `!progress` | Linked account |
| `!achievements` | View unlocked badges | `!achievements` | Profile created |

### 🛡️ Moderation
Commands for server administrators and moderators.

| Command | Description | Usage | Requirements |
|---------|-------------|-------|--------------|
| `!warn @user <reason>` | Issue warning to user | `!warn @user spam` | Moderator |
| `!strikes @user` | View user's warnings | `!strikes @user` | Moderator |
| `!give_coins @user <amount>` | Award coins manually | `!give_coins @user 50` | Admin |
| `!server_stats` | Server-wide statistics | `!server_stats` | Admin |
| `!reset_user @user` | Reset user progress | `!reset_user @user` | Admin |

### 🔧 Utility
General bot commands and utilities.

| Command | Description | Usage | Requirements |
|---------|-------------|-------|--------------|
| `!help [command]` | Show help information | `!help solved` | None |
| `!ping` | Check bot responsiveness | `!ping` | None |
| `!info` | Bot information and stats | `!info` | None |
| `!invite` | Get bot invite link | `!invite` | None |
| `!feedback <message>` | Send feedback to developers | `!feedback Great bot!` | None |

## 🚀 Quick Start Guide

### For New Users
1. **Link Your Account**: `!account your_codeforces_handle`
2. **Check Your Balance**: `!balance`
3. **Get a Problem**: `!problem`
4. **Solve on Codeforces**: Visit the problem link and solve
5. **Verify Solution**: `!solved XXXX`
6. **Check Progress**: `!stats`

### For Server Admins
1. **Check Server Stats**: `!server_stats`
2. **Configure Settings**: Use admin panel or config commands
3. **Moderate Users**: `!warn`, `!strikes` as needed
4. **Reward Activity**: `!give_coins` for special achievements

## 📖 Detailed Documentation

### Individual Command Guides
- **[Account Commands](account.md)** - Profile linking and management
- **[Balance Commands](balance.md)** - Economy system and coin management
- **[Solved Commands](solved.md)** - Problem verification and tracking

### System Overviews
- **[Reward System](../development/rewards.md)** - How coins and XP work
- **[Problem Database](../development/problems.md)** - Problem selection and difficulty
- **[User Progression](../development/progression.md)** - Levels, achievements, and growth

## 🎯 Command Usage Tips

### Maximizing Rewards
- **Daily Consistency** - Solve problems daily for streak bonuses
- **Progressive Difficulty** - Gradually increase problem difficulty
- **Contest Participation** - Extra rewards during live contests
- **Community Engagement** - Help others for social rewards

### Efficient Problem Solving
- **Smart Problem Selection** - Use `!problem <your_rating>` for appropriate difficulty
- **Batch Verification** - Solve multiple problems, then verify with `!solved`
- **Progress Tracking** - Regular `!stats` checks to monitor improvement
- **Goal Setting** - Use `!achievements` to see what you can unlock

## 🔧 Troubleshooting

### Common Issues

| Issue | Possible Cause | Solution |
|-------|----------------|----------|
| Command not responding | Bot offline or no permissions | Check bot status, verify permissions |
| "Account not linked" error | No Codeforces profile connected | Use `!account <handle>` |
| Problem not verifying | Solution not accepted on CF | Check submission verdict on Codeforces |
| Balance not updating | Coin system error | Wait a few minutes, try `!balance` again |
| Wrong statistics | Data sync issue | Use `!update` to refresh profile data |

### Getting Help

1. **Check Documentation** - Use `!help <command>` for specific command info
2. **Verify Setup** - Ensure your account is properly linked
3. **Test Basic Commands** - Try `!ping` and `!info` first
4. **Contact Moderators** - For persistent issues or bugs
5. **Check Status** - Verify Codeforces.com is accessible

## 🎮 Advanced Features

### Power User Commands
- **Custom Filters** - `!problem rating:1500 tag:graphs` for specific problems
- **Batch Operations** - `!solved 467B 1200A 842C` for multiple problems
- **Scheduled Reminders** - Set daily problem notifications
- **Export Data** - Download your progress as CSV/JSON

### Community Features
- **Study Groups** - Commands for group problem-solving sessions
- **Mentorship** - Pair experienced and new users
- **Tournaments** - Server-wide competitive events
- **Team Challenges** - Collaborative problem-solving

## 📊 Command Statistics

### Usage Frequency
- **Most Used**: `!balance`, `!problem`, `!solved`
- **Admin Tools**: `!warn`, `!give_coins`, `!server_stats`
- **Social Features**: `!compare`, `!leaderboard`, `!gift`

### Success Rates
- **Account Linking**: ~95% success rate
- **Problem Verification**: ~85% success rate (dependent on CF API)
- **Coin Transactions**: ~99% success rate

## 🔮 Upcoming Features

### Planned Commands
- [ ] `!contest` - Get information about upcoming contests
- [ ] `!team create <name>` - Create competitive teams
- [ ] `!coach @user` - Request mentorship
- [ ] `!export` - Export personal data
- [ ] `!import` - Import progress from other platforms

### Enhanced Features
- [ ] **Natural Language Processing** - More flexible command parsing
- [ ] **Multi-Platform Support** - AtCoder, LeetCode integration
- [ ] **Advanced Analytics** - Detailed performance insights
- [ ] **Social Features** - Friends, following, and collaboration tools

## 🛠️ For Developers

### Adding New Commands
Commands are organized in the following structure:
- `src/commands/` - Command implementations
- `docs/commands/` - Documentation
- Categories: `codeforces/`, `economy/`, `moderation/`, `utility/`

### Command Template
```rust
// Example command structure
pub async fn command_name(
    ctx: &Context,
    msg: &Message,
    args: Args
) -> CommandResult {
    // Implementation here
}
```

### Testing Commands
```bash
# Run command tests
cargo test commands::

# Test specific category
cargo test commands::economy::balance
```

## 🔒 Permissions & Security

### Permission Levels
- **Everyone**: Basic commands (`!problem`, `!balance`, `!stats`)
- **Verified**: Account-linked commands (`!solved`, `!compare`)
- **Moderator**: Warning and moderation commands
- **Admin**: Server configuration and coin management
- **Owner**: All commands and bot configuration

### Security Features
- **Rate Limiting** - Prevents command spam
- **Input Validation** - Sanitizes all user inputs
- **Permission Checks** - Verifies user permissions before execution
- **Audit Logging** - Logs all administrative actions

## 📞 Support & Resources

### Documentation
- **[Main README](../../README.md)** - Project overview
- **[Development Docs](../development/)** - Technical documentation
- **[Deployment Guides](../deployment/)** - Server setup instructions

### Community
- **Discord Server** - Live support and community
- **GitHub Issues** - Bug reports and feature requests
- **GitHub Discussions** - General questions and ideas

### Contributing
- **[Contributing Guide](../../CONTRIBUTING.md)** - How to contribute
- **[Code of Conduct](../../CODE_OF_CONDUCT.md)** - Community guidelines
- **[Development Setup](../development/setup.md)** - Local development

---

**💡 Pro Tip**: Start with `!help` to see all available commands, then use `!help <command>` to get detailed information about any specific command!

*Last updated: November 2025*