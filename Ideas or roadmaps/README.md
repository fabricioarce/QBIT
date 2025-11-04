# MiBot Discord - Comunidad de Programación Competitiva

Un bot de Discord escrito en **Rust**, diseñado para comunidades de programación competitiva (por ejemplo Codeforces).
Incluye **sistema de gamificación**, economía, roles automáticos, moderación y sincronización con Codeforces, inspirado en bots populares como MEE6.

---

## ⚡ Características principales

### 1. Gamificación

* Los usuarios ganan **XP** al participar en el servidor.
* Suben **niveles** según su XP acumulado.
* Se asignan **roles automáticos** según nivel o actividad.

### 2. Economía

* Los usuarios pueden acumular **monedas virtuales**.
* Existe una **tienda** para gastar monedas.
* Administradores pueden otorgar monedas manualmente.

### 3. Moderación

* Se pueden dar **warns**, **kick** o **ban** a usuarios.
* Todos los eventos de moderación se registran en la base de datos.
* Incluye **filtro de palabras** y **anti-spam**.

### 4. Integración Codeforces

* Sincronización automática de perfiles de Codeforces.
* Rankings y estadísticas de usuarios.
* Problema diario enviado a los servidores.

### 5. Panel web (opcional)

* Permite a los administradores modificar la configuración del bot.
* Configuración de roles automáticos, XP y economía desde la web.

---

## 📂 Estructura del proyecto

```
mi_bot_discord/
├── Cargo.toml            # Configuración y dependencias de Rust
├── .env                  # Variables de entorno (tokens y DB URL)
├── src/
│   ├── main.rs           # Punto de entrada del bot
│   ├── lib.rs            # Exporta módulos públicos
│   ├── bot/              # Lógica del bot y eventos
│   │   ├── mod.rs
│   │   ├── handler.rs    # EventHandler principal
│   │   └── context.rs    # BotContext compartido (DB, APIs)
│   ├── commands/         # Comandos del bot
│   │   ├── codeforces/   # Comandos de Codeforces
│   │   ├── economy/      # Comandos de economía
│   │   ├── moderation/   # Comandos de moderación
│   │   └── utility/      # Comandos generales (!ping, !help)
│   ├── services/         # Integraciones externas y scheduler
│   ├── database/         # Conexión, modelos, queries y migraciones
│   ├── tasks/            # Scheduler y tareas periódicas
│   └── utils/            # Funciones de ayuda (embeds, permisos, errores)
├── migrations/           # Scripts SQL de la base de datos
└── config/               # Configuración opcional del bot
```

💡 **Nota:** Esta estructura modular permite que tu bot sea escalable y fácil de mantener.

---

## 🗄️ Base de datos

El bot utiliza **PostgreSQL** para almacenar información de **todos los servidores y usuarios**.
No se crea una base de datos por servidor; en su lugar, se guardan filas separadas por **`guild_id`**.

### Tablas principales

1. **guilds** – Configuración de cada servidor.

```text
guild_id (BIGINT, PK)
name (TEXT)
config (JSONB) -> prefijo, canales, roles automáticos, etc.
created_at (TIMESTAMP)
```

2. **user_data** – Información de cada usuario por servidor.

```text
guild_id + user_id (PK)
xp, level, coins, warns
codeforces_handle
data (JSONB) -> logros, streaks
last_active (TIMESTAMP)
```

3. **economy_transactions** – Historial de monedas.

```text
id (PK)
guild_id, user_id
amount, reason
created_at
```

4. **moderation_logs** – Historial de moderación.

```text
id (PK)
guild_id, user_id
moderator_id, action, reason
created_at
```

5. **auto_roles** – Roles automáticos según nivel o condición.

```text
id (PK)
guild_id
role_id
condition (JSONB) -> ejemplo: {"type": "level", "value": 10}
```

6. **codeforces_profiles** – Información de perfiles de Codeforces.

```text
handle (PK)
discord_id
guild_id
rating, max_rating, rank
last_update
```

### Relaciones entre tablas

```
guilds
 ├── user_data
 │     ├── economy_transactions
 │     └── codeforces_profiles
 ├── moderation_logs
 └── auto_roles
```

* `guild_id` conecta todas las tablas con el servidor correspondiente.
* `ON DELETE CASCADE` asegura que al eliminar un servidor se borren todos los datos relacionados automáticamente.

---

## 🎯 Cómo funciona el bot

1. **Conexión**:

   * Se conecta a Discord y PostgreSQL usando `BotContext`.

2. **Eventos** (`EventHandler`):

   * `GUILD_CREATE` → Inserta el guild en la base de datos si no existe.
   * `GUILD_DELETE` → Elimina el guild y todos sus datos o lo marca como inactivo.
   * Otros eventos: mensajes, reacciones, roles, moderación, etc.

3. **Comandos**:

   * Cada comando usa `BotContext` para leer/escribir en la DB según el `guild_id` y `user_id`.
   * Comandos organizados por categoría (Codeforces, economía, moderación, utilidad).

4. **Tareas programadas**:

   * Scheduler envía problemas diarios y actualiza rankings de Codeforces automáticamente.

---

## 🏗️ Instalación

1. Clonar el repositorio:

```bash
git clone https://github.com/usuario/mi_bot_discord.git
cd mi_bot_discord
```

2. Crear archivo `.env`:

```
DISCORD_TOKEN=tu_token
DATABASE_URL=postgres://user:password@localhost:5432/tu_db
```

3. Ejecutar migraciones:

```bash
sqlx migrate run
```

4. Ejecutar el bot:

```bash
cargo run
```

---

## 🛠️ Contribuciones

* Puedes agregar nuevos comandos o servicios.
* Mejorar integración con APIs externas.
* Optimizar la base de datos y caché.
* Mejorar moderación, economía y gamificación.

---

## 📌 Notas finales

* La base de datos está diseñada para **escalar**: un solo esquema soporta miles de servidores y usuarios.
* Todas las configuraciones importantes se guardan en **JSONB** para flexibilidad.
* `EventHandler` es el **centro de recepción de eventos de Discord**, donde el bot reacciona a todo lo que ocurre en los servidores.

---

Si quieres, puedo hacer **una versión con diagramas ASCII incluidos** para que GitHub muestre claramente cómo **las tablas y módulos de Rust interactúan**, quedando mucho más profesional.

¿Quieres que haga esa versión visual?
