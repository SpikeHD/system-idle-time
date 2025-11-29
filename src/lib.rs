/*! # system-idle-time
Cross-platform Rust library for getting the last user input on the system.

## Supported Platforms

- Windows
- Linux (X11 and Wayland)
- macOS

## Example

```rust
use system_idle_time::get_idle_time;

match get_idle_time() {
    Ok(idle_time) => println!("Idle time: {} ms", idle_time.as_millis()),
    Err(e) => eprintln!("Error getting idle time: {}", e),
}
```
*/

#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
pub use win::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;
