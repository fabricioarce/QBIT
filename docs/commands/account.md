# Account Command

## Overview

The `!account` command links your **Codeforces** profile with your Discord account on the server. Once linked, the bot can track your progress, statistics, and provide personalized experiences based on your Codeforces performance.

## Syntax

```
!account <codeforces_handle>
```

## Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `codeforces_handle` | String | ✅ | Your Codeforces username/handle |

## Usage Examples

```bash
# Link account with Codeforces handle
!account tourist
!account Errichto
!account your_cf_username

# Alternative syntax (if supported)
!link tourist
```

## How It Works

When you execute this command, the bot performs the following steps:

1. **Input Validation** - Verifies that a Codeforces handle was provided
2. **API Query** - Connects to the official Codeforces API to fetch your profile
3. **User Verification** - Confirms the user exists on Codeforces
4. **Data Storage** - Saves your profile information to the database
5. **Confirmation** - Displays a success message with your current stats

## Stored Information

The bot saves the following data from your Codeforces profile:

| Field | Description | Example |
|-------|-------------|---------|
| **Handle** | Your Codeforces username | `tourist` |
| **Current Rating** | Your current contest rating | `3500` |
| **Rank** | Your current title/rank | `legendary grandmaster` |
| **Max Rating** | Highest rating ever achieved | `3679` |
| **Last Update** | When profile was last synced | `2025-11-09 14:30:00` |

## Bot Responses

### Success Response ✅

```
✅ Codeforces account linked successfully!

👤 Handle: tourist
🏆 Current Rating: 3500
🎯 Rank: legendary grandmaster
📊 Max Rating: 3679
🕒 Last Updated: just now
```

### Error Responses ❌

**Missing Handle**
```
❌ Please provide a Codeforces handle. Usage: `!account <handle>`
```

**User Not Found**
```
❌ User 'invalid_handle' not found on Codeforces. Please check your handle.
```

**API Error**
```
❌ Unable to connect to Codeforces API. Please try again later.
```

**Database Error**
```
❌ Failed to save account information. Please try again later.
```

**Private Profile**
```
❌ Cannot access profile data. Ensure your Codeforces profile is public.
```

## Important Notes

### ⚠️ Account Updates
- If you already have a linked account, running the command again will **update** your information
- Your stats are refreshed with the latest data from Codeforces

### 🔄 Data Synchronization
- Information is fetched in **real-time** from Codeforces
- The bot may cache data for performance reasons
- Manual updates can be triggered by re-running the command

### 📊 Privacy & Data
- Only **public information** from your Codeforces profile is stored
- No private or sensitive data is accessed or stored
- You can unlink your account at any time (see `!unlink` command)

### 🌐 API Dependency
- Command availability depends on Codeforces API status
- During Codeforces maintenance, the command may be temporarily unavailable

## Unlocked Features

Once your account is linked, you gain access to:

### Personalized Commands
- `!stats` - View your detailed statistics
- `!progress` - Track your problem-solving progress
- `!problems <difficulty>` - Get problems suited to your level
- `!compare @user` - Compare stats with other users

### Automatic Features
- **Smart Problem Recommendations** - Based on your skill level
- **Progress Tracking** - Automatic updates when you solve problems
- **Leaderboard Participation** - Appear in server rankings
- **Achievement Badges** - Unlock achievements based on your performance

### Enhanced Experience
- **Difficulty-Based Rewards** - Earn more points for harder problems
- **Rank-Specific Roles** - Automatic role assignment based on your Codeforces rank
- **Personalized Daily Problems** - Problems tailored to your skill level

## Related Commands

| Command | Description | Requires Linked Account |
|---------|-------------|------------------------|
| `!stats [user]` | View detailed statistics | ✅ |
| `!unlink` | Remove account connection | ✅ |
| `!update` | Refresh profile data | ✅ |
| `!problems <difficulty>` | Get problems for your level | ❌ (enhanced with link) |
| `!ranking` | Server leaderboard | ❌ (requires link to appear) |

## Troubleshooting

### Common Issues

**Handle is valid but command fails**
- Ensure your Codeforces profile is set to public
- Try again after a few minutes (API rate limiting)
- Check if Codeforces is experiencing downtime

**Bot shows wrong information**
- Information is cached for performance
- Use `!update` command to force refresh
- Contact administrators if data remains incorrect

**Account linking keeps failing**
- Verify you're using the exact Codeforces handle (case-sensitive)
- Check your internet connection
- Report persistent issues to server moderators

### Getting Help

1. **Verify Handle** - Double-check your Codeforces username spelling
2. **Check Profile Privacy** - Ensure your Codeforces profile is public
3. **API Status** - Visit [Codeforces](https://codeforces.com) to check if the site is accessible
4. **Server Support** - Contact server administrators for persistent issues

## Security & Privacy

- **No Password Required** - Only public profile data is accessed
- **Secure Storage** - All data is encrypted and securely stored
- **Data Control** - You can unlink your account at any time
- **Transparency** - Only stores data visible on your public Codeforces profile

---

**💡 Pro Tip**: Link your account early to get the most out of the bot's competitive programming features and track your progress over time!

*Last updated: November 2025*