// =====================
//   Imports and Dependencies
// =====================

use serenity::http::Http;
use serenity::model::channel::Message;
use sqlx::PgPool;
use std::sync::Arc;

// =====================
//   Set Channel Command Implementation
// =====================

// Main function to handle the !setchannel command
// Configures which channel will receive daily Codeforces problems
pub async fn execute(http: &Arc<Http>, msg: &Message, db: &PgPool) -> Result<(), serenity::Error> {
    // =====================
    //   Command Input Parsing
    // =====================

    // Extract the channel mention from the command
    let mention = msg.content["!setchannel ".len()..].trim();

    // =====================
    //   Channel ID Validation
    // =====================

    // Validate mention format: <#ID> and extract channel ID
    if let Some(id_str) = mention.strip_prefix("<#").and_then(|s| s.strip_suffix(">")) {
        if let Ok(channel_id) = id_str.parse::<i64>() {
            // =====================
            //   Database Operations
            // =====================

            // Get guild ID for database update
            let guild_id: i64 = msg.guild_id.unwrap().get() as i64;

            // Update daily channel ID in guild configuration
            let result = sqlx::query(
                "UPDATE guild_config
                 SET daily_channel_id = $1
                 WHERE guild_id = $2",
            )
            .bind(channel_id)
            .bind(guild_id)
            .execute(db)
            .await;

            // =====================
            //   Success/Error Response
            // =====================

            match result {
                Ok(_) => {
                    // Success: Confirm channel has been configured
                    let response = format!("✅ Daily report channel set to <#{}>", channel_id);
                    msg.channel_id.say(http, response).await?;
                }
                Err(_) => {
                    // Database error occurred
                    println!("Error saving channel");
                    msg.channel_id
                        .say(http, "❌ Error saving the channel")
                        .await?;
                }
            }
        } else {
            // =====================
            //   Invalid Channel ID Error
            // =====================

            msg.channel_id.say(http, "❌ Invalid channel ID").await?;
        }
    } else {
        // =====================
        //   Invalid Format Error
        // =====================

        msg.channel_id
            .say(http, "❌ Incorrect format. Use: !setchannel #channel")
            .await?;
    }

    Ok(())
}
