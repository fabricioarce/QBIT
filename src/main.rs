// necessary imports
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;
use std::sync::Arc;

// import Modules

mod events;
mod tasks;
mod api;
mod commands;

// Define principal bot
pub struct Bot {
    pub db: PgPool,
}

// Define the event handler struct
struct Handler {
    bot: Arc<Bot>,
}

// Implement the EventHandler trait for the Handler struct
#[async_trait]
impl EventHandler for Handler {
    // Confrim connected bot to discord 
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot connected as {} with the id of {}", ready.user.name, ready.user.id);

        let bot = Arc::clone(&self.bot);
        let ctx_http = Arc::clone(&ctx.http);
        
        tokio::spawn(async move {
            if let Err(e) = tasks::daily::start_daily_task(bot, ctx_http).await {
                eprintln!("Daily task error: {}", e);
            }
        });
    }

    async fn message(&self, ctx: Context, msg: Message) {
        events::message::handle_message(&self.bot, &ctx.http, msg).await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        events::guild::handle_guild_create(&self.bot, &ctx.http, guild, is_new).await;
    }

    //async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
    //    events::member::handle_member_join(&self.bot, &ctx.http, new_member).await;
    //}
}

// Main function to initialize the bot
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Load environment variables
    let token = env::var("DISCORD_TOKEN").expect("No se encontró DISCORD_TOKEN");
    let db_url = env::var("DATABASE_URL").expect("No se encontró DATABASE_URL");

    // Connect to the database
    let db = PgPool::connect(&db_url).await?;
    println!("Conectado a la base de datos ✅");

        let _ = sqlx::query!(
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
            )"
        )
        .execute(&db)
        .await?;

    // Initialize the bot struct
    let bot = Arc::new(Bot { db });

    // Set up the Discord client with intents
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { bot })
        .await?;

    if let Err(why) = client.start().await {
        println!("Error al iniciar el bot: {:?}", why);
    }

    Ok(())
}