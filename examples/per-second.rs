use std::time::Duration;
use system_idle_time::get_idle_time;

fn main() {
  loop {
    std::thread::sleep(Duration::from_secs(1));

    match get_idle_time() {
      Ok(idle_time) => println!("Idle time: {} ms", idle_time.as_millis()),
      Err(e) => eprintln!("Error getting idle time: {}", e),
    }
  }
}
