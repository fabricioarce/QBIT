use serenity::http::Http;
use serenity::model::channel::Message;
use std::sync::Arc;
use crate::Bot;
use crate::commands::codeforces::sethour;
use crate::commands::codeforces::setchannel;
use crate::commands::codeforces::problem;
use crate::commands;

pub async fn handle_message(bot: &Bot, http: &Arc<Http>, msg: Message) {

    if msg.author.bot {
        return;
    }

    if msg.content == "!ping" {
        let _ = commands::ping::execute(http, &msg).await;
    }

    if msg.content.starts_with("!problem ") {
        let _ = problem::execute(http, &msg, &bot.db).await;
    }

    if msg.content.starts_with("!sethora ") {
        let _ = sethour::execute(http, &msg, &bot.db).await;
    }

    if msg.content.starts_with("!setchannel ") {
        let _ = setchannel::execute(http, &msg, &bot.db).await;
    }
}