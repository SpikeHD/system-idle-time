use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetLastInputInfo;
use windows_sys::Win32::System::SystemInformation::GetTickCount;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::LASTINPUTINFO;

pub fn get_idle_time() -> Result<std::time::Duration, Box<dyn std::error::Error>> {
  let mut li = LASTINPUTINFO {
    cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
    dwTime: 0,
  };

  let ok = unsafe { GetLastInputInfo(&mut li as *mut _) };
  if ok == 0 {
    return Err("GetLastInputInfo failed".into());
  }

  let now = unsafe { GetTickCount() };

  let delta_ms = if li.dwTime > now {
    // Tick count wrapped
    (u32::MAX - li.dwTime) as u64 + now as u64
  } else {
    (now - li.dwTime) as u64
  };

  std::time::Duration::from_millis(delta_ms)
}
