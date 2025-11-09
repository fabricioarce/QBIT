# Balance Command

## Overview

The `!balance` command displays your current **coin balance** on the server. Coins are earned by solving Codeforces problems and can be used for various bot features and future functionalities.

## Syntax

```
!balance
```

## Parameters

This command requires no additional parameters.

## Usage Examples

```bash
# Check your current balance
!balance

# Alternative aliases (if supported)
!coins
!money
```

## How It Works

When you execute this command, the bot:

1. **Database Query** - Searches for your profile on the current server
2. **Balance Retrieval** - Fetches your current coin count
3. **Display Information** - Shows your balance in a user-friendly format

## Bot Responses

### Success Response 💰

```
💰 Coin Balance

👤 User: @your_username
🪙 Coins: 25
🎯 Rank: #42 on server

💡 Solve Codeforces problems with `!solved` to earn more coins!
```

### Error Responses ❌

**Profile Not Found**
```
❌ You don't have a profile on this server yet. Use `!account <handle>` to create one and start earning coins!
```

**Database Error**
```
❌ Unable to access the database. Please try again later.
```

**Server Not Configured**
```
❌ Coin system is not enabled on this server. Contact administrators for setup.
```

## Coin System 🪙

### How to Earn Coins

Currently, you can earn coins through:

| Activity | Reward | Frequency |
|----------|---------|-----------|
| **Solving Problems** | 1-5 coins | Per unique problem |
| **Daily Streaks** | 2 coins | Per day streak |
| **Contest Participation** | 10-50 coins | Per contest |
| **Community Challenges** | Variable | Special events |
| **First Solve** | Bonus +3 coins | First person to solve |

### Earning Rules

- ✅ **Unique Problems Only** - Earn coins once per problem
- ✅ **Verified Solutions** - Must be marked as "Accepted" on Codeforces  
- ✅ **Difficulty Scaling** - Harder problems give more coins
- ✅ **Streak Bonuses** - Consecutive days solving problems
- ❌ **No Double Earning** - Can't earn from the same problem twice

### Coin Values by Difficulty

| Rating Range | Base Coins | Streak Bonus |
|--------------|-----------|--------------|
| **800-1200** | 1 coin | +0.5 |
| **1300-1600** | 2 coins | +1 |
| **1700-2000** | 3 coins | +1.5 |
| **2100-2400** | 4 coins | +2 |
| **2500+** | 5 coins | +2.5 |

### What Can You Do With Coins?

Coins can be used for:

- **🛒 Shop Items** - Purchase roles, badges, and privileges
- **🎨 Profile Customization** - Custom colors, titles, and banners
- **🎯 Problem Hints** - Get hints for difficult problems
- **⚡ Priority Features** - Skip cooldowns, priority support
- **🎉 Special Events** - Entry to exclusive competitions
- **🎁 Gifts** - Send rewards to other community members

## Usage Examples

### New User Flow

```bash
# Step 1: Check balance (no profile yet)
User: !balance
Bot: ❌ You don't have a profile on this server yet. Use `!account <handle>` to create one!

# Step 2: Create profile  
User: !account my_cf_handle
Bot: ✅ Codeforces account linked successfully! [...]

# Step 3: Check initial balance
User: !balance
Bot: 💰 Coin Balance - 👤 User: @user - 🪙 Coins: 0

# Step 4: Solve a problem
User: !solved 1000A
Bot: 🎉 Problem solved and verified! [...] 💰 +1 coin earned!

# Step 5: Check updated balance
User: !balance  
Bot: 💰 Coin Balance - 👤 User: @user - 🪙 Coins: 1
```

### Experienced User

```bash
User: !balance
Bot: 💰 Coin Balance
     👤 User: @experienced_user
     🪙 Coins: 147
     📈 Weekly Earned: 23
     🔥 Current Streak: 12 days
     🎯 Server Rank: #8
```

## Advanced Features

### Balance Statistics

The balance command can show additional information:

- **📊 Weekly Progress** - Coins earned in the past week
- **🔥 Streak Information** - Current problem-solving streak
- **📈 Server Ranking** - Your position on the server leaderboard
- **🎯 Next Milestone** - Coins needed for next achievement

### Balance Tracking

Track your progress with:

- **Daily Earnings** - See how many coins you earned today
- **Growth Trends** - Weekly and monthly earning patterns
- **Achievement Progress** - How close you are to next milestone
- **Spending History** - Recent coin expenditures

## Integration with Other Commands

### Related Commands

| Command | Purpose | Relationship to Balance |
|---------|---------|-------------------------|
| `!account <handle>` | Create profile | Required to have balance |
| `!solved <problem>` | Mark problem solved | Primary way to earn coins |
| `!shop` | View available items | Shows what you can buy |
| `!buy <item>` | Purchase shop items | Spends your coins |
| `!leaderboard coins` | Server coin rankings | Compare balances |
| `!give @user <amount>` | Transfer coins | Share your wealth |

### Command Flow

```
!account → !balance → !solved → !balance → !shop → !buy → !balance
   ↓           ↓         ↓         ↓        ↓       ↓       ↓
 Profile    Check     Earn      Update   Browse  Spend  Verify
 Created    Status    Coins     Balance  Items   Coins  Purchase
```

## Server-Specific Features

### Multi-Server Support

- **🏠 Independent Balances** - Each Discord server has its own coin economy
- **🔄 No Cross-Server Transfer** - Coins cannot be moved between servers
- **⚙️ Server Configuration** - Each server can customize coin rates and rewards

### Server Leaderboards

Your balance contributes to:

- **💰 Coin Leaderboard** - Richest users on the server
- **📈 Weekly Earners** - Most coins earned this week  
- **🔥 Streak Leaders** - Longest problem-solving streaks
- **🎯 Achievement Hunters** - Most achievements unlocked

## Privacy & Security

### Data Protection

- **🔒 Personal Information** - Only you can see your exact balance
- **📊 Anonymous Statistics** - Server stats don't reveal individual balances
- **🛡️ Secure Storage** - All coin data is encrypted and protected
- **🗑️ Data Cleanup** - Inactive accounts are automatically cleaned

### Transparency

- **📝 Transaction Logs** - Every coin transaction is logged
- **🔍 Audit Trail** - Administrators can verify all transactions
- **📊 Public Statistics** - General server statistics are visible
- **⚖️ Fair Play** - Anti-cheat systems prevent exploitation

## Troubleshooting

### Common Issues

**Command doesn't respond**
- Verify bot has permissions in the channel
- Check if bot is online and functional
- Try again after a few minutes

**Shows 0 coins despite solving problems**
- Ensure you used `!solved` to mark problems correctly
- Verify problems were accepted ("OK" verdict on Codeforces)
- Check that you haven't already earned coins for those problems

**Balance seems incorrect**
- Use `!transactions` to view recent coin activity
- Contact moderators if you suspect an error
- Remember that coins are server-specific

### Getting Help

1. **Check Command Usage** - Ensure you're using the correct syntax
2. **Verify Profile Setup** - Make sure your account is properly linked
3. **Review Recent Activity** - Check what problems you've solved recently
4. **Contact Support** - Reach out to server moderators for persistent issues

## Advanced Tips

### Maximizing Earnings

- **🎯 Daily Practice** - Maintain streaks for bonus coins
- **📈 Progressive Difficulty** - Gradually tackle harder problems for better rewards
- **⏰ Timing** - Some servers offer time-based bonuses
- **🤝 Community Events** - Participate in special challenges

### Smart Spending

- **💡 Investment Items** - Buy items that help you earn more coins
- **🎯 Goal Setting** - Save up for meaningful purchases
- **📊 Cost Analysis** - Compare item values before purchasing
- **🎁 Strategic Gifting** - Sometimes giving coins builds valuable relationships

## Future Features

Planned enhancements for the balance system:

- [ ] **Interest System** - Earn passive income on saved coins
- [ ] **Investment Options** - Put coins to work earning more
- [ ] **Lending System** - Loan coins to other users
- [ ] **Seasonal Bonuses** - Special earning periods
- [ ] **Achievement Multipliers** - Unlock permanent earning bonuses

---

**💡 Pro Tip**: Check your balance regularly to track progress and plan your next purchases. Consistent problem-solving is the key to building wealth!

*Last updated: November 2025*