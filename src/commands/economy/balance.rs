// =====================
//   Imports and Dependencies
// =====================

use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::guild::PartialGuild;
use sqlx::PgPool;
use std::sync::Arc;

// =====================
//   Balance Command Implementation
// =====================

// Main function to handle the !balance command
// Shows the user's current coin balance
pub async fn execute(
    http: &Arc<Http>,
    msg: &Message,
    db: &PgPool,
    guild: &PartialGuild,
) -> Result<(), serenity::Error> {
    // =====================
    //   Query User Balance
    // =====================

    // Get user's coin balance from database
    match sqlx::query_scalar::<_, Option<i64>>(
        "SELECT coins FROM user_info WHERE guild_id = $1 AND user_id = $2",
    )
    .bind(guild.id.get() as i64)
    .bind(msg.author.id.get() as i64)
    .fetch_optional(db)
    .await
    {
        Ok(Some(Some(coins))) => {
            // =====================
            //   Display Balance
            // =====================

            let response = format!(
                "💰 **Balance de monedas**\n\n\
                👤 Usuario: <@{}>\n\
                🪙 **Monedas:** `{}`\n\n\
                💡 *Resuelve problemas de Codeforces con `!solved` para ganar más monedas!*",
                msg.author.id, coins
            );

            let _ = msg.channel_id.say(http, &response).await;
        }
        Ok(Some(None)) | Ok(None) => {
            // =====================
            //   User Not Found Response
            // =====================

            let _ = msg
                .channel_id
                .say(
                    http,
                    "❌ No tienes un perfil en este servidor. Usa `!account tu_handle` para crear uno y comenzar a ganar monedas.",
                )
                .await;
        }
        Err(e) => {
            // =====================
            //   Database Error Response
            // =====================

            eprintln!("Database error in balance command: {}", e);
            let _ = msg
                .channel_id
                .say(
                    http,
                    "❌ Error al acceder a la base de datos. Intenta de nuevo más tarde.",
                )
                .await;
        }
    }

    Ok(())
}
