// =====================
//   Imports
// =====================
use serenity::http::Http;
use serenity::model::channel::Message;
use std::sync::Arc;

// =====================
//   Ping Command
// =====================
// Simple ping command to test bot responsiveness and latency
pub async fn execute(http: &Arc<Http>, msg: &Message) -> Result<(), serenity::Error> {
    // =====================
    //   Send Response
    // =====================
    // Reply with a pong message to confirm bot is active
    msg.channel_id.say(http, "🏓 Pong!").await?;
    Ok(())
}
