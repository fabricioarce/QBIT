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
//   Solved Command Implementation
// =====================

// Main function to handle the !solved command
// Verifies if a user has solved a specific Codeforces problem and marks it as solved
pub async fn execute(
    http: &Arc<Http>,
    msg: &Message,
    db: &PgPool,
    guild: &PartialGuild,
) -> Result<(), serenity::Error> {
    // =====================
    //   Extract Problem ID
    // =====================

    // Extract problem ID from message content after "!solved "
    let problem_id = msg.content["!solved ".len()..].trim();

    // Check if problem ID was provided
    if problem_id.is_empty() {
        let _ = msg
            .channel_id
            .say(
                http,
                "❌ Por favor proporciona un ID de problema. Uso: `!solved 467B`",
            )
            .await;
        return Ok(());
    }

    // =====================
    //   Parse Contest ID and Index
    // =====================

    // Find where the digits end and letters begin
    let split_point = problem_id
        .chars()
        .position(|c| !c.is_ascii_digit())
        .unwrap_or(problem_id.len());

    if split_point == 0 || split_point == problem_id.len() {
        let _ = msg
            .channel_id
            .say(
                http,
                "❌ Formato de problema inválido. Usa el formato: `467B` (contest_id + index)",
            )
            .await;
        return Ok(());
    }

    let contest_id_str = &problem_id[..split_point];
    let index = &problem_id[split_point..];

    // Parse contest ID to number
    let contest_id: u32 = match contest_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            let _ = msg
                .channel_id
                .say(http, "❌ Contest ID inválido. Debe ser un número.")
                .await;
            return Ok(());
        }
    };

    // =====================
    //   Get User's Codeforces Handle
    // =====================

    // Query user's Codeforces handle from database
    let user_handle = match sqlx::query_scalar::<_, String>(
        "SELECT codeforces_handle FROM user_info WHERE guild_id = $1 AND user_id = $2",
    )
    .bind(guild.id.get() as i64)
    .bind(msg.author.id.get() as i64)
    .fetch_optional(db)
    .await
    {
        Ok(Some(handle)) => handle,
        Ok(None) => {
            let _ = msg
                .channel_id
                .say(
                    http,
                    "❌ No tienes una cuenta de Codeforces vinculada. Usa `!account tu_handle` primero.",
                )
                .await;
            return Ok(());
        }
        Err(e) => {
            eprintln!("Database error getting user handle: {}", e);
            let _ = msg
                .channel_id
                .say(
                    http,
                    "❌ Error al acceder a la base de datos. Intenta de nuevo más tarde.",
                )
                .await;
            return Ok(());
        }
    };

    // =====================
    //   Check if Already Marked as Solved
    // =====================

    // Check if problem is already marked as solved in database
    match sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM user_solved_problem WHERE guild_id = $1 AND user_id = $2 AND problem_id = $3)"
    )
    .bind(guild.id.get() as i64)
    .bind(msg.author.id.get() as i64)
    .bind(problem_id)
    .fetch_one(db)
    .await
    {
        Ok(true) => {
            let _ = msg
                .channel_id
                .say(
                    http,
                    &format!("ℹ️ El problema `{}` ya está marcado como resuelto.", problem_id),
                )
                .await;
            return Ok(());
        }
        Ok(false) => {
            // Continue with verification
        }
        Err(e) => {
            eprintln!("Database error checking solved status: {}", e);
            let _ = msg
                .channel_id
                .say(
                    http,
                    "❌ Error al verificar el estado del problema. Intenta de nuevo más tarde.",
                )
                .await;
            return Ok(());
        }
    }

    // =====================
    //   Fetch User Submissions from API
    // =====================

    // Get user submissions from Codeforces API
    match api::codeforces::get_user_status(&user_handle).await {
        Ok(user_status) => {
            // Check if API returned successful response
            if user_status.status != "OK" {
                let _ = msg
                    .channel_id
                    .say(
                        http,
                        "❌ Error al obtener las submissions de Codeforces. Intenta más tarde.",
                    )
                    .await;
                return Ok(());
            }

            // =====================
            //   Search for Solved Submission
            // =====================

            // Look for a successful submission of the specific problem
            let is_solved = user_status.result.iter().any(|submission| {
                // Check if this submission matches our problem
                submission.problem.contestId == Some(contest_id)
                    && submission.problem.index.to_uppercase() == index.to_uppercase()
                    && submission.verdict.as_deref() == Some("OK")
            });

            if is_solved {
                // =====================
                //   Mark Problem as Solved
                // =====================

                // Insert the solved problem into database
                match sqlx::query(
                    "INSERT INTO user_solved_problem (guild_id, user_id, problem_id) VALUES ($1, $2, $3)"
                )
                .bind(guild.id.get() as i64)
                .bind(msg.author.id.get() as i64)
                .bind(problem_id)
                .execute(db)
                .await
                {
                    Ok(_) => {
                        let _ = msg
                            .channel_id
                            .say(
                                http,
                                &format!(
                                    "🎉 **¡Problema resuelto verificado!**\n\n\
                                    ✅ El problema `{}` ha sido marcado como resuelto.\n\
                                    👤 Usuario: `{}`\n\
                                    🏆 ¡Felicitaciones por la solución exitosa!",
                                    problem_id, user_handle
                                ),
                            )
                            .await;
                    }
                    Err(e) => {
                        eprintln!("Database error inserting solved problem: {}", e);
                        let _ = msg
                            .channel_id
                            .say(
                                http,
                                "❌ Error al marcar el problema como resuelto. Intenta de nuevo más tarde.",
                            )
                            .await;
                    }
                }
            } else {
                // =====================
                //   Problem Not Solved Response
                // =====================

                let _ = msg
                    .channel_id
                    .say(
                        http,
                        &format!(
                            "❌ **Problema no resuelto**\n\n\
                            🔍 No se encontró una solución exitosa para el problema `{}`\n\
                            👤 Usuario: `{}`\n\n\
                            💡 **Posibles razones:**\n\
                            • El problema no ha sido resuelto aún\n\
                            • La solución no pasó todos los test cases\n\
                            • El problema no existe o el formato es incorrecto\n\n\
                            ¡Sigue intentando! 💪",
                            problem_id, user_handle
                        ),
                    )
                    .await;
            }
        }
        Err(e) => {
            // =====================
            //   API Error Response
            // =====================

            eprintln!("Codeforces API error in solved command: {}", e);
            let _ = msg
                .channel_id
                .say(
                    http,
                    "❌ Error al conectar con la API de Codeforces. Verifica que tu handle sea correcto e intenta más tarde.",
                )
                .await;
        }
    }

    Ok(())
}
