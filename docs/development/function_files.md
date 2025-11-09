# Project Structure Guide

A comprehensive guide to the Bot Olim P-Code project structure, explaining where each component belongs and how they interact to create a scalable competitive programming Discord bot.

## 🏗️ Architecture Overview

The project follows a **modular, event-driven architecture** with clear separation of concerns:

- **Entry Point** (`main.rs`) - Application initialization and startup
- **Bot Layer** (`bot/`) - Discord integration and event handling  
- **Commands Layer** (`commands/`) - User command implementations
- **Services Layer** (`services/`) - External integrations and business logic
- **Database Layer** (`database/`) - Data persistence and queries
- **Utilities Layer** (`utils/`) - Shared helpers and tools

## 📁 Complete Project Structure

```
bot-olim-p-code/
├── Cargo.toml              # Project configuration and dependencies
├── .env                    # Environment variables (not in git)
├── .gitignore             # Git ignore patterns
├── dockerfile             # Docker container configuration
├── docker-compose.yaml    # Multi-container setup
├── README.md              # Project overview and quick start
│
├── src/
│   ├── main.rs            # 🚀 Application entry point
│   ├── lib.rs             # Public module exports
│   │
│   ├── bot/               # 🤖 Discord bot core
│   │   ├── mod.rs         # Bot module configuration
│   │   ├── handler.rs     # Discord event handler
│   │   └── context.rs     # Shared bot context and state
│   │
│   ├── commands/          # 💬 Discord slash commands
│   │   ├── mod.rs         # Command registration and routing
│   │   │
│   │   ├── codeforces/    # 🎯 Competitive programming commands
│   │   │   ├── mod.rs
│   │   │   ├── problem.rs    # !problem - Get random problems
│   │   │   ├── solved.rs     # !solved - Mark problems solved
│   │   │   ├── stats.rs      # !stats - User statistics
│   │   │   └── leaderboard.rs # !ranking - Server leaderboards
│   │   │
│   │   ├── economy/       # 💰 Virtual economy commands
│   │   │   ├── mod.rs
│   │   │   ├── balance.rs    # !balance - Check coin balance
│   │   │   ├── shop.rs       # !shop, !buy - Virtual shop
│   │   │   ├── transfer.rs   # !gift - Send coins to others
│   │   │   └── admin.rs      # !give_coins - Admin coin commands
│   │   │
│   │   ├── moderation/    # 🛡️ Server moderation commands
│   │   │   ├── mod.rs
│   │   │   ├── warn.rs       # !warn - Issue warnings
│   │   │   ├── strikes.rs    # !strikes - View warnings
│   │   │   ├── kick_ban.rs   # !kick, !ban - Member management
│   │   │   └── cleanup.rs    # !clear - Message cleanup
│   │   │
│   │   └── utility/       # 🔧 General utility commands
│   │       ├── mod.rs
│   │       ├── help.rs       # !help - Command documentation
│   │       ├── ping.rs       # !ping - Bot health check
│   │       ├── info.rs       # !info - Bot information
│   │       └── account.rs    # !account - Profile linking
│   │
│   ├── services/          # 🌐 External services and business logic
│   │   ├── mod.rs
│   │   │
│   │   ├── codeforces/    # Codeforces API integration
│   │   │   ├── mod.rs
│   │   │   ├── client.rs     # API client implementation
│   │   │   ├── models.rs     # Data structures (Problem, User, etc.)
│   │   │   ├── cache.rs      # In-memory problem caching
│   │   │   └── sync.rs       # Profile synchronization
│   │   │
│   │   ├── rewards/       # Reward system logic
│   │   │   ├── mod.rs
│   │   │   ├── calculator.rs # Calculate XP and coin rewards
│   │   │   ├── processor.rs  # Process reward transactions
│   │   │   └── achievements.rs # Achievement badge system
│   │   │
│   │   └── scheduler/     # Background task scheduling
│   │       ├── mod.rs
│   │       ├── daily_problem.rs # Daily problem distribution
│   │       ├── sync_profiles.rs # Periodic CF profile updates
│   │       └── cleanup.rs       # Database maintenance tasks
│   │
│   ├── database/          # 🗄️ Database layer
│   │   ├── mod.rs
│   │   ├── connection.rs     # Database connection pool
│   │   ├── models.rs         # Database entity structs
│   │   ├── migrations.rs     # Migration management
│   │   │
│   │   └── queries/       # SQL queries organized by entity
│   │       ├── mod.rs
│   │       ├── guilds.rs     # Guild/server operations
│   │       ├── users.rs      # User data operations
│   │       ├── problems.rs   # Solved problems tracking
│   │       ├── economy.rs    # Transactions and balances
│   │       ├── moderation.rs # Warnings and moderation logs
│   │       └── shop.rs       # Shop items and purchases
│   │
│   ├── tasks/             # ⏰ Background tasks and jobs
│   │   ├── mod.rs
│   │   ├── scheduler.rs      # Task scheduling system
│   │   ├── daily_problem.rs  # Daily problem task
│   │   ├── profile_sync.rs   # User profile synchronization
│   │   └── maintenance.rs    # Database cleanup and maintenance
│   │
│   └── utils/             # 🛠️ Shared utilities and helpers
│       ├── mod.rs
│       ├── embed_builder.rs  # Discord embed helpers
│       ├── permissions.rs    # Permission checking utilities
│       ├── formatters.rs     # Data formatting helpers
│       ├── validators.rs     # Input validation functions
│       ├── error.rs          # Error types and handling
│       └── constants.rs      # Application constants
│
├── migrations/            # 📊 Database migrations
│   ├── 001_initial_schema.sql
│   ├── 002_add_economy.sql
│   ├── 003_add_codeforces.sql
│   ├── 004_add_shop_system.sql
│   └── 005_add_moderation.sql
│
├── config/               # ⚙️ Configuration files
│   ├── default.toml      # Default configuration
│   ├── development.toml  # Development overrides
│   └── production.toml   # Production overrides
│
├── docs/                 # 📚 Documentation
│   ├── README.md         # Documentation index
│   ├── commands/         # Command documentation
│   ├── development/      # Development guides
│   └── deployment/       # Deployment guides
│
├── tests/               # 🧪 Test suites
│   ├── integration/     # Integration tests
│   ├── unit/           # Unit tests
│   └── fixtures/       # Test data and fixtures
│
└── scripts/            # 📜 Utility scripts
    ├── setup.sh        # Development setup
    ├── deploy.sh       # Deployment script
    └── migrate.sh      # Database migration runner
```

## 🚀 Core Components Deep Dive

### 1. Application Entry Point (`main.rs`)

**Purpose**: Initialize and start the bot application.

```rust
// main.rs responsibilities:
// - Load environment variables
// - Initialize database connection pool
// - Set up logging
// - Start background tasks
// - Initialize Discord bot
// - Handle graceful shutdown

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Environment setup
    dotenvy::dotenv().ok();
    tracing_subscriber::init();
    
    // Database initialization
    let database = database::connect().await?;
    database::migrate(&database).await?;
    
    // Background services
    let scheduler = tasks::start_scheduler(database.clone()).await?;
    
    // Discord bot
    let bot = bot::create_bot(database).await?;
    bot.start().await?;
    
    Ok(())
}
```

### 2. Bot Core (`bot/`)

**Purpose**: Handle Discord integration and events.

#### `bot/handler.rs` - Event Handler
```rust
// Handles all Discord events:
// - guild_create: Add new server to database
// - guild_delete: Clean up or mark server inactive  
// - message: Process XP, check for commands
// - member_join: Send welcome message, assign roles
// - member_leave: Clean up user data
// - reaction_add: Handle reaction-based features

impl EventHandler for BotEventHandler {
    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        // Add guild to database with default configuration
        if let Err(e) = database::queries::guilds::create_guild(&self.db, &guild).await {
            tracing::error!("Failed to create guild: {:?}", e);
        }
    }
    
    async fn message(&self, ctx: Context, msg: Message) {
        // Award XP for messages (with cooldown)
        // Check for auto-moderation triggers
        // Update user activity timestamp
    }
}
```

#### `bot/context.rs` - Shared Context
```rust
// Provides shared state and services to commands:
// - Database connection pool
// - Codeforces API client
// - Cache manager
// - Configuration settings

#[derive(Clone)]
pub struct BotContext {
    pub database: PgPool,
    pub codeforces: services::codeforces::Client,
    pub cache: Arc<RwLock<Cache>>,
    pub config: Config,
}
```

### 3. Commands Layer (`commands/`)

**Purpose**: Implement user-facing Discord commands.

#### Organization by Feature
- **`codeforces/`** - Competitive programming features
- **`economy/`** - Virtual currency and shop
- **`moderation/`** - Server management tools  
- **`utility/`** - General bot functions

#### Example Command Implementation
```rust
// commands/codeforces/solved.rs
pub async fn solved_command(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    options: &[ApplicationCommandInteractionDataOption],
) -> Result<(), CommandError> {
    let problem_id = get_problem_id_from_options(options)?;
    let user_id = interaction.user.id;
    let guild_id = interaction.guild_id.unwrap();
    
    // Verify user has linked Codeforces account
    let user_data = database::queries::users::get_user(&ctx.data, guild_id, user_id).await?;
    let cf_handle = user_data.codeforces_handle
        .ok_or(CommandError::NoLinkedAccount)?;
    
    // Check if problem is solved on Codeforces
    let is_solved = services::codeforces::verify_problem_solved(
        &ctx.data.codeforces, &cf_handle, &problem_id
    ).await?;
    
    if is_solved {
        // Award coins and XP
        let rewards = services::rewards::calculate_problem_reward(&problem_id).await?;
        database::queries::problems::mark_solved(
            &ctx.data, guild_id, user_id, &problem_id, rewards
        ).await?;
        
        // Send success response
        utils::respond_with_embed(ctx, interaction, success_embed).await?;
    } else {
        utils::respond_with_embed(ctx, interaction, not_solved_embed).await?;
    }
    
    Ok(())
}
```

### 4. Services Layer (`services/`)

**Purpose**: Business logic and external integrations.

#### `services/codeforces/client.rs`
```rust
// Handles all Codeforces API interactions:
// - Get user profile information
// - Fetch problem details
// - Verify problem submissions
// - Rate limiting and error handling

pub struct CodeforceClient {
    client: reqwest::Client,
    base_url: String,
    cache: Arc<RwLock<ProblemCache>>,
}

impl CodeforceClient {
    pub async fn get_user_status(&self, handle: &str) -> Result<Vec<Submission>, ApiError> {
        let url = format!("{}/api/user.status?handle={}", self.base_url, handle);
        let response = self.client.get(&url).send().await?;
        // Handle rate limiting, parse response, update cache
    }
}
```

#### `services/rewards/calculator.rs`
```rust
// Calculates rewards for various activities:
// - Problem difficulty-based coin rewards
// - Streak bonuses
// - Contest participation bonuses
// - Achievement unlocks

pub fn calculate_problem_reward(problem: &Problem, user_streak: u32) -> Rewards {
    let base_coins = match problem.rating {
        Some(rating) if rating < 1200 => 1,
        Some(rating) if rating < 1600 => 2,
        Some(rating) if rating < 2000 => 3,
        Some(rating) => 4,
        None => 1,
    };
    
    let streak_bonus = (user_streak as f32 * 0.1).min(2.0);
    let total_coins = base_coins + streak_bonus as u32;
    
    Rewards {
        coins: total_coins,
        xp: total_coins * 10,
        achievements: check_achievement_unlocks(problem, user_streak),
    }
}
```

### 5. Database Layer (`database/`)

**Purpose**: Data persistence and query management.

#### `database/queries/` Organization
Each file handles queries for a specific domain:

```rust
// database/queries/users.rs
pub async fn get_user(pool: &PgPool, guild_id: i64, user_id: i64) -> Result<UserData, DbError> {
    let user = sqlx::query_as!(
        UserData,
        "SELECT * FROM user_data WHERE guild_id = $1 AND user_id = $2",
        guild_id, user_id
    )
    .fetch_optional(pool)
    .await?;
    
    user.ok_or(DbError::UserNotFound)
}

pub async fn update_user_coins(
    pool: &PgPool, 
    guild_id: i64, 
    user_id: i64, 
    amount: i32
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE user_data SET coins = coins + $1 WHERE guild_id = $2 AND user_id = $3",
        amount, guild_id, user_id
    )
    .execute(pool)
    .await?;
    
    Ok(())
}
```

### 6. Background Tasks (`tasks/`)

**Purpose**: Scheduled and background operations.

#### `tasks/daily_problem.rs`
```rust
// Distributes daily problems to all active servers:
// - Select appropriate difficulty problems
// - Avoid recent duplicates  
// - Post to configured channels
// - Track engagement metrics

pub async fn distribute_daily_problems(ctx: Arc<BotContext>) -> Result<(), TaskError> {
    let active_guilds = database::queries::guilds::get_active_guilds(&ctx.database).await?;
    
    for guild in active_guilds {
        let problem = services::codeforces::select_daily_problem(&ctx.codeforces, &guild).await?;
        let embed = utils::embed_builder::create_daily_problem_embed(&problem);
        
        if let Some(channel_id) = guild.config.daily_problem_channel {
            send_to_channel(&ctx, channel_id, embed).await?;
            database::queries::daily_problems::record_daily_problem(
                &ctx.database, guild.id, &problem
            ).await?;
        }
    }
    
    Ok(())
}
```

## 🔄 Data Flow Examples

### User Solves a Problem Flow

```
1. User runs /solved 1234A
   ↓
2. commands/codeforces/solved.rs processes command
   ↓
3. services/codeforces/client.rs verifies solution
   ↓
4. services/rewards/calculator.rs calculates rewards
   ↓
5. database/queries/problems.rs marks problem solved
   ↓
6. database/queries/users.rs updates user stats
   ↓
7. database/queries/economy.rs records transaction
   ↓
8. utils/embed_builder.rs creates success message
```

### Daily Problem Distribution Flow

```
1. tasks/scheduler.rs triggers daily_problem task
   ↓
2. tasks/daily_problem.rs selects problems for each server
   ↓
3. services/codeforces/client.rs fetches problem details
   ↓
4. database/queries/daily_problems.rs records selections
   ↓
5. bot/context.rs sends messages to Discord channels
   ↓
6. database/queries/guilds.rs updates last activity
```

## 🎯 Adding New Features

### Where to Put New Code

| Feature Type | Location | Example |
|--------------|----------|---------|
| **New Command** | `commands/{category}/` | Contest command → `commands/codeforces/contest.rs` |
| **External API** | `services/{api}/` | AtCoder integration → `services/atcoder/` |
| **Database Entity** | `database/queries/` | Teams system → `database/queries/teams.rs` |
| **Background Task** | `tasks/` | Weekly reports → `tasks/weekly_reports.rs` |
| **Utility Function** | `utils/` | New embed type → `utils/embed_builder.rs` |

### Example: Adding Contest Feature

1. **Command**: `commands/codeforces/contest.rs`
   ```rust
   pub async fn contest_command(/* ... */) -> Result<(), CommandError> {
       // Command implementation
   }
   ```

2. **Service**: `services/codeforces/contests.rs`
   ```rust
   pub async fn get_upcoming_contests() -> Result<Vec<Contest>, ApiError> {
       // API integration
   }
   ```

3. **Database**: `database/queries/contests.rs`
   ```rust
   pub async fn save_contest_registration(/* ... */) -> Result<(), DbError> {
       // Database operations
   }
   ```

4. **Migration**: `migrations/006_add_contests.sql`
   ```sql
   CREATE TABLE contest_registrations (
       guild_id BIGINT NOT NULL,
       user_id BIGINT NOT NULL,
       contest_id INTEGER NOT NULL,
       -- ...
   );
   ```

## 🛠️ Development Guidelines

### Code Organization Principles

1. **Separation of Concerns**: Each module has a single, well-defined responsibility
2. **Dependency Direction**: Higher layers depend on lower layers (commands → services → database)
3. **Error Handling**: Each layer defines its own error types and conversions
4. **Testing**: Each module should be independently testable

### Module Naming Conventions

- **Verbs for Actions**: `calculate_reward()`, `verify_solution()`, `send_message()`
- **Nouns for Data**: `UserData`, `Problem`, `Transaction`
- **Descriptive Files**: `leaderboard.rs`, `daily_problem.rs`, `permissions.rs`

### Best Practices

1. **Keep Functions Small**: Each function should do one thing well
2. **Use Type Safety**: Leverage Rust's type system to prevent errors
3. **Document Public APIs**: All public functions should have documentation
4. **Handle Errors Gracefully**: Every external call should handle potential failures
5. **Log Important Events**: Use structured logging for debugging and monitoring

## 🔧 Configuration Management

### Environment Variables (`.env`)
```env
# Required
DISCORD_TOKEN=your_bot_token_here
DATABASE_URL=postgresql://user:pass@localhost/dbname

# Optional
RUST_LOG=info
CODEFORCES_API_BASE=https://codeforces.com
CACHE_TTL_SECONDS=3600
```

### Configuration Files (`config/`)
```toml
# config/default.toml
[bot]
default_prefix = "!"
command_cooldown = 3

[rewards]
base_coin_rate = 1
max_daily_streak_bonus = 10

[codeforces]
api_timeout_seconds = 30
rate_limit_per_minute = 60

[database]
connection_timeout = 30
max_connections = 10
```

## 📊 Testing Strategy

### Unit Tests
- Test individual functions in isolation
- Mock external dependencies
- Focus on business logic validation

### Integration Tests
- Test component interactions
- Use test database
- Verify end-to-end flows

### Test Organization
```
tests/
├── unit/
│   ├── commands/
│   ├── services/
│   └── database/
└── integration/
    ├── discord_flows/
    └── api_integration/
```

---

**💡 Pro Tips for Development**:

1. **Start with the Database**: Define your data model first, then build services around it
2. **Use the Type System**: Create specific types for IDs, handles, and domain concepts
3. **Think in Events**: Discord bots are event-driven - design your handlers accordingly
4. **Cache Strategically**: Cache expensive API calls but ensure data consistency
5. **Monitor Performance**: Log slow queries and API calls for optimization opportunities

This structure provides a solid foundation for a scalable, maintainable Discord bot that can grow with your community's needs.

*Last updated: November 2025*