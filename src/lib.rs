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

mod c {
    extern "C" {
        pub(crate) fn tzset();
        pub(crate) fn strftime(
            s: *mut libc::c_char,
            max: libc::size_t,
            format: *const libc::c_char,
            tm: *const libc::tm,
        ) -> usize;
        pub(crate) fn time(tloc: *const libc::time_t) -> libc::time_t;
        pub(crate) fn localtime_r(t: *const libc::time_t, tm: *mut libc::tm);
        pub(crate) fn gmtime_r(t: *const libc::time_t, tm: *mut libc::tm);
    }
}

/// Get a tm struct in local timezone
pub fn get_local_tm_from_epoch(epoch: libc::time_t) -> libc::tm {
    let mut now = libc::tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: std::ptr::null(),
    };
    unsafe { c::localtime_r(&epoch, &mut now) };
    now
}

/// Get a tm struct in GMT
pub fn get_gmt_tm_from_epoch(epoch: libc::time_t) -> libc::tm {
    let mut now = libc::tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: std::ptr::null(),
    };
    unsafe { c::gmtime_r(&epoch, &mut now) };
    now
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
        c::tzset();
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
        assert_eq!(gmt, "Wed 07 Aug 2019 04:19:56 AM GMT");
        assert_eq!(local, "Wed 07 Aug 2019 06:19:56 AM CEST");

        env::set_var("LC_ALL", "fr_BE.UTF-8");
        env::set_var("TZ", "Europe/Brussels");

        tzset();
        set_locale();

        let gmt = strftime_gmt("%c", EPOCH);
        let local = strftime_local("%c", EPOCH);
        assert_eq!(gmt, "mer 07 aoû 2019 04:19:56 GMT");
        assert_eq!(local, "mer 07 aoû 2019 06:19:56 CEST");
    }
}
