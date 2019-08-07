[![Build Status](https://travis-ci.org/cecton/libc-strftime.svg?branch=master)](https://travis-ci.org/cecton/libc-strftime)
[![Latest Version](https://img.shields.io/crates/v/libc-strftime.svg)](https://crates.io/crates/libc-strftime)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](http://opensource.org/licenses/MIT)
[![Docs.rs](https://docs.rs/libc-strftime/badge.svg)](https://docs.rs/libc-strftime)
[![LOC](https://tokei.rs/b1/github/cecton/libc-strftime)](https://github.com/cecton/libc-strftime)
[![Dependency Status](https://deps.rs/repo/github/cecton/libc-strftime/status.svg)](https://deps.rs/repo/github/cecton/libc-strftime)

libc-strftime
=============

A wrapper library for the glibc strftime function

Examples
--------

Format the current date and time in Brussels in French:

```
use std::env;

env::set_var("LC_ALL", "fr_BE.UTF-8");
env::set_var("TZ", "Europe/Brussels");

libc_strftime::tzset();
libc_strftime::set_locale();

let now = libc_strftime::epoch(); // most likely a u64
let local = libc_strftime::strftime_local("%c", now);
println!("On est: {}", local);
```

Known Issues
------------
 *  The translation doesn't seen to work with MUSL.
