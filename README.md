# system-idle-time
Cross-platform Rust library for getting the last user input on the system.

# Installation

```bash
cargo add system-idle-time
```

# Usage

```rust
use system_idle_time::get_idle_time;

fn main() {
  let duration = get_idle_time();
  
  println!("Time since last input: {:?}", duration);
}
```

# Contributing

PRs, issues, etc are all welcome!