use serenity::http::Http;
use serenity::model::channel::Message;
use std::sync::Arc;
use sqlx::PgPool;

pub async fn execute(http: &Arc<Http>, msg: &Message, db: &PgPool) -> Result<(), serenity::Error> {
    // Extraer la hora después del comando
    let hora = msg.content["!sethora ".len()..].trim();

    if let Some((horas_str, minutos_str)) = hora.split_once(':') {
        if let (Ok(horas), Ok(minutos)) = (horas_str.parse::<i32>(), minutos_str.parse::<i32>()) {
            if horas >= 0 && horas < 24 && minutos >= 0 && minutos < 60 {
                let guild_id = msg.guild_id.unwrap().get() as i64;

                let result = sqlx::query(
                    "UPDATE guild_config 
                    SET daily_hour = $1, daily_minute = $2 
                    WHERE guild_id = $3"
                )
                .bind(horas)
                .bind(minutos)
                .bind(guild_id)
                .execute(db)
                .await;

                match result {
                    Ok(_) => {
                        let response = format!(
                            "⏰ Hora del reporte diario configurada a las {:02}:{:02}",
                            horas, minutos
                        );
                        msg.channel_id.say(http, response).await?;
                    }
                    Err(_) => {
                        println!("Error guardando hora");
                        msg.channel_id.say(http, "❌ Error al guardar la hora").await?;
                    }
                }
            } else {
                msg.channel_id.say(http, "❌ Hora inválida. Usa formato HH:MM (00:00 - 23:59)").await?;
            }
        } else {
            msg.channel_id.say(http, "❌ Formato incorrecto. Usa: !sethora HH:MM (ejemplo: !sethora 09:00)").await?;
        }
    } else {
        msg.channel_id.say(http, "❌ Formato incorrecto. Usa: !sethora HH:MM (ejemplo: !sethora 09:00)").await?;
    }

    Ok(())
}