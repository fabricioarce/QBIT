// =====================
//   Imports and Dependencies
// =====================

use crate::Bot;
use serenity::model::guild::Guild;

// =====================
//   Guild Event Handler
// =====================

// Handle guild creation/joining events - called when bot joins a new guild or reconnects
pub async fn handle_guild_create(
    bot: &Bot,
    ctx_http: &serenity::http::Http,
    guild: Guild,
    is_new: Option<bool>,
) {
    // =====================
    //   Event Logging
    // =====================

    // Suppress unused variable warning for ctx_http
    let _ = ctx_http;
    println!("Guild created: {} (ID: {})", guild.name, guild.id);

    // =====================
    //   Database Operations
    // =====================

    // Insert or update guild configuration in database
    let _ = sqlx::query(
        "INSERT INTO guild_config (guild_id, guild_name)
        VALUES ($1, $2)
        ON CONFLICT (guild_id) DO NOTHING",
    )
    .bind(guild.id.get() as i64)
    .bind(&guild.name)
    .execute(&bot.db)
    .await
    .expect("Error inserting guild in database");

    println!("Guild entry secured in database ✅");

    // =====================
    //   New Guild Detection
    // =====================

    // Check if this is a genuinely new guild for the bot
    if let Some(true) = is_new {
        println!("This guild is new to the bot!");
    }
}
