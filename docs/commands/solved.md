# Solved Command

## Overview

The `!solved` command allows you to verify that you have successfully solved a specific **Codeforces** problem and marks it as completed in your server profile. The bot automatically queries your Codeforces submissions to confirm that the problem was solved correctly.

## Syntax

```
!solved <contest_id><problem_index>
```

## Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `contest_id` | Integer | ✅ | Numeric ID of the Codeforces contest |
| `problem_index` | String | ✅ | Letter or index of the problem (A, B, C, D, etc.) |

**Format Note**: Parameters must be written together without spaces (e.g., `467B`, `1200A`, `842D`)

## Usage Examples

```bash
# Basic problem verification
!solved 467B        # Contest 467, Problem B
!solved 1200A       # Contest 1200, Problem A
!solved 842D2       # Contest 842, Problem D2
!solved 1500C       # Contest 1500, Problem C

# Alternative syntax (if supported)
!verify 467B
!check 1200A
```

## Prerequisites

⚠️ **Important**: Before using this command, you must have your Codeforces account linked:

```bash
!account your_codeforces_handle
```

## How It Works

When you execute this command, the bot performs the following verification process:

1. **Input Validation** - Checks if the problem format is correct (number + letter/index)
2. **Account Verification** - Confirms you have a linked Codeforces account
3. **Database Check** - Verifies if the problem is already marked as solved
4. **API Query** - Connects to `https://codeforces.com/api/user.status?handle=your_handle`
5. **Submission Search** - Examines all your submissions to find a successful solution
6. **Verdict Verification** - Confirms the submission verdict is "OK" (Accepted)
7. **Database Update** - Marks the problem as solved if verification succeeds
8. **Reward Processing** - Awards coins and updates statistics

## Verification Criteria

For a problem to be considered solved, it must meet:

- ✅ **Correct Contest ID** - Submission must be from the specified contest
- ✅ **Correct Problem Index** - Submission must be for the specific problem (A, B, C, etc.)
- ✅ **"OK" Verdict** - Submission must be fully accepted
- ✅ **User Ownership** - Submission must belong to your Codeforces account
- ✅ **Complete Solution** - Must pass all test cases

## Bot Responses

### Success Response 🎉

```
🎉 Problem solved and verified!

✅ Problem 467B has been marked as solved
👤 Handle: your_handle
🏆 Congratulations on the successful solution!
💰 +2 coins earned (Rating: 1400)
📊 Problems solved: 15 → 16
```

### Problem Not Solved ❌

```
❌ Problem not solved

🔍 No successful solution found for problem 467B
👤 Handle: your_handle

💡 Possible reasons:
• Problem hasn't been solved yet
• Solution didn't pass all test cases
• Problem doesn't exist or format is incorrect
• Submission is still being judged

Keep trying! 💪
```

### Already Solved ℹ️

```
ℹ️ Problem 467B is already marked as solved
📊 First solved on: 2025-11-09
💰 Coins already earned for this problem
```

## Error Responses

### Format Errors
```
❌ Invalid problem format. Use format: 467B (contest_id + index)
❌ Please provide a problem ID. Usage: !solved 467B
❌ Contest ID must be a valid number
```

### Account Errors
```
❌ No Codeforces account linked. Use !account <handle> first
❌ Unable to access your Codeforces profile. Ensure it's public
```

### API Errors
```
❌ Error connecting to Codeforces API. Please try again later
❌ Codeforces API is temporarily unavailable
❌ Rate limit exceeded. Please wait before retrying
```

## Valid Problem Formats

### ✅ Correct Formats

| Format | Description | Example |
|--------|-------------|---------|
| `467B` | Contest 467, Problem B | Classic format |
| `1200A` | Contest 1200, Problem A | Standard contest |
| `842D2` | Contest 842, Problem D2 | Multi-part problem |
| `1500C` | Contest 1500, Problem C | Educational round |
| `100A` | Contest 100, Problem A | Old contest |

### ❌ Incorrect Formats

| Format | Issue | Correct Version |
|--------|-------|----------------|
| `467 B` | Contains spaces | `467B` |
| `B467` | Wrong order | `467B` |
| `467` | Missing problem index | `467B` |
| `B` | Missing contest ID | `467B` |
| `contest467B` | Invalid prefix | `467B` |

## Reward System

### Coin Rewards

Earn coins based on problem difficulty:

| Rating Range | Base Reward | Streak Bonus |
|--------------|-------------|--------------|
| **800-1200** | 1 coin | +0.5 |
| **1300-1600** | 2 coins | +1 |
| **1700-2000** | 3 coins | +1.5 |
| **2100-2400** | 4 coins | +2 |
| **2500+** | 5 coins | +2.5 |

### Additional Bonuses

- **🔥 Daily Streak** - Extra coins for consecutive days
- **🎯 First Solve** - Bonus for being first on server to solve
- **📈 Difficulty Jump** - Bonus for solving significantly harder problems
- **⏰ Contest Time** - Bonus for solving during live contests

## Use Cases & Examples

### Scenario 1: Recent Problem Solved
```bash
User: !solved 1200A
Bot: 🎉 Problem solved and verified! 
     ✅ Problem 1200A has been marked as solved
     💰 +1 coin earned (Rating: 800)
```

### Scenario 2: Attempted but Not Solved
```bash
User: !solved 1500D
Bot: ❌ Problem not solved
     🔍 No successful solution found for problem 1500D
     💡 You have 3 submissions but none were accepted
```

### Scenario 3: Verifying Old Problem
```bash
User: !solved 4A
Bot: 🎉 Problem solved and verified!
     ✅ Problem 4A has been marked as solved
     📅 Original solve date: 2023-08-15
     💰 +1 coin earned (Rating: 800)
```

### Scenario 4: During Contest
```bash
User: !solved 1600C
Bot: 🎉 Problem solved and verified!
     ✅ Problem 1600C has been marked as solved
     🏆 Contest bonus: +1 coin
     💰 Total earned: 4 coins (Rating: 1600 + contest bonus)
```

## Advanced Features

### Submission Analysis

The bot can provide detailed feedback:

```
📊 Submission Analysis for 1400B:
• Total attempts: 7
• Wrong answers: 4
• Time limit exceeded: 2
• Accepted: 1
• Time to solve: 45 minutes
• Language used: C++17
```

### Progress Tracking

Track your solving patterns:

- **📈 Difficulty Progression** - See how your skill level improves
- **🔥 Streak Tracking** - Monitor daily solving streaks  
- **🎯 Category Analysis** - Performance by problem type
- **⏰ Time Patterns** - When you solve most problems

## Integration with Other Commands

### Related Commands

| Command | Purpose | Relationship |
|---------|---------|--------------|
| `!account <handle>` | Link Codeforces profile | **Required** for !solved |
| `!problem [difficulty]` | Get problem recommendations | Provides problems to solve |
| `!stats [user]` | View solving statistics | Shows problems marked by !solved |
| `!leaderboard` | Server rankings | Includes problems from !solved |
| `!balance` | Check coin balance | Updated by !solved rewards |

### Command Workflow

```
!account → !problem → solve on CF → !solved → !stats
   ↓          ↓           ↓           ↓        ↓
 Link      Get        Solve      Verify    View
Profile   Problem   on Site    Solution  Progress
```

## Best Practices

### For Maximum Efficiency

1. **Solve First, Verify Later** - Focus on solving, then batch verify
2. **Use During Streaks** - Verify daily to maintain streak bonuses  
3. **Check Format** - Double-check problem format before submitting
4. **Monitor API Status** - Be aware of Codeforces maintenance times

### For Learning

1. **Track Progress** - Regular verification helps monitor improvement
2. **Celebrate Milestones** - Use verification as motivation
3. **Analyze Patterns** - Look at your solving trends over time
4. **Challenge Yourself** - Gradually increase problem difficulty

## Troubleshooting

### Problem Solved but Not Detected

**Check These First:**
1. **Format Accuracy** - Ensure exact format `contestID + index`
2. **Account Status** - Verify account linking with `!account`
3. **Profile Privacy** - Confirm Codeforces profile is public
4. **Submission Status** - Ensure verdict is exactly "OK"
5. **API Timing** - Sometimes there's a delay; retry after a few minutes

**Common Issues:**
- **Partial Acceptance** - Some contests have pretests; ensure full acceptance
- **Wrong Problem** - Double-check you solved the correct problem variant
- **Recent Submissions** - Very recent submissions might not be immediately visible

### Command Not Responding

**Possible Causes:**
1. **API Downtime** - Codeforces API might be under maintenance
2. **Network Issues** - Temporary connectivity problems
3. **Rate Limiting** - Too many requests; wait before retrying
4. **Bot Permissions** - Bot might lack channel permissions

**Solutions:**
1. **Wait and Retry** - API issues are usually temporary
2. **Check Codeforces** - Visit codeforces.com to confirm site status
3. **Contact Admins** - If persistent, report to server administrators
4. **Alternative Verification** - Some servers have manual verification options

### Submission Not Found

**Why This Happens:**
- **Private Profile** - Your Codeforces profile must be public
- **Handle Mismatch** - Linked handle differs from actual handle
- **Contest Visibility** - Some contests have delayed public results
- **API Cache** - Codeforces API might be returning cached data

**How to Fix:**
1. **Make Profile Public** - Check your Codeforces privacy settings
2. **Re-link Account** - Use `!account` again with correct handle
3. **Wait for Results** - Contest results might not be immediately public
4. **Manual Check** - Verify the submission exists on your Codeforces profile

## Security & Privacy

### Data Protection

- 🔒 **Public Data Only** - Only accesses publicly available profile information
- 👤 **Personal Verification** - Users can only verify their own problems
- 📝 **Local Storage** - Solved problems are stored locally on the server
- 🚫 **No Modification** - Bot never modifies your Codeforces account

### Anti-Cheat Measures

- **🔍 Submission Validation** - Verifies actual submissions on Codeforces
- **⏰ Timestamp Checking** - Prevents backdating of solutions
- **🛡️ Handle Verification** - Ensures submission ownership
- **📊 Pattern Analysis** - Detects unusual solving patterns

### Privacy Considerations

- **🌐 Public Profile Required** - Your Codeforces profile must be public
- **📊 Server Statistics** - Solved problems contribute to server statistics  
- **🏆 Leaderboards** - Your progress appears in server rankings
- **💰 Transparent Rewards** - Coin earnings are logged and auditable

## Performance & Limitations

### API Rate Limits

- **⏱️ Codeforces Limits** - Respects official API rate limits
- **🔄 Retry Logic** - Automatically retries failed requests
- **⏰ Cooldown Periods** - May impose brief cooldowns during high usage
- **📊 Fair Usage** - Rate limiting ensures fair access for all users

### System Limitations

- **🌐 Internet Dependency** - Requires active internet connection to Codeforces
- **🔄 API Availability** - Subject to Codeforces API uptime
- **📝 Data Accuracy** - Relies on Codeforces data accuracy
- **⚡ Response Time** - May be slower during peak usage periods

## Future Enhancements

Planned improvements for the !solved command:

- [ ] **Batch Verification** - Verify multiple problems at once
- [ ] **Auto-Detection** - Automatically detect new solutions
- [ ] **Solution Analysis** - Provide feedback on solution quality
- [ ] **Team Challenges** - Group problem-solving verification
- [ ] **Contest Integration** - Special handling for ongoing contests

---

**💡 Pro Tip**: Use `!solved` regularly to track your progress and earn rewards. Consistent verification helps maintain accurate statistics and maximizes your coin earnings!

*Last updated: November 2025*