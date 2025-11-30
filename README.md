# system-idle-time
Cross-platform Rust library for getting the last user input on the system.

# Supported Platforms

- Windows
- Linux (X11 and Wayland via DBus)
- macOS

# Installation

```bash
cargo add system-idle-time
```

# Usage

```rust
use system_idle_time::get_idle_time;

fn main() {
    match get_idle_time() {
        Ok(idle_time) => println!("Idle time: {} ms", idle_time.as_millis()),
        Err(e) => eprintln!("Error getting idle time: {}", e),
    }
}
```

# Contributing

PRs, issues, etc are all welcome!
