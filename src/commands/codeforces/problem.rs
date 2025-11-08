// =====================
//   Imports and Dependencies
// =====================

use crate::api;
use rand::seq::SliceRandom;
use serenity::http::Http;
use serenity::model::channel::Message;
use sqlx::PgPool;
use std::sync::Arc;

// =====================
//   Problem Command Implementation
// =====================

// Main function to handle the !problem command
// Fetches a random Codeforces problem within specified difficulty range
pub async fn execute(http: &Arc<Http>, msg: &Message, db: &PgPool) -> Result<(), serenity::Error> {
    // =====================
    //   Command Input Parsing
    // =====================

    // Suppress unused database parameter warning
    let _ = db;

    // Extract difficulty range from command (format: !problem MIN_RATING MAX_RATING)
    let difficult = msg.content["!problem ".len()..].trim();

    // Parse min and max rating from user input
    if let Some((min_str, max_str)) = difficult.split_once(' ') {
        if let (Ok(min), Ok(max)) = (min_str.parse::<i32>(), max_str.parse::<i32>()) {
            // =====================
            //   Codeforces API Call
            // =====================

            // Fetch problems from Codeforces API with specified rating range
            let problems_result =
                api::codeforces::get_problems(Some(min as u32), Some(max as u32)).await;

            match problems_result {
                Ok(problems_data) => {
                    // =====================
                    //   Problem Selection and Validation
                    // =====================

                    let problems = problems_data.result.problems;

                    // Check if any problems were found in the specified range
                    if problems.is_empty() {
                        msg.channel_id
                            .say(http, "❌ No problems found in that difficulty range")
                            .await?;
                        return Ok(());
                    }

                    // Prepare problems list for random selection
                    let mut filtered_problems = problems;

                    // Shuffle problems randomly in a separate scope to drop RNG before await
                    {
                        let mut rng = rand::rng();
                        filtered_problems.shuffle(&mut rng);
                    } // RNG dropped here to avoid holding across await points

                    let problem = &filtered_problems[0];
                    let rating = problem.rating.unwrap_or(0);

                    // =====================
                    //   Difficulty Visualization
                    // =====================

                    // Create difficulty bars based on problem rating
                    let difficulty_bars = match rating {
                        r if r <= 1000 => "🟩".repeat(1), // Beginner
                        r if r <= 1400 => "🟩".repeat(2), // Easy
                        r if r <= 1800 => "🟩".repeat(3), // Medium
                        r if r <= 2200 => "🟩".repeat(4), // Hard
                        _ => "🟩".repeat(5),              // Expert
                    };

                    // =====================
                    //   Problem Information Formatting
                    // =====================

                    // Format problem tags for display
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

                    // Generate Codeforces problem URL
                    let problem_url = format!(
                        "https://codeforces.com/problemset/problem/{}/{}",
                        problem.contest_id.unwrap_or(0),
                        problem.index
                    );

                    // =====================
                    //   Discord Embed Creation
                    // =====================

                    // Create rich embed with problem information
                    let embed = serenity::builder::CreateEmbed::new()
                        .title("💻 Codeforces Problem")
                        .description(format!("**{}**", problem.name))
                        .field(
                            "🎯 Difficulty",
                            format!("{} **{}**", difficulty_bars, rating),
                            true,
                        )
                        .field("🏷️ Tags", tags_text, false)
                        .field(
                            "🔗 Link",
                            format!("[Solve problem]({})", problem_url),
                            false,
                        );

                    // Create message with embed and user mention
                    let message = serenity::builder::CreateMessage::new()
                        .content(format!("Hello <@{}>! Here's your problem:", msg.author.id))
                        .embed(embed);

                    // Send the problem message to Discord
                    msg.channel_id.send_message(http, message).await?;
                }
                Err(_) => {
                    // =====================
                    //   API Error Handling
                    // =====================

                    println!("Error obtaining problems");
                    msg.channel_id
                        .say(http, "❌ Error fetching problems from Codeforces")
                        .await?;
                }
            }

            Ok(())
        } else {
            // =====================
            //   Invalid Rating Format Error
            // =====================

            msg.channel_id
                .say(
                    http,
                    "❌ Incorrect format. Use: !problem MIN_RATING MAX_RATING",
                )
                .await?;
            Ok(())
        }
    } else {
        // =====================
        //   Missing Parameters Error
        // =====================

        msg.channel_id
            .say(
                http,
                "❌ Incorrect format. Use: !problem MIN_RATING MAX_RATING",
            )
            .await?;
        Ok(())
    }
}
