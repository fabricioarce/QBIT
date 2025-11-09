# Documentation Index

Welcome to the Bot Olim P-Code documentation! This guide will help you navigate all available documentation and find exactly what you need.

## 📖 Quick Navigation

### For New Users
- **[Main README](../README.md)** - Project overview and quick start
- **[Getting Started](#-getting-started)** - Your first steps
- **[Commands Reference](#-commands)** - Available bot commands

### For Developers  
- **[Development Guide](#-development)** - Architecture and setup
- **[Contributing Guide](../CONTRIBUTING.md)** - How to contribute
- **[Architecture Overview](development/ARCHITECTURE.md)** - System design

### For Deployment
- **[Deployment Guides](#-deployment)** - Production deployment
- **[Docker Guide](deployment/DOCKER-README.md)** - Docker deployment
- **[Coolify Quick Start](deployment/COOLIFY-QUICKSTART.md)** - Fast deployment

## 🏗️ Documentation Structure

```
docs/
├── README.md                    # This file - documentation index
├── commands/                    # Command documentation
│   ├── account.md              # Account management commands  
│   ├── balance.md              # Economy and balance commands
│   └── solved.md               # Problem tracking commands
├── development/                 # Developer documentation
│   ├── ARCHITECTURE.md         # System architecture overview
│   ├── Database_idea.md        # Database schema and design
│   └── function_files.md       # Code organization guide
└── deployment/                  # Deployment documentation
    ├── DOCKER-README.md        # Comprehensive Docker guide
    ├── COOLIFY-QUICKSTART.md   # Quick Coolify deployment
    ├── DEPLOYMENT-CHECKLIST.md # Step-by-step checklist
    └── README.docker.md        # Docker deployment reference
```

## 🚀 Getting Started

### New to the Bot?
1. **[Main README](../README.md)** - Understand what this bot does
2. **[Commands Reference](#-commands)** - Learn available commands
3. **[Quick Start Guide](../README.md#-quick-start)** - Get the bot running

### Setting Up Development?
1. **[Development Setup](../README.md#-development-setup)** - Development environment
2. **[Architecture Overview](development/ARCHITECTURE.md)** - Understand the system
3. **[Database Design](development/Database_idea.md)** - Database structure
4. **[Project Structure](development/function_files.md)** - Code organization

### Deploying to Production?
1. **[Deployment Checklist](deployment/DEPLOYMENT-CHECKLIST.md)** - Pre-deployment tasks
2. **[Coolify Quick Start](deployment/COOLIFY-QUICKSTART.md)** - Fast deployment (5 minutes)
3. **[Docker Guide](deployment/DOCKER-README.md)** - Comprehensive deployment guide

## 📋 Commands

### User Commands
- **[Account Management](commands/account.md)** - Profile linking and user settings
- **[Balance & Economy](commands/balance.md)** - Points, shop, and transactions  
- **[Problem Tracking](commands/solved.md)** - Mark problems solved, view progress

### Command Categories
| Category | Description | Commands |
|----------|-------------|----------|
| **Codeforces** | Problem solving and tracking | `!problem`, `!solved`, `!stats` |
| **Economy** | Points and shop system | `!balance`, `!shop`, `!buy` |
| **Moderation** | Server management | `!warn`, `!kick`, `!ban` |
| **Utility** | General bot functions | `!help`, `!ping`, `!info` |

## 🏗️ Development  

### Architecture & Design
- **[System Architecture](development/ARCHITECTURE.md)** - High-level system design with diagrams
- **[Database Schema](development/Database_idea.md)** - Complete database design and relationships
- **[Code Organization](development/function_files.md)** - Project structure and file organization

### Key Concepts
- **Modular Design** - Each feature is self-contained
- **Event-Driven** - Responds to Discord events
- **Database-Driven** - PostgreSQL for persistence
- **Async Architecture** - Built on Tokio runtime

### Development Workflow
1. **Fork & Clone** - Get the code
2. **Setup Environment** - Database and tokens
3. **Run Tests** - `cargo test`
4. **Make Changes** - Follow coding standards
5. **Submit PR** - With tests and documentation

## 🚀 Deployment

### Quick Deployment
- **[Coolify Quick Start](deployment/COOLIFY-QUICKSTART.md)** ⚡ **(5-10 minutes)**
  - Fast deployment guide
  - Common issues and solutions
  - Verification checklist

### Comprehensive Guides  
- **[Docker README](deployment/DOCKER-README.md)** 📖 **(15-20 minutes)**
  - Complete deployment guide
  - Troubleshooting section
  - Local development setup
  - Production best practices

### Step-by-Step Process
- **[Deployment Checklist](deployment/DEPLOYMENT-CHECKLIST.md)** ✅ **(During deployment)**
  - Interactive checklist
  - Pre-deployment tasks
  - Post-deployment verification
  - Security considerations

### Docker Reference
- **[Docker Index](deployment/README.docker.md)** 🐳
  - Navigation between Docker docs
  - Quick commands reference
  - Troubleshooting links

## 🎯 Use Cases & Scenarios

### "I want to understand the bot"
→ **[Main README](../README.md)** → **[Commands](commands/)** → **[Architecture](development/ARCHITECTURE.md)**

### "I want to deploy quickly"  
→ **[Coolify Quick Start](deployment/COOLIFY-QUICKSTART.md)**

### "I need comprehensive deployment info"
→ **[Docker README](deployment/DOCKER-README.md)**

### "I'm getting deployment errors"
→ **[Docker README](deployment/DOCKER-README.md)** (Troubleshooting section)

### "I want to contribute code"
→ **[Development Guide](development/)** → **[Contributing](../CONTRIBUTING.md)**

### "I need to understand the database"  
→ **[Database Design](development/Database_idea.md)**

### "I want to add new commands"
→ **[Code Organization](development/function_files.md)** → **[Architecture](development/ARCHITECTURE.md)**

## 📊 Documentation Statistics

| Section | Files | Purpose |
|---------|-------|---------|
| **Commands** | 3 files | User command reference |
| **Development** | 3 files | Architecture and development |
| **Deployment** | 4 files | Production deployment guides |
| **Total** | **10 files** | Complete documentation coverage |

## 🎓 Learning Paths

### Beginner Path
1. **[Main README](../README.md)** - Project overview
2. **[Commands Reference](commands/)** - Learn bot capabilities  
3. **[Quick Start](../README.md#-quick-start)** - Get it running

### Developer Path  
1. **[Architecture](development/ARCHITECTURE.md)** - Understand the design
2. **[Database Schema](development/Database_idea.md)** - Data model
3. **[Code Structure](development/function_files.md)** - Navigate the code
4. **[Contributing Guide](../CONTRIBUTING.md)** - Start contributing

### DevOps Path
1. **[Docker Guide](deployment/DOCKER-README.md)** - Understand deployment
2. **[Deployment Checklist](deployment/DEPLOYMENT-CHECKLIST.md)** - Follow process
3. **[Coolify Guide](deployment/COOLIFY-QUICKSTART.md)** - Cloud deployment

## 🔍 Find What You Need

### By Role
| Role | Start Here | Then Read |
|------|------------|-----------|
| **End User** | [Main README](../README.md) | [Commands](commands/) |
| **Developer** | [Architecture](development/ARCHITECTURE.md) | [Development](development/) |
| **DevOps** | [Docker Guide](deployment/DOCKER-README.md) | [Deployment](deployment/) |
| **Contributor** | [Contributing](../CONTRIBUTING.md) | [Development](development/) |

### By Task
| Task | Documentation |
|------|---------------|
| **First time setup** | [Main README](../README.md) → [Quick Start](../README.md#-quick-start) |
| **Add new feature** | [Architecture](development/ARCHITECTURE.md) → [Code Structure](development/function_files.md) |
| **Deploy to production** | [Deployment Checklist](deployment/DEPLOYMENT-CHECKLIST.md) |
| **Understand commands** | [Commands Reference](commands/) |
| **Debug database issues** | [Database Schema](development/Database_idea.md) |
| **Troubleshoot deployment** | [Docker README](deployment/DOCKER-README.md) |

## ❓ Still Can't Find It?

1. **Search within docs** - Use Ctrl+F to search for keywords
2. **Check main README** - [../README.md](../README.md) has overview info
3. **Look at related sections** - Documentation is cross-linked
4. **Check troubleshooting** - Most guides have troubleshooting sections

## 🆘 Get Help

- **Issues**: [GitHub Issues](https://github.com/your-username/bot-olim-p-code/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-username/bot-olim-p-code/discussions)
- **Documentation**: You're reading it! 📚

## 🔄 Keep Updated

This documentation is actively maintained. Check the timestamps on files to see when they were last updated.

**Last Updated**: November 2025  
**Version**: 1.0  
**Status**: ✅ Complete and current

---

**Happy coding! 🚀**