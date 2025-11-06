use serenity::http::Http;
use serenity::model::guild::Guild;
use std::sync::Arc;
use crate::Bot;

pub async fn handle_guild_create(
    bot: &Bot, 
    _http: &Arc<Http>, 
    guild: Guild,
    is_new: Option<bool>,
) {
    println!("Guild created: {} (ID: {})", guild.name, guild.id);

    // Check if the guild is new and perform any initialization if necessary
    if is_new.unwrap_or(false) {
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
                max_rating INT DEFAULT 2000,
                level_system_enabled BOOLEAN DEFAULT true
            )"
        )
        .execute(&bot.db)
        .await;

        println!("Tables created for {}", guild.name);
    }
}