// =====================
//   Imports
// =====================
use crate::Bot;
use crate::commands;
use crate::commands::codeforces::problem;
use crate::commands::codeforces::setchannel;
use crate::commands::codeforces::sethour;
use serenity::http::Http;
use serenity::model::channel::Message;
use std::sync::Arc;

// =====================
//   Message Handler
// =====================
// Main message handler that processes all incoming messages
pub async fn handle_message(bot: &Bot, http: &Arc<Http>, msg: Message) {
    // =====================
    //   Bot Check
    // =====================
    // Ignore messages from bots to prevent loops
    if msg.author.bot {
        return;
    }

    // =====================
    //   Ping Command
    // =====================
    // Responds to ping command to check bot status
    if msg.content == "!ping" {
        let _ = commands::ping::execute(http, &msg).await;
    }

    // =====================
    //   Problem Command
    // =====================
    // Fetches and displays a Codeforces problem
    if msg.content.starts_with("!problem ") {
        let _ = problem::execute(http, &msg, &bot.db).await;
    }

    // =====================
    //   Set Hour Command
    // =====================
    // Sets the hour for daily problem posting
    if msg.content.starts_with("!sethour ") {
        let _ = sethour::execute(http, &msg, &bot.db).await;
    }

    // =====================
    //   Set Channel Command
    // =====================
    // Sets the channel for daily problem posting
    if msg.content.starts_with("!setchannel ") {
        let _ = setchannel::execute(http, &msg, &bot.db).await;
    }
}
