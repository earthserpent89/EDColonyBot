# ED Colony Bot

## Overview
ED Colony Bot is a Discord bot designed to track construction sites in the game Elite Dangerous. It provides users with commands to manage and retrieve information about construction sites, enhancing the gameplay experience.

## Features
- Command handling for user interactions
- Event listening for real-time updates
- Data management for construction sites
- API interactions for external data retrieval

## Project Structure
```
ed-colony-bot
├── src
│   ├── main.rs          # Entry point of the application
│   ├── bot              # Module for bot-related functionality
│   │   ├── mod.rs
│   │   ├── commands.rs
│   │   └── events.rs
│   ├── data             # Module for data handling
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   └── database.rs
│   └── elite            # Module for Elite Dangerous interactions
│       ├── mod.rs
│       ├── construction.rs
│       └── api.rs
├── .gitignore           # Files and directories to ignore by Git
├── Cargo.toml          # Cargo configuration file
├── config.json         # Configuration settings for the bot
└── README.md           # Project documentation
```

## Setup Instructions
1. **Install Rust**: Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install) to install Rust and Cargo.
2. **Clone the Repository**: Use Git to clone the repository to your local machine.
   ```
   git clone <repository-url>
   ```
3. **Navigate to the Project Directory**:
   ```
   cd ed-colony-bot
   ```
4. **Install Dependencies**: Run the following command to install the necessary dependencies.
   ```
   cargo build
   ```
5. **Configure the Bot**: Edit the `config.json` file to include your API keys and other configuration settings.
6. **Run the Bot**: Start the bot using Cargo.
   ```
   cargo run
   ```

## Usage
Once the bot is running, you can interact with it through Discord by using the defined commands. Refer to the command documentation in `src/bot/commands.rs` for a list of available commands and their usage.

## Contributing
Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.