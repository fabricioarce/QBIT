// =====================
//   Imports and Dependencies
// =====================

use serenity::http::Http;
use serenity::model::channel::Message;
use sqlx::PgPool;
use std::sync::Arc;

// =====================
//   Set Hour Command Implementation
// =====================

// Main function to handle the !sethora command
// Sets the time for daily problem posting in HH:MM format
pub async fn execute(http: &Arc<Http>, msg: &Message, db: &PgPool) -> Result<(), serenity::Error> {
    // =====================
    //   Command Input Parsing
    // =====================

    // Extract the time after the command
    let hora = msg.content["!sethora ".len()..].trim();

    // Parse time format (HH:MM)
    if let Some((horas_str, minutos_str)) = hora.split_once(':') {
        if let (Ok(horas), Ok(minutos)) = (horas_str.parse::<i32>(), minutos_str.parse::<i32>()) {
            // =====================
            //   Time Validation
            // =====================

            // Validate hour and minute ranges (24-hour format)
            if horas >= 0 && horas < 24 && minutos >= 0 && minutos < 60 {
                // =====================
                //   Database Operations
                // =====================

                // Get guild ID for database update
                let guild_id = msg.guild_id.unwrap().get() as i64;

                // Update daily hour and minute in guild configuration
                let result = sqlx::query(
                    "UPDATE guild_config
                    SET daily_hour = $1, daily_minute = $2
                    WHERE guild_id = $3",
                )
                .bind(horas)
                .bind(minutos)
                .bind(guild_id)
                .execute(db)
                .await;

                // =====================
                //   Success/Error Response
                // =====================

                match result {
                    Ok(_) => {
                        // Success: Confirm time has been set
                        let response =
                            format!("⏰ Daily report time set to {:02}:{:02}", horas, minutos);
                        msg.channel_id.say(http, response).await?;
                    }
                    Err(_) => {
                        // Database error occurred
                        println!("Error saving time");
                        msg.channel_id.say(http, "❌ Error saving the time").await?;
                    }
                }
            } else {
                // =====================
                //   Invalid Time Range Error
                // =====================

                msg.channel_id
                    .say(http, "❌ Invalid time. Use HH:MM format (00:00 - 23:59)")
                    .await?;
            }
        } else {
            // =====================
            //   Invalid Format Error
            // =====================

            msg.channel_id
                .say(
                    http,
                    "❌ Incorrect format. Use: !sethora HH:MM (example: !sethora 09:00)",
                )
                .await?;
        }
    } else {
        // =====================
        //   Missing Colon Separator Error
        // =====================

        msg.channel_id
            .say(
                http,
                "❌ Incorrect format. Use: !sethora HH:MM (example: !sethora 09:00)",
            )
            .await?;
    }

    Ok(())
}
