// =====================
//   Imports
// =====================
use crate::Bot;
use crate::api;
use chrono::{Local, Timelike};
use rand::seq::SliceRandom;
use serenity::http::Http;
use serenity::model::id::ChannelId;
use sqlx::Row;
use std::error::Error;
use std::sync::Arc;

// =====================
//   Daily Task Function
// =====================
// Starts the daily task that posts Codeforces problems at scheduled times
#[allow(deprecated)]
pub async fn start_daily_task(
    bot: Arc<Bot>,
    http: Arc<Http>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // =====================
    //   Task Initialization
    // =====================
    println!("Daily task started");

    // =====================
    //   Main Task Loop
    // =====================
    // Infinite loop that checks every minute for scheduled tasks
    loop {
        // =====================
        //   Get Current Time
        // =====================
        let now = Local::now();
        let actual_hour = now.hour() as i32;
        let actual_minute = now.minute() as i32;

        println!("Current time: {:02}:{:02}", actual_hour, actual_minute);

        // =====================
        //   Fetch Server Configurations
        // =====================
        // Get all servers that have a daily channel configured
        let server_result = sqlx::query(
            "SELECT guild_id, daily_channel_id, daily_hour, daily_minute, min_rating, max_rating
            FROM guild_config
            WHERE daily_channel_id IS NOT NULL",
        )
        .fetch_all(&bot.db)
        .await;

        // =====================
        //   Handle Database Error
        // =====================
        let servers = match server_result {
            Ok(s) => s,
            Err(_) => {
                println!("Error fetching servers from database");
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                continue;
            }
        };

        // =====================
        //   Process Each Server
        // =====================
        for server in servers {
            // =====================
            //   Get Scheduled Time
            // =====================
            // Retrieve the scheduled time for daily messages and set default (9:00) if not set
            let scheduled_hour = server
                .try_get::<Option<i32>, _>("daily_hour")
                .unwrap_or_default()
                .unwrap_or(9);
            let scheduled_minute = server
                .try_get::<Option<i32>, _>("daily_minute")
                .unwrap_or_default()
                .unwrap_or(0);

            // =====================
            //   Check If It's Time to Post
            // =====================
            if actual_hour == scheduled_hour && actual_minute == scheduled_minute {
                // =====================
                //   Get Channel and Guild IDs
                // =====================
                // Get channel and guild ids from the database row
                let channel_id = match server
                    .try_get::<Option<i64>, _>("daily_channel_id")
                    .unwrap_or_default()
                {
                    Some(id) => id,
                    None => continue, // No channel configured, skip this server
                };
                let guild_id = server.try_get::<i64, _>("guild_id").unwrap_or(0);

                // =====================
                //   Get Rating Range
                // =====================
                // Get rating range from server config with defaults (800-1200)
                let min_rating = server
                    .try_get::<Option<i32>, _>("min_rating")
                    .unwrap_or_default()
                    .unwrap_or(800);
                let max_rating = server
                    .try_get::<Option<i32>, _>("max_rating")
                    .unwrap_or_default()
                    .unwrap_or(1200);

                // =====================
                //   Fetch Problems from API
                // =====================
                // Fetch problems from Codeforces API with server's rating range
                let problems_result =
                    api::codeforces::get_problems(Some(min_rating as u32), Some(max_rating as u32))
                        .await;

                match problems_result {
                    Ok(problems_data) => {
                        let problems = problems_data.result.problems;

                        // =====================
                        //   Filter Problems
                        // =====================
                        // Collect and filter problems that have a rating
                        let mut filtered_problems: Vec<_> = problems
                            .into_iter()
                            .filter(|p| p.rating.is_some())
                            .collect();

                        if filtered_problems.is_empty() {
                            // =====================
                            //   No Problems Found
                            // =====================
                            // Send error message if no problems found in the range
                            let message = serenity::builder::CreateMessage::new()
                                .content("❌ No problems found in that difficulty range.");
                            if let Err(why) = ChannelId::new(channel_id as u64)
                                .send_message(&http, message)
                                .await
                            {
                                println!(
                                    "Error sending error message to guild {}: {:?}",
                                    guild_id, why
                                );
                            }
                        } else {
                            // =====================
                            //   Select Random Problem
                            // =====================
                            // Randomly shuffle and get index in a separate scope
                            let problem_index = {
                                let mut rng = rand::thread_rng();
                                filtered_problems.shuffle(&mut rng);
                                0
                            };
                            // RNG is now dropped here, before any await

                            let problem = &filtered_problems[problem_index];

                            // =====================
                            //   Generate Difficulty Visual
                            // =====================
                            // Create difficulty bars based on rating
                            let rating = problem.rating.unwrap_or(0);
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
                                .title("💻 Daily Codeforces Problem")
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
                            //   Send Daily Problem
                            // =====================
                            // Create and send the daily problem message
                            let message = serenity::builder::CreateMessage::new()
                                .content("Hello everyone! Here's your daily problem:")
                                .embed(embed);

                            if let Err(why) = ChannelId::new(channel_id as u64)
                                .send_message(&http, message)
                                .await
                            {
                                println!(
                                    "Error sending daily message to guild {}: {:?}",
                                    guild_id, why
                                );
                            }
                        }
                    }
                    Err(_) => {
                        // =====================
                        //   API Error
                        // =====================
                        println!("Error fetching problems from Codeforces API");
                    }
                }
            }
        }

        // =====================
        //   Wait Until Next Minute
        // =====================
        // Sleep for 60 seconds before checking again
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
