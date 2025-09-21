pub fn get_idle_time() -> Result<u64, Box<dyn std::error::Error>> {
  // check XDG_SESSION_TYPE and WAYLAND_DISPLAY. If we are on Wayland, return 0 as it's unsupported
  if std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland"
    || std::env::var("WAYLAND_DISPLAY").is_ok()
  {
    return Err("Wayland is not supported".into());
  }

  // Otherwise, attempt to get idle time using `xprintidle`
  let proc = std::process::Command::new("xprintidle")
    .output()?;

  if !proc.status.success() {
    return Err(format!("xprintidle returned non-zero exit status: {}", proc.status).into());
  }

  let output = String::from_utf8(proc.stdout)?;
  let idle_time = output.trim().parse::<u64>()?;

  Ok(idle_time)
}
