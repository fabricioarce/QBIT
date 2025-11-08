// =====================
//   Imports
// =====================
use serenity::http::Http;
use serenity::model::channel::Message;
use sqlx::PgPool;
use std::sync::Arc;

// =====================
//   Set Channel Command Execution
// =====================
// Sets the channel for daily Codeforces problem posting
pub async fn execute(http: &Arc<Http>, msg: &Message, db: &PgPool) -> Result<(), serenity::Error> {
    // =====================
    //   Parse Command Arguments
    // =====================
    // Extract the channel mention from the message
    let mention = msg.content["!setchannel ".len()..].trim();

    // =====================
    //   Validate Channel Mention Format
    // =====================
    // Check if the mention follows Discord channel format: <#ID>
    if let Some(id_str) = mention.strip_prefix("<#").and_then(|s| s.strip_suffix(">")) {
        if let Ok(channel_id) = id_str.parse::<i64>() {
            // =====================
            //   Get Guild ID
            // =====================
            let guild_id: i64 = msg.guild_id.unwrap().get() as i64;

            // =====================
            //   Update Database
            // =====================
            // Save the channel ID in the guild configuration
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
            //   Handle Database Response
            // =====================
            match result {
                Ok(_) => {
                    // =====================
                    //   Success Message
                    // =====================
                    let response = format!("✅ Daily report channel set to <#{}>", channel_id);
                    msg.channel_id.say(http, response).await?;
                }
                Err(_) => {
                    // =====================
                    //   Database Error
                    // =====================
                    println!("Error saving channel");
                    msg.channel_id.say(http, "❌ Error saving channel").await?;
                }
            }
        } else {
            // =====================
            //   Invalid Channel ID
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
