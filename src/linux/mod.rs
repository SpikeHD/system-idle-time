/**
Get system idle time (Linux).

This implementation supports both X11 (using `x11rb`) and Wayland (using `zbus`).
It checks the `XDG_SESSION_TYPE` environment variable to determine
the current session type.

The implementation for each compositor is feature-gated, so you can pick and choose which to support.
*/
pub fn get_idle_time() -> Result<std::time::Duration, Box<dyn std::error::Error>> {
  // Check if wayland
  if std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland" {
    #[cfg(feature = "wayland")]
    return get_idle_time_wayland();
    #[cfg(not(feature = "wayland"))]
    return Err("Wayland feature is disabled".into());
  }

  // Assume X11
  #[cfg(feature = "x11")]
  return get_idle_time_x11();
  #[cfg(not(feature = "x11"))]
  Err("X11 feature is disabled".into())
}

// Implementation based on https://github.com/bkbilly/dbus_idle/blob/master/dbus_idle/__init__.py
#[cfg(feature = "wayland")]
fn get_idle_time_wayland() -> Result<std::time::Duration, Box<dyn std::error::Error>> {
  use zbus::blocking::Connection;

  let conn = Connection::session()?;
  let reply = conn.call_method(
    Some("org.freedesktop.DBus"),
    "/org/freedesktop/DBus",
    Some("org.freedesktop.DBus"),
    "ListNames",
    &(),
  )?;

  let service_names: Vec<String> = reply.body().deserialize()?;

  let idle_service = service_names
    .into_iter()
    .find(|s| s.contains("IdleMonitor"))
    .ok_or("No IdleMonitor service found")?;

  // Convert name to path
  let service_path = format!("/{}", idle_service.replace('.', "/"));
  let service_path = format!("{}/Core", service_path);
  let interface = idle_service.clone();
  let reply = conn.call_method(
    Some(idle_service.as_str()),
    service_path.as_str(),
    Some(interface.as_str()),
    "GetIdletime",
    &(),
  )?;

  let idle_time_ms: u64 = reply.body().deserialize()?;
  Ok(std::time::Duration::from_millis(idle_time_ms))
}

#[cfg(feature = "x11")]
fn get_idle_time_x11() -> Result<std::time::Duration, Box<dyn std::error::Error>> {
  use x11rb::connection::Connection;
  use x11rb::protocol::screensaver::ConnectionExt;
  use x11rb::rust_connection::RustConnection;

  // Connect to X11
  let (conn, screen_num) = RustConnection::connect(None)?;
  let screen = &conn.setup().roots[screen_num];

  // Ensure the ScreenSaver extension is available
  let _ = conn.screensaver_query_version(1, 0)?.reply()?;

  // Query idle info
  let info = conn.screensaver_query_info(screen.root)?.reply()?;

  Ok(std::time::Duration::from_millis(
    info.ms_since_user_input.into(),
  ))
}
