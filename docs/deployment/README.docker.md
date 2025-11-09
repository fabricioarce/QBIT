# Docker & Coolify Deployment Guide

This guide explains how to deploy the Discord bot using Docker and Coolify with an external PostgreSQL database.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Environment Variables](#environment-variables)
- [Local Development with Docker](#local-development-with-docker)
- [Deploying to Coolify](#deploying-to-coolify)
- [Connecting to External PostgreSQL](#connecting-to-external-postgresql)
- [Troubleshooting](#troubleshooting)

## Prerequisites

- Docker and Docker Compose installed (for local development)
- A Discord bot token (from [Discord Developer Portal](https://discord.com/developers/applications))
- Access to a PostgreSQL database (version 12 or higher recommended)
- Coolify instance (for production deployment)

## Environment Variables

The bot requires the following environment variables:

| Variable | Description | Required | Example |
|----------|-------------|----------|---------|
| `DISCORD_TOKEN` | Your Discord bot token | Yes | `MTIzNDU2Nzg5MDEyMzQ1Njc4OQ...` |
| `DATABASE_URL` | PostgreSQL connection string | Yes | `postgresql://user:pass@host:5432/db` |
| `RUST_LOG` | Logging level | No (default: info) | `info`, `debug`, `warn`, `error` |
| `RUST_BACKTRACE` | Enable Rust backtraces | No (default: 0) | `0` or `1` |

### Database URL Format

```
postgresql://[username]:[password]@[host]:[port]/[database]
```

Examples:
- Local: `postgresql://postgres:password@localhost:5432/discord_bot`
- Docker network: `postgresql://postgres:password@postgres-container:5432/discord_bot`
- Remote: `postgresql://botuser:securepass@db.example.com:5432/production_db`

## Local Development with Docker

### Option 1: Using Docker Compose (Recommended)

1. Copy the environment example file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your credentials:
   ```bash
   nano .env  # or use your preferred editor
   ```

3. Build and run:
   ```bash
   docker-compose up --build
   ```

### Option 2: Using Docker directly

1. Build the image:
   ```bash
   docker build -t discord-bot .
   ```

2. Run the container:
   ```bash
   docker run -d \
     --name discord-bot \
     -e DISCORD_TOKEN="your_token_here" \
     -e DATABASE_URL="postgresql://user:pass@host:5432/db" \
     -e RUST_LOG=info \
     discord-bot
   ```

## Deploying to Coolify

### Step 1: Prepare Your PostgreSQL Database

First, ensure you have a PostgreSQL database running in Coolify:

1. Create a new PostgreSQL service in Coolify (if not already done)
2. Note the following information:
   - Database name
   - Username
   - Password
   - Container/service name (e.g., `postgres-xyz123`)
   - Network name (usually `coolify`)

### Step 2: Deploy the Bot

1. **Create a new service in Coolify:**
   - Type: Docker Compose or Dockerfile
   - Repository: Link your Git repository
   - Branch: main (or your preferred branch)

2. **Configure environment variables in Coolify:**
   - Go to your service → Environment Variables
   - Add the required variables:
     ```
     DISCORD_TOKEN=your_discord_bot_token
     DATABASE_URL=postgresql://username:password@postgres-service-name:5432/database_name
     RUST_LOG=info
     ```

3. **Network Configuration:**
   
   If your PostgreSQL is in the same Coolify instance:
   - Both containers should be on the `coolify` network
   - Use the PostgreSQL container's service name as the host
   - Example: `postgresql://postgres:password@coolify-postgres-abc123:5432/botdb`

4. **Deploy:**
   - Click "Deploy" in Coolify
   - Monitor the build logs
   - Check application logs for successful database connection

### Step 3: Verify Connection

Check the logs in Coolify:
```
Bot connected as YourBotName with the id of 123456789
Conectado a la base de datos ✅
```

## Connecting to External PostgreSQL

### Scenario 1: PostgreSQL in Another Coolify Service

Update `docker-compose.yaml`:

```yaml
services:
  bot:
    # ... other configuration
    networks:
      - coolify

networks:
  coolify:
    external: true
```

Set `DATABASE_URL`:
```
postgresql://username:password@coolify-postgres-service-name:5432/database
```

### Scenario 2: PostgreSQL on Same Host (Different Network)

1. Find the PostgreSQL container's IP:
   ```bash
   docker inspect postgres-container-name | grep IPAddress
   ```

2. Use the IP in `DATABASE_URL`:
   ```
   postgresql://username:password@172.17.0.2:5432/database
   ```

### Scenario 3: PostgreSQL on External Host

Simply use the external hostname/IP:
```
postgresql://username:password@db.example.com:5432/database
```

Make sure:
- Firewall allows connections on port 5432
- PostgreSQL `pg_hba.conf` allows remote connections
- SSL is configured if required

## Docker Network Configuration

### Listing Networks

To see available Docker networks:
```bash
docker network ls
```

### Connecting Containers to Same Network

If containers are not on the same network:

```bash
# Connect bot to PostgreSQL network
docker network connect network-name bot-container-name
```

### Inspecting Networks

To see which containers are on a network:
```bash
docker network inspect coolify
```

## Troubleshooting

### Issue: "No se encontró DATABASE_URL" or "No se encontró DISCORD_TOKEN"

**Solution:** Ensure environment variables are properly set in Coolify's service settings.

### Issue: "Connection refused" to PostgreSQL

**Possible causes:**
1. Wrong hostname/IP in `DATABASE_URL`
2. Containers not on same network
3. PostgreSQL not accepting connections

**Solutions:**
- Verify PostgreSQL container is running: `docker ps | grep postgres`
- Check if containers are on same network: `docker network inspect coolify`
- Test connection from bot container:
  ```bash
  docker exec -it bot-container-name sh
  apt-get update && apt-get install -y postgresql-client
  psql "$DATABASE_URL"
  ```

### Issue: "password authentication failed"

**Solution:** 
- Verify username and password in `DATABASE_URL`
- Check PostgreSQL user permissions
- Ensure password doesn't contain special characters that need URL encoding

### Issue: Build fails in Coolify

**Solution:**
- Check build logs for specific errors
- Ensure all source files are committed to Git
- Verify `.sqlx` directory is included (needed for offline mode)
- Check Rust edition in `Cargo.toml` (should be 2021, not 2024)

### Issue: Bot starts but doesn't respond

**Solution:**
- Check bot has correct permissions in Discord Developer Portal
- Verify bot token is valid
- Check `RUST_LOG` for detailed logs
- Ensure intents are enabled in Discord Developer Portal

### Issue: High memory usage

**Solution:**
- The multi-stage Dockerfile reduces image size significantly
- Monitor with: `docker stats bot-container-name`
- Consider setting memory limits in Coolify

## Health Checks

The bot includes a basic health check. To verify it's running:

```bash
# Check if container is healthy
docker ps | grep discord-bot

# View logs
docker logs -f discord-bot

# In Coolify, use the built-in log viewer
```

## Best Practices

1. **Security:**
   - Never commit `.env` file or tokens to Git
   - Use strong passwords for PostgreSQL
   - Run bot as non-root user (already configured in Dockerfile)

2. **Logging:**
   - Set `RUST_LOG=info` for production
   - Use `RUST_LOG=debug` only for troubleshooting
   - Monitor logs regularly in Coolify

3. **Database:**
   - Keep PostgreSQL and bot on same network for better performance
   - Regular backups of PostgreSQL data
   - Use connection pooling (already handled by SQLx)

4. **Updates:**
   - Test updates in development first
   - Use Coolify's rollback feature if issues occur
   - Monitor bot after deployments

## Useful Commands

```bash
# View bot logs
docker logs -f discord-bot

# Restart bot
docker restart discord-bot

# Enter bot container
docker exec -it discord-bot sh

# Check bot resource usage
docker stats discord-bot

# Rebuild and restart
docker-compose up --build -d

# Stop and remove
docker-compose down
```

## Support

If you encounter issues not covered here:

1. Check Coolify documentation: https://coolify.io/docs
2. Check Docker logs for detailed error messages
3. Verify all environment variables are correctly set
4. Ensure PostgreSQL is accessible from the bot container

## Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Coolify Documentation](https://coolify.io/docs)
- [PostgreSQL Docker Image](https://hub.docker.com/_/postgres)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Serenity Discord Bot Framework](https://docs.rs/serenity/)