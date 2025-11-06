use serenity::http::Http;
use std::sync::Arc;
use std::error::Error;
use crate::Bot;
use crate::api;
use chrono::{Local, Timelike};
use rand::seq::SliceRandom;
use serenity::model::id::ChannelId;
use sqlx::Row;

#[allow(deprecated)]
pub async fn start_daily_task(bot: Arc<Bot>, http: Arc<Http>) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("daily task started");

    loop {
        let now = Local::now();
        let actual_hour = now.hour() as i32;
        let actual_minute = now.minute() as i32;

        println!("Current time: {:02}:{:02}", actual_hour, actual_minute);

        let server_result = sqlx::query(
            "SELECT guild_id, daily_channel_id, daily_hour, daily_minute, min_rating, max_rating 
            FROM guild_config 
            WHERE daily_channel_id IS NOT NULL"
        )
        .fetch_all(&bot.db)
        .await;

        let servers = match server_result {
            Ok(s) => s,
            Err(_) => {
                println!("Error fetching servers from database");
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                continue;
            }
        };

        for server in servers {
            // Retrieve the scheduled time for daily messages and set default (9:00) if not set
            let scheduled_hour = server.try_get::<Option<i32>, _>("daily_hour").unwrap_or_default().unwrap_or(9);
            let scheduled_minute = server.try_get::<Option<i32>, _>("daily_minute").unwrap_or_default().unwrap_or(0);

            if actual_hour == scheduled_hour && actual_minute == scheduled_minute {
                // Get channel and guild ids
                let channel_id = match server.try_get::<Option<i64>, _>("daily_channel_id").unwrap_or_default() {
                    Some(id) => id,
                    _none => continue, // no channel configured
                };
                let guild_id = server.try_get::<i64, _>("guild_id").unwrap_or(0);

                // Get rating range from server config
                let min_rating = server.try_get::<Option<i32>, _>("min_rating").unwrap_or_default().unwrap_or(800);
                let max_rating = server.try_get::<Option<i32>, _>("max_rating").unwrap_or_default().unwrap_or(2000);

                // Fetch problems from Codeforces API with server's rating range
                let problems_result = api::codeforces::get_problems(Some(min_rating as u32), Some(max_rating as u32)).await;
                
                match problems_result {
                    Ok(problems_data) => {
                        let problems = problems_data.result.problems;

                        // Collect and filter rated problems
                        let mut filtered_problems: Vec<_> = problems.into_iter().filter(|p| p.rating.is_some()).collect();

                        if filtered_problems.is_empty() {
                            // No problem found in the desired range
                            let message = serenity::builder::CreateMessage::new()
                                .content("❌ No problem found in that difficulty range.");
                            if let Err(why) = ChannelId::new(channel_id as u64).send_message(&http, message).await {
                                println!("Error sending error message to guild {}: {:?}", guild_id, why);
                            }
                        } else {
                            // Randomly shuffle and get index in a separate scope
                            let problem_index = {
                                let mut rng = rand::thread_rng();
                                filtered_problems.shuffle(&mut rng);
                                0
                            };
                            // RNG is now dropped here, before any await
                            
                            let problem = &filtered_problems[problem_index];
                            
                            // Create difficulty bars based on rating
                            let rating = problem.rating.unwrap_or(0);
                            let difficulty_bars = match rating {
                                r if r <= 1000 => "█".repeat(1),
                                r if r <= 1400 => "█".repeat(2),
                                r if r <= 1800 => "█".repeat(3),
                                r if r <= 2200 => "█".repeat(4),
                                _ => "█".repeat(5),
                            };

                            // Format tags
                            let tags_text = if problem.tags.is_empty() {
                                "No tags".to_string()
                            } else {
                                problem.tags.iter()
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

                            // Create embed
                            let embed = serenity::builder::CreateEmbed::new()
                                .title("💻 Daily Codeforces Problem")
                                .description(format!("**{}**", problem.name))
                                .field("🎯 Difficulty", 
                                      format!("{} **{}**", difficulty_bars, rating), 
                                      true)
                                .field("🏷️ Tags", tags_text, false)
                                .field("🔗 Link", 
                                      format!("[Solve problem]({})", problem_url),
                                      false);

                            let message = serenity::builder::CreateMessage::new()
                                .content(format!("Hello <@&{}>! Here's your daily problem:", guild_id))
                                .embed(embed);

                            if let Err(why) = ChannelId::new(channel_id as u64).send_message(&http, message).await {
                                println!("Error sending daily message to guild {}: {:?}", guild_id, why);
                            }
                        }
                    }
                    Err(_) => {
                        println!("Error fetching problems from Codeforces API");
                    }
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}