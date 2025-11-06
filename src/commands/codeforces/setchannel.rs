use serenity::client::Context;
use serenity::model::channel::Message;
use sqlx::PgPool;

pub async fn execute(ctx: &Context, msg: &Message, db: &PgPool) -> Result<(), serenity::Error> {
    // Extraer la mención del canal
    let mention = msg.content["!setchannel ".len()..].trim();

    // Validar formato de mención: <#ID>
    if let Some(id_str) = mention.strip_prefix("<#").and_then(|s| s.strip_suffix(">")) {
        if let Ok(channel_id) = id_str.parse::<i64>() {
            let guild_id: i64 = msg.guild_id.unwrap().get() as i64;

            // Guardar en base de datos
            let result = sqlx::query(
                "UPDATE guild_config 
                 SET daily_channel_id = $1
                 WHERE guild_id = $2"
            )
            .bind(channel_id)
            .bind(guild_id)
            .execute(db)
            .await;

            match result {
                Ok(_) => {
                    let response = format!("✅ Canal del reporte diario configurado a <#{}>", channel_id);
                    msg.channel_id.say(&ctx.http, response).await?;
                }
                Err(e) => {
                    println!("Error guardando canal: {:?}", e);
                    msg.channel_id.say(&ctx.http, "❌ Error al guardar el canal").await?;
                }
            }
        } else {
            msg.channel_id.say(&ctx.http, "❌ ID de canal inválido").await?;
        }
    } else {
        msg.channel_id.say(&ctx.http, "❌ Formato incorrecto. Usa: !setchannel #canal").await?;
    }

    Ok(())
}
