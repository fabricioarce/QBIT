// =====================
//   Imports and Dependencies
// =====================

use serenity::http::Http;
use serenity::model::channel::Message;
use std::sync::Arc;

// =====================
//   Ping Command Implementation
// =====================

// Simple ping command to test bot connectivity and response time
// Responds with a "Pong!" message when user sends "!ping"
pub async fn execute(http: &Arc<Http>, msg: &Message) -> Result<(), serenity::Error> {
    // =====================
    //   Command Response
    // =====================

    // Send pong response to the same channel where ping was received
    msg.channel_id.say(http, "🏓 Pong!").await?;
    Ok(())
}
