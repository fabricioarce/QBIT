// =====================
//   Imports
// =====================
use serenity::http::Http;
use serenity::model::channel::Message;
use sqlx::PgPool;
use std::sync::Arc;

// =====================
//   Set Hour Command Execution
// =====================
// Sets the time when daily problems will be posted
pub async fn execute(http: &Arc<Http>, msg: &Message, db: &PgPool) -> Result<(), serenity::Error> {
    // =====================
    //   Extract Time from Command
    // =====================
    // Get the time after the command
    let hora = msg.content["!sethour ".len()..].trim();

    // =====================
    //   Parse Time Format
    // =====================
    // Split time into hours and minutes (HH:MM format)
    if let Some((horas_str, minutos_str)) = hora.split_once(':') {
        if let (Ok(horas), Ok(minutos)) = (horas_str.parse::<i32>(), minutos_str.parse::<i32>()) {
            // =====================
            //   Validate Time Range
            // =====================
            // Check if hours and minutes are within valid range
            if horas >= 0 && horas < 24 && minutos >= 0 && minutos < 60 {
                // =====================
                //   Get Guild ID
                // =====================
                let guild_id = msg.guild_id.unwrap().get() as i64;

                // =====================
                //   Update Database
                // =====================
                // Save the daily report time in the database
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
                //   Handle Database Response
                // =====================
                match result {
                    Ok(_) => {
                        // =====================
                        //   Success Message
                        // =====================
                        let response =
                            format!("⏰ Daily report time set to {:02}:{:02}", horas, minutos);
                        msg.channel_id.say(http, response).await?;
                    }
                    Err(_) => {
                        // =====================
                        //   Database Error
                        // =====================
                        println!("Error saving time");
                        msg.channel_id.say(http, "❌ Error saving the time").await?;
                    }
                }
            } else {
                // =====================
                //   Invalid Time Range
                // =====================
                msg.channel_id
                    .say(http, "❌ Invalid time. Use format HH:MM (00:00 - 23:59)")
                    .await?;
            }
        } else {
            // =====================
            //   Parse Error
            // =====================
            msg.channel_id
                .say(
                    http,
                    "❌ Incorrect format. Use: !sethour HH:MM (example: !sethour 09:00)",
                )
                .await?;
        }
    } else {
        // =====================
        //   Missing Colon Error
        // =====================
        msg.channel_id
            .say(
                http,
                "❌ Incorrect format. Use: !sethour HH:MM (example: !sethour 09:00)",
            )
            .await?;
    }

    Ok(())
}
