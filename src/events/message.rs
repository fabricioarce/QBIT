// =====================
//   Imports and Dependencies
// =====================

use crate::commands;
use crate::commands::codeforces::account;
use crate::commands::codeforces::problem;
use crate::commands::codeforces::setchannel;
use crate::commands::codeforces::sethour;
use crate::commands::codeforces::solved;
use crate::Bot;
use serenity::http::Http;
use serenity::model::channel::Message;
use std::sync::Arc;

// =====================
//   Message Event Handler
// =====================

// Main message handler function - processes all incoming messages and dispatches commands
pub async fn handle_message(bot: &Bot, http: &Arc<Http>, msg: Message) {
    // =====================
    //   Bot Message Filter
    // =====================

    // Ignore messages from other bots to prevent infinite loops
    if msg.author.bot {
        return;
    }

    // =====================
    //   Command Dispatching
    // =====================

    // Handle ping command - simple connectivity test
    if msg.content == "!ping" {
        let _ = commands::ping::execute(http, &msg).await;
    }

    // Handle problem command - get random Codeforces problem with specified difficulty range
    if msg.content.starts_with("!problem ") {
        let _ = problem::execute(http, &msg, &bot.db).await;
    }

    // Handle set hour command - configure daily problem posting time
    if msg.content.starts_with("!sethora ") {
        let _ = sethour::execute(http, &msg, &bot.db).await;
    }

    // Handle set channel command - configure which channel receives daily problems
    if msg.content.starts_with("!setchannel ") {
        let _ = setchannel::execute(http, &msg, &bot.db).await;
    }

    // Handle account command - link Codeforces account to Discord user
    if msg.content.starts_with("!account ") {
        if let Some(guild_id) = msg.guild_id {
            if let Ok(guild) = http.get_guild(guild_id).await {
                let _ = account::execute(http, &msg, &bot.db, &guild).await;
            }
        }
    }

    // Handle solved command - verify if a problem is solved and mark it
    if msg.content.starts_with("!solved ") {
        if let Some(guild_id) = msg.guild_id {
            if let Ok(guild) = http.get_guild(guild_id).await {
                let _ = solved::execute(http, &msg, &bot.db, &guild).await;
            }
        }
    }
}
