use serenity::model::guild::Guild;
use crate::Bot;

pub async fn handle_guild_create(
    bot: &Bot, 
    ctx_http: &serenity::http::Http, 
    guild: Guild, 
    is_new: Option<bool>
) {
    let _ = ctx_http;
    println!("Guild created: {} (ID: {})", guild.name, guild.id);

    // Lógica de la base de datos igual
    let _ = sqlx::query(
        "INSERT INTO guild_config (guild_id, guild_name)
        VALUES ($1, $2)
        ON CONFLICT (guild_id) DO NOTHING"
    )
    .bind(guild.id.get() as i64)
    .bind(&guild.name)
    .execute(&bot.db)
    .await
    .expect("Error insertando guild en la base de datos");

    println!("Guild entry asegurada en la base de datos ✅");

    // Ejemplo de uso de is_new
    if let Some(true) = is_new {
        println!("Este guild es nuevo para el bot!");
    }
}
