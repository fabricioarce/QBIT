use serenity::client::Context;
use serenity::utils::Color;
use std::sync::Arc;
use crate::Bot;
use crate::api;
use chrono::{Local, Timelike};
use rand;

pub async fn start_daily_task(bot: Arc<Bot>, ctx: Context) {
    println!("daily task started");

    loop {
        let now = Local::now();
        let actual_hour = now.hour() as i32;
        let actual_minute = now.minute() as i32;

        println!("Current time: {:02}:{:02}", actual_hour, actual_minute);

        let server_result = sqlx::query!(
            "SELECT guild_id, daily_channel_id, daily_hour, daily_minute, min_rating, max_rating
            FROM guild_config
            WHERE daily_channel_id IS NOT NULL"
        )
        .fetch_all(&bot.db)
        .await;

        if let OK(servers) = server_result {
            for server in servers {
                // Retrieve the scheduled time for daily messages and set defaul (9:00) if not set
                let scheduled_hour = server.daily_hour.unwrap_or(9);
                let scheduled_minute = server.daily_minute.unwrap_or(0);

                if actual_hour == scheduled_hour && actual_minute == scheduled_minute {
                    let channel_id = server.daily_channel_id.unwrap();
                    let guild_id = server.guild_id;

                    // Get rating range from server config
                    let min_rating = server.min_rating.unwrap_or(800);
                    let max_rating = server.max_rating.unwrap_or(2000);

                    // Fetch problems from Codeforces API with server's rating range
                    match api::codeforces::get_problems(Some(min_rating), Some(max_rating)).await {
                        Ok(problems_data) => {
                            let problems = problems_data.result.problems;
                            let problem_count = problems.len();

                            // Randomly select one problem
                            if let Some(problem) = problems.into_iter().filter(|p| p.rating.is_some()).collect::<Vec<_>>().into_iter().nth(
                                rand::random::<usize>() % problem_count
                            ) {
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
                                    problem.contestId.unwrap_or(0),
                                    problem.index
                                );

                                if let Err(why) = ctx.http.send_message(channel_id as u64, |m| {
                                    m.content(format!("Hello <@&{}>! Here's your daily problem:", guild_id))
                                        .embed(|e| {
                                            e.title("💻 Daily Codeforces Problem")
                                                .color(0x3498db)
                                                .description(format!("**{}**", problem.name))
                                                .field("🎯 Difficulty", 
                                                      format!("{} **{}**", difficulty_bars, rating), 
                                                      true)
                                                .field("🏷️ Tags", tags_text, false)
                                                .field("🔗 Link", 
                                                      format!("[Solve problem]({})", problem_url),
                                                      false)
                                        })
                                }).await {
                                    println!("Error sending daily message to guild {}: {:?}", guild_id, why);
                                }
                            } else {
                                if let Err(why) = ctx.http.send_message(channel_id as u64, |m| {
                                    m.content("❌ No problem found in that difficulty range.")
                                }).await {
                                    println!("Error sending error message to guild {}: {:?}", guild_id, why);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error fetching problems from Codeforces API: {:?}", e);
                        }
                    }
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}