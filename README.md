# ED Colony Bot

## Overview
ED Colony Bot is a Discord bot designed to track construction sites and commodity deliveries in Elite Dangerous. The bot helps coordinate community efforts for system colonization by tracking needed resources and player contributions.

## Features
- **Construction Site Tracking**: Monitor multiple construction projects simultaneously
- **Commodity Management**: Track needed commodities and quantities for each site
- **Detailed Reporting**: View remaining commodities via slash commands
- **Live Status Display**: Channel-based embeds showing real-time progress for all sites
- **Role-Based Access**:
  - **System Architects** (admin role): Create, rename, and manage construction sites
  - **Haulers**: Log commodity deliveries to construction sites
- **CSV Import**: Add new sites by uploading commodity requirement spreadsheets
- **Contribution Tracking**: Record which players delivered what quantities
- **Admin Tools**: Edit quantities to correct errors when needed

## Project Structure
```
EDColonyBot
├── src
│   ├── main.rs               # Entry point of the application
│   └── .gitignore                # Files and directories to ignore by Git
├── Cargo.toml                # Cargo configuration file
├── .env                      # Environment variables (not in git)
└── README.md                 # Project documentation
```

## Setup Instructions
1. **Install Rust**: Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install) to install Rust and Cargo.
2. **Clone the Repository**: Use Git to clone the repository to your local machine.
   ```
   git clone <repository-url>
   ```
3. **Navigate to the Project Directory**:
   ```
   cd EDColonyBot
   ```
4. **Create Environment File**: Create a .env file with the following variables:
   ```
   DISCORD_TOKEN=your_discord_bot_token
   DISCORD_GUILD_ID=your_server_id
   DATABASE_URL=sqlite:edcolonybot.db
   ```
5. **Install Dependencies**: Run the following command to install the necessary dependencies.
   ```
   cargo build
   ```
6. **Run the Bot**: Start the bot using Cargo.
   ```
   cargo run
   ```

## Discord Setup
1. Create a Discord bot at the [Discord Developer Portal](https://discord.com/developers/applications)
2. Enable necessary intents (Server Members Intent, Message Content Intent)
3. Invite the bot to your server with appropriate permissions
4. Create roles in your Discord server:
   - "System Architect" for administrators
   - "Hauler" for users who can log deliveries

## Available Commands

### For System Architects (Admins)
- `/site create [name]` - Create a new construction site
- `/site remove [name]` - Remove an existing construction site
- `/site rename [old_name] [new_name]` - Rename a construction site
- `/site upload [name] [csv_file]` - Upload commodity requirements from CSV
- `/edit [site] [commodity] [quantity]` - Adjust commodity quantities

### For Haulers
- `/delivered [site] [commodity] [quantity]` - Log a delivery to a construction site

### For Everyone
- `/sites` - List all active construction sites
- `/site info [name]` - Show details about a specific site
- `/status` - Show overall progress of all sites

## Contributing
Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
