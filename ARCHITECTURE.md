# Architecture Overview

This document provides a visual overview of the Discord bot architecture and deployment scenarios.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Discord API                           │
│                    (External Service)                        │
└────────────────────────┬────────────────────────────────────┘
                         │
                         │ WebSocket (Gateway)
                         │ REST API
                         │
┌────────────────────────▼────────────────────────────────────┐
│                                                               │
│                    Discord Bot Container                     │
│                  (bot-olim-p-code)                          │
│                                                               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Serenity Framework                                  │   │
│  │  ├─ Event Handler (ready, message, guild_create)   │   │
│  │  ├─ Commands Module                                 │   │
│  │  ├─ Events Module                                   │   │
│  │  └─ Tasks Module (daily tasks)                      │   │
│  └─────────────────────┬───────────────────────────────┘   │
│                        │                                     │
│  ┌─────────────────────▼───────────────────────────────┐   │
│  │  SQLx (PostgreSQL Client)                           │   │
│  │  ├─ Connection Pool                                 │   │
│  │  ├─ Query Execution                                 │   │
│  │  └─ Type-safe SQL                                   │   │
│  └─────────────────────┬───────────────────────────────┘   │
│                        │                                     │
└────────────────────────┼─────────────────────────────────────┘
                         │
                         │ TCP Port 5432
                         │ PostgreSQL Protocol
                         │
┌────────────────────────▼────────────────────────────────────┐
│                                                               │
│                PostgreSQL Container                          │
│                  (External Database)                         │
│                                                               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Database: discord_bot                              │   │
│  │  ├─ Table: guild_config                             │   │
│  │  │   ├─ guild_id (PRIMARY KEY)                      │   │
│  │  │   ├─ guild_name                                  │   │
│  │  │   ├─ welcome_channel_id                          │   │
│  │  │   ├─ daily_channel_id                            │   │
│  │  │   └─ ... (other config)                          │   │
│  │  └─ (Other tables as needed)                        │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

## Deployment Scenarios

### Scenario 1: Coolify Same Network (Recommended)

```
┌─────────────────────────────────────────────────────────────────┐
│                       Coolify Host Server                        │
│                                                                   │
│  ┌────────────────────────────────────────────────────────┐    │
│  │              Docker Network: coolify                    │    │
│  │                                                          │    │
│  │   ┌──────────────────────┐     ┌──────────────────┐   │    │
│  │   │   Discord Bot        │     │   PostgreSQL     │   │    │
│  │   │   Container          │────▶│   Container      │   │    │
│  │   │                      │     │                  │   │    │
│  │   │  Port: (internal)    │     │  Port: 5432      │   │    │
│  │   │  Name: bot-abc123    │     │  Name: pg-xyz456 │   │    │
│  │   └──────────────────────┘     └──────────────────┘   │    │
│  │                                                          │    │
│  │  DATABASE_URL=postgresql://user:pass@pg-xyz456:5432/db │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
         ▲
         │
         │ Internet
         │
    ┌────┴────┐
    │ Discord │
    │   API   │
    └─────────┘
```

**Key Points:**
- Both containers on same network
- Use service name as hostname
- Lowest latency
- Most reliable

### Scenario 2: Coolify Different Networks

```
┌─────────────────────────────────────────────────────────────────┐
│                       Coolify Host Server                        │
│                                                                   │
│  ┌──────────────────────┐         ┌────────────────────┐       │
│  │  Network: coolify    │         │  Network: postgres  │       │
│  │                      │         │                     │       │
│  │  ┌────────────────┐ │         │  ┌──────────────┐  │       │
│  │  │  Discord Bot   │ │         │  │ PostgreSQL   │  │       │
│  │  │  Container     │ │─ ─ ─ ─ ─│─▶│  Container   │  │       │
│  │  │                │ │  Bridge  │  │              │  │       │
│  │  │  IP: 172.18.0.5│ │         │  │ IP:172.19.0.3│  │       │
│  │  └────────────────┘ │         │  └──────────────┘  │       │
│  └──────────────────────┘         └────────────────────┘       │
│                                                                   │
│  DATABASE_URL=postgresql://user:pass@172.19.0.3:5432/db        │
│  OR connect bot to postgres network                             │
└─────────────────────────────────────────────────────────────────┘
```

**Key Points:**
- Containers on different networks
- Use IP address or bridge networks
- Less ideal but workable

### Scenario 3: External PostgreSQL

```
┌─────────────────────────────────────┐
│       Coolify Host Server           │
│                                     │
│  ┌────────────────────────────┐   │        ┌──────────────────┐
│  │  Docker Network: coolify    │   │        │  External Server │
│  │                              │   │        │                  │
│  │  ┌────────────────────┐    │   │        │  ┌────────────┐ │
│  │  │  Discord Bot       │    │   │  Port  │  │ PostgreSQL │ │
│  │  │  Container         │────┼───┼───5432─┼─▶│  Database  │ │
│  │  │                    │    │   │        │  │            │ │
│  │  └────────────────────┘    │   │        │  └────────────┘ │
│  └────────────────────────────┘   │        │                  │
│                                     │        │  db.example.com │
└─────────────────────────────────────┘        └──────────────────┘

DATABASE_URL=postgresql://user:pass@db.example.com:5432/dbname
```

**Key Points:**
- PostgreSQL on different server
- Requires network connectivity
- Firewall rules must allow port 5432
- SSL recommended for production

## Docker Multi-Stage Build Flow

```
┌──────────────────────────────────────────────────────────────┐
│                    Docker Build Process                       │
└──────────────────────────────────────────────────────────────┘

 STAGE 1: Builder                    STAGE 2: Runtime
 ┌─────────────────────┐            ┌──────────────────────┐
 │ rust:1.91-bullseye  │            │ debian:bullseye-slim │
 │                     │            │                      │
 │ ┌─────────────────┐ │            │ ┌────────────────┐  │
 │ │ Cargo.toml      │ │            │ │ /app/bot       │  │
 │ │ Cargo.lock      │ │            │ │ (binary only)  │  │
 │ │ src/            │ │            │ └────────────────┘  │
 │ │ .sqlx/          │ │            │                      │
 │ └─────────────────┘ │            │ ┌────────────────┐  │
 │         │           │            │ │ Runtime libs:  │  │
 │         ▼           │            │ │ - libssl1.1    │  │
 │ ┌─────────────────┐ │            │ │ - ca-certs     │  │
 │ │ cargo build     │ │            │ └────────────────┘  │
 │ │ --release       │ │            │                      │
 │ └─────────────────┘ │            │ User: botuser       │
 │         │           │            │ (non-root)          │
 │         ▼           │            │                      │
 │ ┌─────────────────┐ │   COPY     │                      │
 │ │ target/release/ │ │───────────▶│                      │
 │ │ bot-olim-p-code │ │            │                      │
 │ └─────────────────┘ │            │                      │
 │                     │            │                      │
 │  Size: ~2.5 GB     │            │  Size: ~200 MB      │
 │  (Discarded)       │            │  (Final Image)      │
 └─────────────────────┘            └──────────────────────┘
```

## Data Flow: Message Handling

```
Discord User sends message
        │
        ▼
┌───────────────────┐
│   Discord API     │
│   (Gateway)       │
└────────┬──────────┘
         │
         │ WebSocket Event
         │
         ▼
┌─────────────────────────────────────┐
│  Bot Container                       │
│                                      │
│  ┌────────────────────────────────┐ │
│  │ EventHandler::message()        │ │
│  └──────────┬─────────────────────┘ │
│             │                        │
│             ▼                        │
│  ┌────────────────────────────────┐ │
│  │ events::message::               │ │
│  │   handle_message()              │ │
│  └──────────┬─────────────────────┘ │
│             │                        │
│             ├─ Check if command     │
│             │                        │
│             ▼                        │
│  ┌────────────────────────────────┐ │
│  │ commands::process()            │ │
│  └──────────┬─────────────────────┘ │
│             │                        │
│             ▼                        │
│  ┌────────────────────────────────┐ │
│  │ Database Query via SQLx        │ │
│  │ - Get guild config             │ │
│  │ - Update user data             │ │
│  │ - Store message info           │ │
│  └──────────┬─────────────────────┘ │
└─────────────┼────────────────────────┘
              │
              ▼
    ┌─────────────────┐
    │   PostgreSQL    │
    │   Database      │
    └─────────────────┘
```

## Environment Variables Flow

```
┌──────────────────────────────────────────────────────────┐
│                   Deployment Platform                     │
│                      (Coolify UI)                         │
│                                                            │
│  Environment Variables:                                   │
│  ┌──────────────────────────────────────────────────┐   │
│  │ DISCORD_TOKEN=MTIzNDU2Nzg5...                     │   │
│  │ DATABASE_URL=postgresql://user:pass@host:5432/db │   │
│  │ RUST_LOG=info                                     │   │
│  └────────────────────┬─────────────────────────────┘   │
└───────────────────────┼──────────────────────────────────┘
                        │
                        │ Injected at runtime
                        │
                        ▼
┌──────────────────────────────────────────────────────────┐
│               Docker Container Runtime                    │
│                                                            │
│  ┌──────────────────────────────────────────────────┐   │
│  │  /app/bot (Binary)                                │   │
│  │                                                    │   │
│  │  dotenvy::dotenv().ok();                         │   │
│  │  env::var("DISCORD_TOKEN")                       │   │
│  │  env::var("DATABASE_URL")                        │   │
│  │                                                    │   │
│  │  ┌──────────────────────────────────────────┐   │   │
│  │  │ Uses variables to:                        │   │   │
│  │  │ - Connect to Discord                      │   │   │
│  │  │ - Connect to PostgreSQL                   │   │   │
│  │  │ - Configure logging                       │   │   │
│  │  └──────────────────────────────────────────┘   │   │
│  └──────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────┘
```

## Component Interaction Matrix

```
╔═══════════════╦═════════════╦══════════════╦═════════════╗
║ Component     ║ Discord API ║ PostgreSQL   ║ File System ║
╠═══════════════╬═════════════╬══════════════╬═════════════╣
║ EventHandler  ║     ✓       ║      ✓       ║      -      ║
║ Commands      ║     ✓       ║      ✓       ║      -      ║
║ Tasks         ║     ✓       ║      ✓       ║      -      ║
║ API Module    ║     -       ║      -       ║      -      ║
║ Database Pool ║     -       ║      ✓       ║      -      ║
╚═══════════════╩═════════════╩══════════════╩═════════════╝
```

## Network Ports

```
┌─────────────────────────────────────────────────────────────┐
│                        Port Usage                            │
└─────────────────────────────────────────────────────────────┘

Discord Bot Container:
  - No exposed ports (outbound only)
  - Connects OUT to Discord API (443/WSS)
  - Connects OUT to PostgreSQL (5432/TCP)

PostgreSQL Container:
  - Port 5432 (internal to Docker network)
  - NOT exposed to internet (security)
  - Accessible only within Docker network

External Connections:
  - Discord API: 443 (HTTPS/WSS) - Outbound
  - PostgreSQL: 5432 (TCP) - Outbound from bot
```

## Security Layers

```
┌─────────────────────────────────────────────────────────┐
│                    Security Model                        │
└─────────────────────────────────────────────────────────┘

Layer 1: Container Isolation
  ├─ Non-root user (UID 1000)
  ├─ Read-only root filesystem (optional)
  └─ Minimal attack surface

Layer 2: Network Isolation
  ├─ Private Docker network
  ├─ No exposed ports
  └─ Controlled service connections

Layer 3: Secret Management
  ├─ Environment variables (not in code)
  ├─ No secrets in Git
  └─ Coolify encrypted storage

Layer 4: Dependencies
  ├─ Minimal runtime image
  ├─ No build tools in production
  └─ Regular security updates
```

## Resource Requirements

```
╔═══════════════════╦═══════════╦═══════════╦═══════════╗
║ Component         ║ Min RAM   ║ Rec RAM   ║ CPU       ║
╠═══════════════════╬═══════════╬═══════════╬═══════════╣
║ Discord Bot       ║ 128 MB    ║ 256 MB    ║ 0.5 vCPU  ║
║ PostgreSQL        ║ 256 MB    ║ 512 MB    ║ 0.5 vCPU  ║
║ Total             ║ 384 MB    ║ 768 MB    ║ 1.0 vCPU  ║
╚═══════════════════╩═══════════╩═══════════╩═══════════╝
```

## Scaling Considerations

```
Single Instance (Current):
  [Discord Bot] ←→ [PostgreSQL]
      ▲
      │
  Discord API

Horizontal Scaling (Future):
  [Discord Bot 1] ─┐
  [Discord Bot 2] ─┼──→ [PostgreSQL Primary]
  [Discord Bot 3] ─┘         │
                              ├──→ [PostgreSQL Replica]
                              └──→ [PostgreSQL Replica]
```

**Note:** Current architecture supports single instance.
For scaling, implement:
- Shared state management
- Database connection pooling (already implemented)
- Load balancing (Discord API handles this)

---

**Last Updated:** November 7, 2024  
**Version:** 1.0  
**Deployment:** Coolify with External PostgreSQL