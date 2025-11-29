use core_foundation::base::{CFTypeRef, TCFType};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::number::{CFNumberGetValue, kCFNumberSInt64Type};
use core_foundation::string::CFStringRef;

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {
  fn IOServiceGetMatchingService(master: u32, matching: CFDictionaryRef) -> u32;

  fn IOServiceMatching(name: *const i8) -> CFDictionaryRef;

  fn IORegistryEntryCreateCFProperty(
    entry: u32,
    key: CFStringRef,
    allocator: CFTypeRef,
    options: u32,
  ) -> CFTypeRef;

  fn IOObjectRelease(obj: u32) -> i32;
}

/**
Get system idle time (macOS).

This implementation reads the `HIDIdleTime` property from the `IOHIDSystem` service
in the I/O Registry.

**Unsafe disclaimer:** This function uses unsafe code to interface with macOS IOKit APIs.
*/
pub fn get_idle_time() -> Result<std::time::Duration, Box<dyn std::error::Error>> {
  unsafe {
    let matching = IOServiceMatching("IOHIDSystem\0".as_ptr() as *const i8);
    let service = IOServiceGetMatchingService(0, matching);
    if service == 0 {
      return Err("IOServiceGetMatchingService failed".into());
    }

    let key = core_foundation::string::CFString::new("HIDIdleTime");
    let value_ref =
      IORegistryEntryCreateCFProperty(service, key.as_concrete_TypeRef(), std::ptr::null(), 0);

    IOObjectRelease(service);

    if value_ref.is_null() {
      return Err("HIDIdleTime read failed".into());
    }

    // Convert CFNumber → u64
    let mut idle_ns: u64 = 0;
    let ok = CFNumberGetValue(
      value_ref as *const _,
      kCFNumberSInt64Type,
      &mut idle_ns as *mut u64 as *mut _,
    );

    if !ok {
      return Err("CFNumberGetValue failed".into());
    }

    Ok(std::time::Duration::from_nanos(idle_ns))
  }
}
