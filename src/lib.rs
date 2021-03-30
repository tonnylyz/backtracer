//! A library for acquiring a backtrace at runtime
#![no_std]

extern crate addr2line;
extern crate alloc;
extern crate cfg_if;
extern crate log;

#[cfg(target_arch = "aarch64")]
extern crate cortex_a;

pub use backtrace::{trace, trace_from, EntryPoint, Frame};
mod backtrace;

pub use symbolize::{resolve, Symbol};
mod symbolize;

#[allow(dead_code)]
struct Bomb {
    enabled: bool,
}

#[allow(dead_code)]
impl Drop for Bomb {
    fn drop(&mut self) {
        if self.enabled {
            panic!("cannot panic during the backtrace function");
        }
    }
}
