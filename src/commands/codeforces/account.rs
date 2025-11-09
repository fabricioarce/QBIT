// =====================
//   Imports and Dependencies
// =====================

use crate::api;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::guild::PartialGuild;
use sqlx::PgPool;
use std::sync::Arc;

// =====================
//   Account Command Implementation
// =====================

// Main function to handle the !account command
// Links the user's Codeforces handle to their Discord ID and saves profile info
pub async fn execute(
    http: &Arc<Http>,
    msg: &Message,
    db: &PgPool,
    guild: &PartialGuild,
) -> Result<(), serenity::Error> {
    // =====================
    //   Extract Codeforces Handle
    // =====================

    // Extract Codeforces handle from message content after "!account "
    let handle = msg.content["!account ".len()..].trim();

    // Check if handle was provided
    if handle.is_empty() {
        let _ = msg
            .channel_id
            .say(
                http,
                "❌ Por favor proporciona un handle de Codeforces. Uso: `!account tu_handle`",
            )
            .await;
        return Ok(());
    }

    // =====================
    //   API Call to Codeforces
    // =====================

    // Fetch user information from Codeforces API
    match api::codeforces::get_user_info(handle).await {
        Ok(user_info) => {
            // Check if API returned successful response
            if user_info.status != "OK" || user_info.result.is_empty() {
                let _ = msg.channel_id.say(http,
                    &format!("❌ No se encontró el usuario `{}` en Codeforces. Verifica que el handle sea correcto.", handle)
                ).await;
                return Ok(());
            }

            // Get the first (and should be only) user from results
            let user_data = &user_info.result[0];

            // =====================
            //   Database Update
            // =====================

            // Extract relevant information with defaults
            let codeforces_handle = user_data.handle.as_deref().unwrap_or(handle);
            let codeforces_rating = user_data.rating.unwrap_or(0);
            let codeforces_rank = user_data.rank.as_deref().unwrap_or("unrated");

            // Insert or update user information in database
            match sqlx::query(
                "INSERT INTO user_info (guild_id, user_id, codeforces_handle, codeforces_rating, codeforces_rank)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (guild_id, user_id)
                DO UPDATE SET
                    codeforces_handle = EXCLUDED.codeforces_handle,
                    codeforces_rating = EXCLUDED.codeforces_rating,
                    codeforces_rank = EXCLUDED.codeforces_rank"
            )
            .bind(guild.id.get() as i64)
            .bind(msg.author.id.get() as i64)
            .bind(codeforces_handle)
            .bind(codeforces_rating)
            .bind(codeforces_rank)
            .execute(db)
            .await
            {
                Ok(_) => {
                    // =====================
                    //   Success Response
                    // =====================

                    // Send confirmation message with user info
                    let response = format!(
                        "✅ **Cuenta de Codeforces vinculada exitosamente!**\n\n\
                        👤 **Handle:** `{}`\n\
                        🏆 **Rating:** `{}`\n\
                        🎯 **Rank:** `{}`\n\
                        📊 **Rating Máximo:** `{}`",
                        codeforces_handle,
                        codeforces_rating,
                        codeforces_rank,
                        user_data.maxRating.unwrap_or(codeforces_rating)
                    );

                    let _ = msg.channel_id.say(http, &response).await;
                }
                Err(e) => {
                    // =====================
                    //   Database Error Response
                    // =====================

                    eprintln!("Database error in account command: {}", e);
                    let _ = msg.channel_id.say(http,
                        "❌ Error al guardar la información en la base de datos. Intenta de nuevo más tarde."
                    ).await;
                }
            }
        }
        Err(e) => {
            // =====================
            //   API Error Response
            // =====================

            eprintln!("Codeforces API error: {}", e);
            let _ = msg
                .channel_id
                .say(
                    http,
                    "❌ Error al conectar con la API de Codeforces. Intenta de nuevo más tarde.",
                )
                .await;
        }
    }

    Ok(())
}
