// =====================
//   Imports and Dependencies
// =====================

// necessary imports
use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

// import Modules
mod api;
mod commands;
mod events;
mod tasks;

// =====================
//   Bot Structure Definition
// =====================

// Define principal bot
pub struct Bot {
    pub db: PgPool,
}

// =====================
//   Event Handler Implementation
// =====================

// Define the event handler struct
struct Handler {
    bot: Arc<Bot>,
}

// Implement the EventHandler trait for the Handler struct
#[async_trait]
impl EventHandler for Handler {
    // Confirm connected bot to discord
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Bot connected as {} with the id of {}",
            ready.user.name, ready.user.id
        );

        let bot = Arc::clone(&self.bot);
        let ctx_http = Arc::clone(&ctx.http);

        // Start daily task in background
        tokio::spawn(async move {
            if let Err(e) = tasks::daily::start_daily_task(bot, ctx_http).await {
                eprintln!("Daily task error: {}", e);
            }
        });
    }

    // Handle incoming messages
    async fn message(&self, ctx: Context, msg: Message) {
        events::message::handle_message(&self.bot, &ctx.http, msg).await;
    }

    // Handle guild creation/joining
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        events::guild::handle_guild_create(&self.bot, &ctx.http, guild, is_new).await;
    }

    // Handle new member joining (commented out)
    //async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
    //    events::member::handle_member_join(&self.bot, &ctx.http, new_member).await;
    //}
}

// =====================
//   Main Function - Bot Initialization
// =====================

// Main function to initialize the bot
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // =====================
    //   Environment Variables
    // =====================

    // Load environment variables
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    // =====================
    //   Database Connection
    // =====================

    // Connect to the database
    let db = PgPool::connect(&db_url).await?;
    println!("Connected to database ✅");

    // =====================
    //   Database Table Creation
    // =====================

    // Create guild configuration table if it doesn't exist
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS guild_config (
            guild_id BIGINT PRIMARY KEY,
            guild_name TEXT,
            guild_prefix TEXT DEFAULT '!',
            welcome_channel_id BIGINT,
            daily_channel_id BIGINT,
            daily_hour INT,
            daily_minute INT,
            min_rating INT DEFAULT 800,
            max_rating INT DEFAULT 1200,
            level_system_enabled BOOLEAN DEFAULT true
        )",
    )
    .execute(&db)
    .await?;

    // Create user configuration table if it doesn't exist
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_config (
            guild_id BIGINT,
            user_id BIGINT,
            xp bigint DEFAULT 0,
            level bigint DEFAULT 1,
            coins bigint DEFAULT 0,
            warns int DEFAULT 0,
            codeforces_handle TEXT,
            codeforces_rating INT DEFAULT 0,
            codeforces_rank TEXT DEFAULT 'unrated',
            primary key (guild_id, user_id),
            foreign key (guild_id) references guild_config(guild_id)
        )",
    )
    .execute(&db)
    .await?;

    // Create user_solved_problem table if it doesn't exist
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_solved_problem (
            guild_id BIGINT,
            user_id BIGINT,
            problem_id TEXT,
            primary key (guild_id, user_id, problem_id),
            foreign key (guild_id, user_id) references user_config(guild_id, user_id)
        )",
    )
    .execute(&db)
    .await?;

    // =====================
    //   Bot Instance Creation
    // =====================

    // Initialize the bot struct
    let bot = Arc::new(Bot { db });

    // =====================
    //   Discord Client Setup
    // =====================

    // Set up the Discord client with intents
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { bot })
        .await?;

    // =====================
    //   Bot Startup
    // =====================

    // Start the bot
    if let Err(why) = client.start().await {
        println!("Error starting bot: {:?}", why);
    }

    Ok(())
}
