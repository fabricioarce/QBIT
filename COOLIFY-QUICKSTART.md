# Coolify Quick Start Guide

Quick reference for deploying the Discord bot to Coolify with external PostgreSQL.

## 🚀 Quick Deploy Steps

### 1. Create PostgreSQL Database in Coolify

If you don't have one already:

1. Go to **Resources** → **+ Add Resource** → **Database** → **PostgreSQL**
2. Set a name (e.g., `discord-bot-db`)
3. Set database name: `discord_bot`
4. Set username: `postgres` (or custom)
5. Set password: `<strong-password>`
6. Deploy and wait for it to be ready
7. **Note the internal service name** (e.g., `coolify-postgres-abc123`)

### 2. Deploy the Bot

1. Go to **Resources** → **+ Add Resource** → **Application**
2. Choose **Public Repository** or connect your private repo
3. Repository URL: `https://github.com/your-username/bot-olim-p-code`
4. Build Pack: **Dockerfile**
5. Click **Continue**

### 3. Configure Environment Variables

In your bot service settings, add these environment variables:

```
DISCORD_TOKEN=MTIzNDU2Nzg5MDEyMzQ1Njc4OQ.GaBcDe.FgHiJkLmNoPqRsTuVwXyZ
DATABASE_URL=postgresql://postgres:your_password@coolify-postgres-abc123:5432/discord_bot
RUST_LOG=info
```

**Important:** Replace:
- `MTIzNDU2...` with your actual Discord bot token
- `your_password` with your PostgreSQL password
- `coolify-postgres-abc123` with your actual PostgreSQL service name

### 4. Network Configuration

Both services should be on the `coolify` network by default. To verify:

1. Go to your bot service → **Advanced** → **Network**
2. Ensure `coolify` network is selected
3. Do the same for PostgreSQL service

### 5. Deploy

Click **Deploy** and monitor the logs. Look for:
```
✅ Conectado a la base de datos
Bot connected as YourBotName with the id of 123456789
```

## 🔍 Finding Your PostgreSQL Service Name

### Method 1: From Coolify UI
1. Go to your PostgreSQL service
2. Look at the URL or service name in Coolify
3. The internal hostname is usually shown in the service details

### Method 2: From Container Name
In your server terminal:
```bash
docker ps | grep postgres
```
The container name can be used as the hostname.

### Method 3: Network Inspection
```bash
docker network inspect coolify
```
Look for your PostgreSQL container and its aliases.

## 📝 Common DATABASE_URL Formats

```bash
# Same Coolify instance (recommended)
postgresql://postgres:password@coolify-postgres-abc123:5432/discord_bot

# Using container name
postgresql://postgres:password@postgres-container-name:5432/discord_bot

# Using IP address (less reliable)
postgresql://postgres:password@172.18.0.5:5432/discord_bot

# External host
postgresql://postgres:password@external.host.com:5432/discord_bot
```

## ⚠️ Common Issues & Solutions

### Issue: "Connection refused to PostgreSQL"

**Quick Fix:**
1. Verify PostgreSQL service is running in Coolify
2. Check both services are on same network (`coolify`)
3. Use the correct PostgreSQL service/container name

### Issue: "No se encontró DISCORD_TOKEN"

**Quick Fix:**
1. Go to service → **Environment** tab
2. Ensure `DISCORD_TOKEN` is added and saved
3. Redeploy the service

### Issue: "password authentication failed"

**Quick Fix:**
1. Check PostgreSQL password in Coolify
2. Update `DATABASE_URL` with correct password
3. Special characters in password? URL encode them:
   - `@` → `%40`
   - `#` → `%23`
   - `&` → `%26`

### Issue: Build fails

**Quick Fix:**
1. Check build logs in Coolify
2. Ensure `.sqlx` directory is committed to Git
3. Verify `Cargo.toml` edition is `2021` (not `2024`)

## 🔐 Security Checklist

- [ ] Strong PostgreSQL password set
- [ ] Discord bot token kept secure (never commit to Git)
- [ ] `.env` file in `.gitignore`
- [ ] Bot running as non-root user (already configured)
- [ ] Regular backups configured for PostgreSQL

## 📊 Monitoring

### View Logs
In Coolify: Service → **Logs** tab

### Check Resource Usage
In Coolify: Service → **Metrics** tab

### Restart Service
In Coolify: Service → Click **Restart** button

## 🔄 Updating the Bot

1. Push changes to your Git repository
2. In Coolify, click **Redeploy** on your bot service
3. Monitor the build logs
4. Verify bot reconnects successfully

## 📚 Getting PostgreSQL Connection Details

If you need to find your PostgreSQL details:

1. Go to your PostgreSQL service in Coolify
2. Click on **Environment** tab
3. You'll see:
   - `POSTGRES_USER`
   - `POSTGRES_PASSWORD`
   - `POSTGRES_DB`
4. The hostname is the service/container name (shown in the service title)

## 🧪 Testing Database Connection

From your server (SSH):

```bash
# Install PostgreSQL client
apt-get update && apt-get install -y postgresql-client

# Test connection (replace with your values)
psql "postgresql://postgres:password@coolify-postgres-abc123:5432/discord_bot"

# If successful, you'll see:
# psql (14.x)
# Type "help" for help.
# discord_bot=#
```

## 🎯 Quick Troubleshooting Commands

```bash
# List all running containers
docker ps

# Check bot logs
docker logs -f <bot-container-name>

# Check PostgreSQL logs
docker logs -f <postgres-container-name>

# List networks
docker network ls

# Inspect coolify network
docker network inspect coolify

# Restart bot container
docker restart <bot-container-name>
```

## ✅ Verification Checklist

After deployment, verify:

- [ ] PostgreSQL service is running (green status in Coolify)
- [ ] Bot service is running (green status in Coolify)
- [ ] Logs show "Conectado a la base de datos ✅"
- [ ] Logs show "Bot connected as..."
- [ ] Bot appears online in Discord
- [ ] Bot responds to commands

## 🆘 Still Having Issues?

1. **Check Logs First:** Most issues are visible in the logs
2. **Verify Environment Variables:** Ensure all are set correctly
3. **Network Connectivity:** Ensure both containers are on `coolify` network
4. **PostgreSQL Accessibility:** Test connection with `psql` command
5. **Discord Token:** Verify token is valid and bot is added to servers

## 📞 Support Resources

- Coolify Docs: https://coolify.io/docs
- Discord.js Guide: https://discord.js.org/
- PostgreSQL Docs: https://www.postgresql.org/docs/
- Full Guide: See `README.docker.md` in this repository

---

**Need more details?** Check the comprehensive `README.docker.md` file.