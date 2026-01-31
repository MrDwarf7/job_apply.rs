# Job Apply Bot

A Rust-based automation bot for LinkedIn (and planned Seek) job applications using a Finite State Machine (FSM) architecture.
Built with [fantoccini](https://docs.rs/fantoccini) (WebDriver/Selenium client) for browser automation.

## Features

- **FSM-driven workflow** - Clean state transitions for navigating, searching, and applying to jobs
- **Configurable via TOML** - All settings externalized to `config/config.toml`
- **Automatic driver management** - Spawns and monitors ChromeDriver with graceful shutdown
- **Structured logging** - Tracing-based logging with configurable levels
- **Output tracking** - CSV-based success/failure logging for applications

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         State Machine                           │
├─────────────┬─────────────┬─────────────┬─────────────┬────────┤
│  Navigate   │   Search    │   Action    │   Paused    │ Error  │
│  (ToUrl,    │  (FindBy,   │  (Click,    │  (user      │ (fatal │
│  ToElement) │  FindAllBy) │  InputText) │  intervention│ issues)│
└─────────────┴─────────────┴─────────────┴─────────────┴────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      fantoccini Client                          │
│                   (WebDriver / Selenium)                        │
└─────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
job_apply/
├── config/
│   └── config.toml          # Application configuration
├── output/                   # Generated output files
│   ├── success.csv          # Successful applications log
│   └── failure.csv          # Failed applications log
├── src/
│   ├── config/              # Configuration modules
│   │   ├── core.rs          # AppConfig - main config struct
│   │   ├── driver.rs        # WebDriver config & process management
│   │   ├── driver_type.rs   # Supported driver types (Chrome/Chromium)
│   │   ├── general.rs       # General settings (iterations, etc.)
│   │   ├── logging.rs       # Logging configuration
│   │   ├── login.rs         # Login credentials config
│   │   └── output.rs        # Output file paths config
│   ├── states/              # FSM state implementations
│   │   ├── action.rs        # Click/InputText actions
│   │   ├── error_state.rs   # Unrecoverable error handling
│   │   ├── navigate.rs      # URL/element navigation + element enums
│   │   ├── paused.rs        # User intervention state
│   │   └── search.rs        # Element search by locator
│   ├── constants.rs         # Static paths and constants
│   ├── error.rs             # Custom error types (thiserror)
│   ├── macros.rs            # Helper macros
│   ├── main.rs              # Entry point & app lifecycle
│   ├── prelude.rs           # Common imports, Result type, utilities
│   └── state.rs             # Top-level State struct (config + client)
└── Cargo.toml
```

## Configuration

Edit `config/config.toml`:

```toml
[general]
unfollow_companies = true      # Untick "follow company" checkbox
maximum_iterations = 255       # Max application loop iterations

[driver]
driver_type = "chrome"         # "chrome" or "chromium"
driver_path = "/usr/bin/chromedriver"
background_driver_check_delay_secs = 1

[logging]
log_level = "info"             # trace, debug, info, warn, error

[logging.debug_logging]
enabled = false
file_path = "./output/output.log"

[output.success]
file_path = "./output/success.csv"

[output.failure]
file_path = "./output/failure.csv"

[login]
login_url = "https://www.linkedin.com/login"
username = "your_username_here"
password = "your_password_here"
```

## State Machine

The bot operates as a Finite State Machine with these states:

| State                  | Description                                   |
| ---------------------- | --------------------------------------------- |
| `Navigate`             | Go to a URL or focus an element               |
| `Search`               | Find element(s) by locator (CSS, XPath, etc.) |
| `Action`               | Perform clicks or text input                  |
| `Paused`               | Wait for user intervention (e.g., CAPTCHA)    |
| `MaxIterationsReached` | Iteration limit hit                           |
| `ErrorState`           | Unrecoverable error with custom handler       |

All states implement the `Transition` trait:

```rust
#[async_trait]
pub trait Transition {
    async fn execute(&self) -> Result<()>;
    async fn current_state(&self) -> &dyn Transition;
}
```

## Element Types

The `navigate.rs` module defines valid interaction elements:

- **LoginElements**: `LoginUsername`, `LoginPassword`
- **JobElements**: `Listings`, `Description` (with `ApplyButton`, `Submit`)
- **ApplicationElements**: `BulletPoint`, `TextField`, `NumberField`, `TickBox`, `ResumeUpload`, `FollowCompanyTickBox`

## Prerequisites

- Rust 2024 edition (nightly)
- ChromeDriver (or compatible WebDriver) on PATH or specified in config
- `cranelift` codegen backend enabled

## Building & Running

```bash
# Build
cargo build

# Run
cargo run

# Watch mode (development)
cargo watch -q -c -w src/ -x run
```

## Development Status

🚧 **In Development** - Core scaffolding complete, state logic implementations in progress.

### TODO

- [ ] Implement actual WebDriver interactions in state handlers
- [ ] Add Seek.com.au support
- [ ] CAPTCHA detection → Paused state transition
- [ ] Resume upload handling
- [ ] Application form field detection and filling
- [ ] Exponential backoff for retries

## License

See [LICENSE](./LICENSE)
