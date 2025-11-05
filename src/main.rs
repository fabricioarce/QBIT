// necessary imports
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;
use std::sync::Arc;

// import Modules


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
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot connected as {} with the id of {}", ready.user.name, ready.user.id);

        //Initialize daily tasks
        //let bot = Arc::clone(&self.bot);
        //let ctx_clone = ctx.clone();
        //tokio::spawn(async move {
        //     tasks::daily_tasks::start_daily_tasks(bot).await();
        //};
    }
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
    client.start().await?;

    Ok(())
}
