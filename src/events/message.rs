use serenity::client::Context;
use serenity::model::channel::Message;
use crate::Bot;
use crate::commands::codeforces::sethour;
use crate::commands::codeforces::setchannel;

pub async fn handle_message(bot: &Bot, ctx: Context, msg: Message) {

    if msg.author.bot {
        return;
    }

    if msg.content.starts_with("!sethora ") {
        let _ = sethour::execute(&ctx, &msg, &bot.db).await;
    }

    if msg.content.starts_with("!setchannel ") {
        let _ = setchannel::execute(&ctx, &msg, &bot.db).await;
    }
}
