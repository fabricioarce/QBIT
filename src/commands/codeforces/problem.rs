use serenity::http::Http;
use serenity::model::channel::Message;
use sqlx::PgPool;
use std::sync::Arc;
use crate::api;
use rand::seq::SliceRandom;

pub async fn execute(http: &Arc<Http>, msg: &Message, db: &PgPool) -> Result<(), serenity::Error> {
    let _ = db;
    let difficult = msg.content["!problem ".len()..].trim();

    if let Some((min_str, max_str)) = difficult.split_once(' ') {
        if let (Ok(min), Ok(max)) = (min_str.parse::<i32>(), max_str.parse::<i32>()) {
            let problems_result =
                api::codeforces::get_problems(Some(min as u32), Some(max as u32)).await;

            match problems_result {
                Ok(problems_data) => {
                    let problems = problems_data.result.problems;

                    if problems.is_empty() {
                        msg.channel_id
                            .say(http, "❌ No se encontraron problemas en ese rango de dificultad")
                            .await?;
                        return Ok(());
                    }

                    let mut filtered_problems = problems;
                    
                    // Shuffle in a separate scope to drop RNG before await
                    {
                        let mut rng = rand::rng();
                        filtered_problems.shuffle(&mut rng);
                    } // RNG dropped here

                    let problem = &filtered_problems[0];
                    let rating = problem.rating.unwrap_or(0);

                    let difficulty_bars = match rating {
                        r if r <= 1000 => "🟩".repeat(1),
                        r if r <= 1400 => "🟩".repeat(2),
                        r if r <= 1800 => "🟩".repeat(3),
                        r if r <= 2200 => "🟩".repeat(4),
                        _ => "🟩".repeat(5),
                    };

                    let tags_text = if problem.tags.is_empty() {
                        "No tags".to_string()
                    } else {
                        problem
                            .tags
                            .iter()
                            .map(|tag| format!("`{}`", tag))
                            .collect::<Vec<_>>()
                            .join(", ")
                    };

                    let problem_url = format!(
                        "https://codeforces.com/problemset/problem/{}/{}",
                        problem.contest_id.unwrap_or(0),
                        problem.index
                    );

                    let embed = serenity::builder::CreateEmbed::new()
                        .title("💻 Codeforces Problem")
                        .description(format!("**{}**", problem.name))
                        .field("🎯 Difficulty", format!("{} **{}**", difficulty_bars, rating), true)
                        .field("🏷️ Tags", tags_text, false)
                        .field("🔗 Link", format!("[Solve problem]({})", problem_url), false);

                    let message = serenity::builder::CreateMessage::new()
                        .content(format!("Hello <@{}>! Here's your problem:", msg.author.id))
                        .embed(embed);

                    msg.channel_id.send_message(http, message).await?;
                }
                Err(_) => {
                    println!("Error obteniendo problemas");
                    msg.channel_id
                        .say(http, "❌ Error al obtener problemas de Codeforces")
                        .await?;
                }
            }

            Ok(())
        } else {
            msg.channel_id
                .say(http, "❌ Formato incorrecto. Usa: !problem MIN_RATING MAX_RATING")
                .await?;
            Ok(())
        }
    } else {
        msg.channel_id
            .say(http, "❌ Formato incorrecto. Usa: !problem MIN_RATING MAX_RATING")
            .await?;
        Ok(())
    }
}