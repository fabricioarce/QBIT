use serenity::client::Context;
use std::sync::Arc;
use crate::Bot;
use crate::api;
use chrono::{Local, Timelike};

pub async fn start_daily_task(bot: Arc<Bot>, ctx: Context) {
    println!("daily task started");

    loop {
        let now = Local::now();
        let actual_hour = now.hour() as i32;
        let actual_minute = now.minute() as i32;

        println!("Current time: {:02}:{:02}", actual_hour, actual_minute);

        let server_result = sqlx::query!(
            "SELECT guild_id, daily_channel_id, daily_hour, daily_minute
            FROM guild_config
            WHERE daily_channel_id IS NOT NULL"
        )
        .fetch_all(&bot.db)
        .await;
    
    }