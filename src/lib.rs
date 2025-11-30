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

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
use linux::get_idle_time as plat_idle_time;
#[cfg(target_os = "windows")]
use win::get_idle_time as plat_idle_time;
#[cfg(target_os = "macos")]
use macos::get_idle_time as plat_idle_time;

/// Get system idle time as a `Duration`.
#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
pub fn get_idle_time() -> Result<std::time::Duration, Box<dyn std::error::Error>> {
  plat_idle_time()
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn get_idle_time() -> Result<std::time::Duration, Box<dyn std::error::Error>> {
  Err("Unsupported platform".into())
}
