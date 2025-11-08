// =====================
//   Imports
// =====================
use crate::Bot;
use serenity::model::guild::Guild;

// =====================
//   Guild Create Event Handler
// =====================
// Handles when the bot joins a new guild or when a guild becomes available
pub async fn handle_guild_create(
    bot: &Bot,
    ctx_http: &serenity::http::Http,
    guild: Guild,
    is_new: Option<bool>,
) {
    // Keep reference to ctx_http (currently unused but available for future use)
    let _ = ctx_http;

    // =====================
    //   Log Guild Creation
    // =====================
    println!("Guild created: {} (ID: {})", guild.name, guild.id);

    // =====================
    //   Database Operations
    // =====================
    // Insert guild into database or ignore if it already exists
    let _ = sqlx::query(
        "INSERT INTO guild_config (guild_id, guild_name)
        VALUES ($1, $2)
        ON CONFLICT (guild_id) DO NOTHING",
    )
    .bind(guild.id.get() as i64)
    .bind(&guild.name)
    .execute(&bot.db)
    .await
    .expect("Error inserting guild into database");

    // =====================
    //   Confirm Database Entry
    // =====================
    println!("Guild entry secured in database ✅");

    // =====================
    //   Check If New Guild
    // =====================
    // Check if this is a new guild (bot just joined) vs existing guild
    if let Some(true) = is_new {
        println!("This guild is new to the bot!");
    }
}
