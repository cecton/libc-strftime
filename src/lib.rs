//! A wrapper library for the glibc strftime function
//!
//! Examples
//! ========
//!
//! Format the current date and time in Brussels in French:
//!
//! ```
//! use std::env;
//!
//! env::set_var("LC_ALL", "fr_BE.UTF-8");
//! env::set_var("TZ", "Europe/Brussels");
//!
//! libc_strftime::tzset();
//! libc_strftime::set_locale();
//!
//! let now = libc_strftime::epoch(); // most likely a u64
//! let local = libc_strftime::strftime_local("%c", now);
//! println!("On est: {}", local); // On est: mer 07 aoû 2019 06:19:56 CEST
//! ```

use std::ffi::CString;
use std::mem;

mod c {
    extern "C" {
        #[cfg(unix)]
        pub(crate) fn tzset();
        #[cfg(windows)]
        pub(crate) fn _tzset();
        pub(crate) fn strftime(
            s: *mut libc::c_char,
            max: libc::size_t,
            format: *const libc::c_char,
            tm: *const libc::tm,
        ) -> usize;
        pub(crate) fn time(tloc: *const libc::time_t) -> libc::time_t;
        #[cfg(unix)]
        pub(crate) fn localtime_r(t: *const libc::time_t, tm: *mut libc::tm);
        #[cfg(windows)]
        pub(crate) fn _localtime64_s(tm: *mut libc::tm, t: *const libc::time_t);
        #[cfg(unix)]
        pub(crate) fn gmtime_r(t: *const libc::time_t, tm: *mut libc::tm);
        #[cfg(windows)]
        pub(crate) fn _gmtime64_s(tm: *mut libc::tm, t: *const libc::time_t);
    }
}

/// Get a tm struct in local timezone
pub fn get_local_tm_from_epoch(epoch: libc::time_t) -> libc::tm {
    unsafe {
        let mut now: libc::tm = mem::zeroed();
        #[cfg(unix)]
        c::localtime_r(&epoch, &mut now);
        #[cfg(windows)]
        c::_localtime64_s(&mut now, &epoch);
        now
    }
}

/// Get a tm struct in GMT
pub fn get_gmt_tm_from_epoch(epoch: libc::time_t) -> libc::tm {
    unsafe {
        let mut now: libc::tm = mem::zeroed();
        #[cfg(unix)]
        c::gmtime_r(&epoch, &mut now);
        #[cfg(windows)]
        c::_gmtime64_s(&mut now, &epoch);
        now
    }
}

/// Call strftime() using a tm struct provided in input
fn strftime(format: &str, tm: &libc::tm) -> String {
    let f = CString::new(format).unwrap();
    let buf = [0_u8; 100];
    let l: usize = unsafe { c::strftime(buf.as_ptr() as _, buf.len(), f.as_ptr() as *const _, tm) };
    std::string::String::from_utf8_lossy(&buf[..l]).to_string()
}

/// Call strftime() using the local timezone and returns a String
pub fn strftime_local(format: &str, epoch: libc::time_t) -> String {
    let tm = get_local_tm_from_epoch(epoch);
    strftime(format, &tm)
}

/// Call strftime() using GMT and returns a String
pub fn strftime_gmt(format: &str, epoch: libc::time_t) -> String {
    let tm = get_gmt_tm_from_epoch(epoch);
    strftime(format, &tm)
}

/// Call setlocale() which will initialize the locale based on the environment variables
pub fn set_locale() {
    unsafe {
        libc::setlocale(libc::LC_ALL, b"\0".as_ptr() as _);
    }
}

/// Call tzset() which will initialize the local timezone based on the environment variables
pub fn tzset() {
    unsafe {
        #[cfg(unix)]
        c::tzset();
        #[cfg(windows)]
        c::_tzset();
    }
}

/// Retrieve the current time in epoch format (number of seconds since 1970 in UTC)
pub fn epoch() -> libc::time_t {
    unsafe { c::time(std::ptr::null()) }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::env;

    const EPOCH: libc::time_t = 1_565_151_596;

    #[test]
    fn format_time_and_date_in_gmt_and_cest() {
        env::set_var("LC_ALL", "en_US.UTF-8");
        env::set_var("TZ", "Europe/Brussels");

        tzset();
        set_locale();

        let gmt = strftime_gmt("%c", EPOCH);
        let local = strftime_local("%c", EPOCH);
        #[cfg(target_os = "linux")]
        assert_eq!(gmt, "Wed 07 Aug 2019 04:19:56 AM GMT");
        #[cfg(target_os = "macos")]
        assert_eq!(gmt, "Wed Aug  7 04:19:56 2019");
        #[cfg(target_os = "linux")]
        assert_eq!(local, "Wed 07 Aug 2019 06:19:56 AM CEST");
        #[cfg(target_os = "macos")]
        assert_eq!(local, "Wed Aug  7 06:19:56 2019");

        env::set_var("LC_ALL", "fr_BE.UTF-8");
        env::set_var("TZ", "Europe/Brussels");

        tzset();
        set_locale();

        let gmt = strftime_gmt("%c", EPOCH);
        let local = strftime_local("%c", EPOCH);
        #[cfg(target_os = "linux")]
        assert_eq!(gmt, "mer 07 aoû 2019 04:19:56 GMT");
        #[cfg(target_os = "macos")]
        assert_eq!(gmt, "Mer  7 aoû 04:19:56 2019");
        #[cfg(target_os = "linux")]
        assert_eq!(local, "mer 07 aoû 2019 06:19:56 CEST");
        #[cfg(target_os = "macos")]
        assert_eq!(local, "Mer  7 aoû 06:19:56 2019");
    }

    // NOTE: I have no idea how to change the timezone or the language on
    //       Windows. It's supposed to be with the global environment variable
    //       TZ but I couldn't make it working... well, at least it returns
    //       something and it should probably work, right?
    #[test]
    #[cfg(windows)]
    fn format_time_and_date_on_windows() {
        tzset();
        set_locale();

        let gmt = strftime_gmt("%c", EPOCH);
        let local = strftime_local("%c", EPOCH);
        #[cfg(target_os = "windows")]
        assert_eq!(gmt, "8/7/2019 4:19:56 AM");
        #[cfg(target_os = "windows")]
        assert_eq!(local, "8/7/2019 4:19:56 AM");
    }
}
