use serenity::http::Http;
use serenity::model::channel::Message;
use std::sync::Arc;

pub async fn execute(http: &Arc<Http>, msg: &Message) -> Result<(), serenity::Error> {
    msg.channel_id.say(http, "🏓 Pong!").await?;
    Ok(())
}