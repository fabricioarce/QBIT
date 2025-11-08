# Deployment Checklist

Use this checklist to ensure successful deployment to Coolify.

## Pre-Deployment

### PostgreSQL Setup
- [ ] PostgreSQL service is created in Coolify
- [ ] Database name is set (e.g., `discord_bot`)
- [ ] Username and password are configured
- [ ] PostgreSQL service is running (green status)
- [ ] Note the PostgreSQL service name (e.g., `coolify-postgres-abc123`)

### Discord Bot Configuration
- [ ] Discord bot is created in [Discord Developer Portal](https://discord.com/developers/applications)
- [ ] Bot token is copied and saved securely
- [ ] Required intents are enabled:
  - [ ] Server Members Intent
  - [ ] Message Content Intent
  - [ ] Presence Intent (optional)
- [ ] Bot is invited to at least one test server

### Repository Setup
- [ ] All code is committed to Git
- [ ] `.sqlx` directory is included in repository
- [ ] `.env` file is NOT committed (in `.gitignore`)
- [ ] `Cargo.toml` edition is set to `2021`
- [ ] Repository is accessible by Coolify

## Deployment to Coolify

### Step 1: Create Application
- [ ] New application created in Coolify
- [ ] Repository connected (GitHub/GitLab/etc.)
- [ ] Build Pack set to **Dockerfile**
- [ ] Dockerfile path is `dockerfile` or `Dockerfile`

### Step 2: Configure Environment Variables
- [ ] `DISCORD_TOKEN` - Set your Discord bot token
- [ ] `DATABASE_URL` - Format: `postgresql://user:pass@host:5432/db`
- [ ] `RUST_LOG` - Set to `info` (or `debug` for troubleshooting)
- [ ] All variables are saved in Coolify

### Step 3: Network Configuration
- [ ] Bot service is on `coolify` network
- [ ] PostgreSQL service is on `coolify` network
- [ ] Both services can communicate (same network)

### Step 4: Build & Deploy
- [ ] Click **Deploy** in Coolify
- [ ] Monitor build logs for errors
- [ ] Wait for build to complete (may take 5-10 minutes first time)

## Post-Deployment Verification

### Check Logs
- [ ] No build errors in Coolify logs
- [ ] Log shows: `Conectado a la base de datos ✅`
- [ ] Log shows: `Bot connected as [BotName] with the id of [ID]`
- [ ] No error messages in logs

### Test Bot Functionality
- [ ] Bot appears online in Discord
- [ ] Bot responds to commands
- [ ] Bot can write to channels (if applicable)
- [ ] Database operations work (guild config, etc.)

### Security Check
- [ ] Bot token is not exposed in logs
- [ ] `.env` file is not in repository
- [ ] PostgreSQL password is strong
- [ ] Bot runs as non-root user (default in our Dockerfile)

## Troubleshooting Reference

### Common Issues

**"Connection refused" to PostgreSQL**
```
DATABASE_URL=postgresql://user:pass@CORRECT-SERVICE-NAME:5432/db
```
- Verify PostgreSQL service name
- Check both services are on same network
- Ensure PostgreSQL is running

**"No se encontró DISCORD_TOKEN"**
- Go to Environment Variables in Coolify
- Add `DISCORD_TOKEN` and save
- Redeploy the service

**Build fails**
- Check edition in `Cargo.toml` is `2021`
- Ensure `.sqlx` directory exists
- Check Rust syntax errors

**Bot offline in Discord**
- Verify Discord token is valid
- Check required intents are enabled
- Ensure bot is invited to server

## Rollback Plan

If deployment fails:
- [ ] Click **Rollback** in Coolify (if available)
- [ ] Check previous working commit
- [ ] Review recent changes
- [ ] Test locally with Docker before redeploying

## Maintenance

### Regular Tasks
- [ ] Monitor logs weekly
- [ ] Check resource usage (CPU/Memory)
- [ ] Backup PostgreSQL database regularly
- [ ] Update dependencies monthly
- [ ] Review and rotate secrets quarterly

### Update Procedure
1. Test changes locally with Docker
2. Commit and push to repository
3. Click **Redeploy** in Coolify
4. Monitor deployment logs
5. Verify bot functionality
6. Rollback if issues occur

## Quick Commands

```bash
# View bot logs (SSH to server)
docker logs -f <bot-container-name>

# View PostgreSQL logs
docker logs -f <postgres-container-name>

# Restart bot
docker restart <bot-container-name>

# Check network connectivity
docker network inspect coolify

# Test database connection
docker exec -it <bot-container-name> sh
# (then try connecting with psql if available)
```

## Environment Variables Template

Copy this template for your `.env` or Coolify settings:

```env
# Discord Bot Token
DISCORD_TOKEN=your_discord_bot_token_here

# Database URL (adjust service name and credentials)
DATABASE_URL=postgresql://postgres:your_password@coolify-postgres-abc123:5432/discord_bot

# Logging (optional)
RUST_LOG=info
RUST_BACKTRACE=0
```

## Support

- **Full Documentation**: See `README.docker.md`
- **Quick Start**: See `COOLIFY-QUICKSTART.md`
- **Coolify Docs**: https://coolify.io/docs
- **Discord API**: https://discord.com/developers/docs

---

**Last Updated**: 2024-11-07
**Version**: 1.0