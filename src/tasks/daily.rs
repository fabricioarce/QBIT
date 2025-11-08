// =====================
//   Imports and Dependencies
// =====================

use crate::api;
use crate::Bot;
use chrono::{Local, Timelike};
use rand::seq::SliceRandom;
use serenity::http::Http;
use serenity::model::id::ChannelId;
use sqlx::Row;
use std::error::Error;
use std::sync::Arc;

// =====================
//   Daily Task Implementation
// =====================

// Main daily task function - runs continuously to check for scheduled problem postings
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

    loop {
        // =====================
        //   Time Management
        // =====================

        // Get current local time for scheduling comparison
        let now = Local::now();
        let actual_hour = now.hour() as i32;
        let actual_minute = now.minute() as i32;

        println!("Current time: {:02}:{:02}", actual_hour, actual_minute);

        // =====================
        //   Database Query - Guild Configurations
        // =====================

        // Fetch all guild configurations that have a daily channel configured
        let server_result = sqlx::query(
            "SELECT guild_id, daily_channel_id, daily_hour, daily_minute, min_rating, max_rating
            FROM guild_config
            WHERE daily_channel_id IS NOT NULL",
        )
        .fetch_all(&bot.db)
        .await;

        // Handle database query results
        let servers = match server_result {
            Ok(s) => s,
            Err(_) => {
                println!("Error fetching servers from database");
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                continue;
            }
        };

        // =====================
        //   Guild Processing Loop
        // =====================

        // Process each guild configuration for scheduled posting
        for server in servers {
            // =====================
            //   Schedule Verification
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

            // Check if current time matches scheduled time
            if actual_hour == scheduled_hour && actual_minute == scheduled_minute {
                // =====================
                //   Guild Configuration Extraction
                // =====================

                // Get channel and guild IDs
                let channel_id = match server.try_get::<i64, _>("daily_channel_id") {
                    Ok(id) => id,
                    Err(_) => continue, // error getting channel_id
                };
                let guild_id = server.try_get::<i64, _>("guild_id").unwrap_or(0);

                // Get rating range from server config with defaults
                let min_rating = server
                    .try_get::<Option<i32>, _>("min_rating")
                    .unwrap_or_default()
                    .unwrap_or(800);
                let max_rating = server
                    .try_get::<Option<i32>, _>("max_rating")
                    .unwrap_or_default()
                    .unwrap_or(1200);

                // =====================
                //   Codeforces API Integration
                // =====================

                // Fetch problems from Codeforces API with server's rating range
                let problems_result =
                    api::codeforces::get_problems(Some(min_rating as u32), Some(max_rating as u32))
                        .await;

                match problems_result {
                    Ok(problems_data) => {
                        // =====================
                        //   Problem Processing and Filtering
                        // =====================

                        let problems = problems_data.result.problems;

                        // Collect and filter rated problems
                        let mut filtered_problems: Vec<_> = problems
                            .into_iter()
                            .filter(|p| p.rating.is_some())
                            .collect();

                        if filtered_problems.is_empty() {
                            // =====================
                            //   No Problems Found - Error Message
                            // =====================

                            // No problem found in the desired range
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
                            //   Random Problem Selection
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
                            //   Difficulty Visualization
                            // =====================

                            // Create difficulty bars based on rating
                            let rating = problem.rating.unwrap_or(0);
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

                            // Format tags for display
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

                            // Generate problem URL
                            let problem_url = format!(
                                "https://codeforces.com/problemset/problem/{}/{}",
                                problem.contest_id.unwrap_or(0),
                                problem.index
                            );

                            // =====================
                            //   Discord Message Creation
                            // =====================

                            // Create rich embed with problem information
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

                            // Create message with guild mention and embed
                            let message = serenity::builder::CreateMessage::new()
                                .content(format!(
                                    "Hello <@&{}>! Here's your daily problem:",
                                    guild_id
                                ))
                                .embed(embed);

                            // =====================
                            //   Message Delivery
                            // =====================

                            // Send daily problem message to configured channel
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
                        //   API Error Handling
                        // =====================

                        println!("Error fetching problems from Codeforces API");
                    }
                }
            }
        }

        // =====================
        //   Task Sleep Interval
        // =====================

        // Wait 60 seconds before next check to prevent excessive API calls
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
