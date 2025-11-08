// =====================
//   Imports
// =====================
use crate::api;
use rand::seq::SliceRandom;
use serenity::http::Http;
use serenity::model::channel::Message;
use sqlx::PgPool;
use std::sync::Arc;

// =====================
//   Problem Command Execution
// =====================
// Fetches a random Codeforces problem within a specified difficulty range
pub async fn execute(http: &Arc<Http>, msg: &Message, db: &PgPool) -> Result<(), serenity::Error> {
    // Keep database reference (for future use)
    let _ = db;

    // =====================
    //   Parse Command Arguments
    // =====================
    // Extract difficulty range from message content
    let difficult = msg.content["!problem ".len()..].trim();

    // =====================
    //   Parse Min and Max Rating
    // =====================
    // Split the input to get min and max difficulty ratings
    if let Some((min_str, max_str)) = difficult.split_once(' ') {
        if let (Ok(min), Ok(max)) = (min_str.parse::<i32>(), max_str.parse::<i32>()) {
            // =====================
            //   Fetch Problems from API
            // =====================
            // Get problems from Codeforces API within the specified rating range
            let problems_result =
                api::codeforces::get_problems(Some(min as u32), Some(max as u32)).await;

            match problems_result {
                Ok(problems_data) => {
                    let problems = problems_data.result.problems;

                    // =====================
                    //   Check for Empty Results
                    // =====================
                    // If no problems found in the range, notify the user
                    if problems.is_empty() {
                        msg.channel_id
                            .say(http, "❌ No problems found in that difficulty range")
                            .await?;
                        return Ok(());
                    }

                    // =====================
                    //   Randomize Problem Selection
                    // =====================
                    let mut filtered_problems = problems;

                    // Shuffle in a separate scope to drop RNG before await
                    {
                        let mut rng = rand::rng();
                        filtered_problems.shuffle(&mut rng);
                    } // RNG dropped here

                    // =====================
                    //   Select Random Problem
                    // =====================
                    // Get the first problem from shuffled list
                    let problem = &filtered_problems[0];
                    let rating = problem.rating.unwrap_or(0);

                    // =====================
                    //   Create Difficulty Visual
                    // =====================
                    // Generate difficulty bars based on rating
                    let difficulty_bars = match rating {
                        r if r <= 1000 => "🟩".repeat(1),
                        r if r <= 1400 => "🟩".repeat(2),
                        r if r <= 1800 => "🟩".repeat(3),
                        r if r <= 2200 => "🟩".repeat(4),
                        _ => "🟩".repeat(5),
                    };

                    // =====================
                    //   Format Tags
                    // =====================
                    // Create formatted tag list or default message
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

                    // =====================
                    //   Generate Problem URL
                    // =====================
                    // Create direct link to the problem on Codeforces
                    let problem_url = format!(
                        "https://codeforces.com/problemset/problem/{}/{}",
                        problem.contest_id.unwrap_or(0),
                        problem.index
                    );

                    // =====================
                    //   Create Embed Message
                    // =====================
                    // Build rich embed with problem information
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

                    // =====================
                    //   Send Problem Message
                    // =====================
                    // Send the problem embed to the channel
                    let message = serenity::builder::CreateMessage::new()
                        .content(format!("Hello <@{}>! Here's your problem:", msg.author.id))
                        .embed(embed);

                    msg.channel_id.send_message(http, message).await?;
                }
                Err(_) => {
                    // =====================
                    //   Handle API Error
                    // =====================
                    println!("Error fetching problems");
                    msg.channel_id
                        .say(http, "❌ Error fetching problems from Codeforces")
                        .await?;
                }
            }

            Ok(())
        } else {
            // =====================
            //   Invalid Number Format
            // =====================
            // Send error message for invalid number format
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
        //   Missing Arguments
        // =====================
        // Send error message for missing arguments
        msg.channel_id
            .say(
                http,
                "❌ Incorrect format. Use: !problem MIN_RATING MAX_RATING",
            )
            .await?;
        Ok(())
    }
}
