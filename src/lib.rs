#![feature(error_in_core)]
#![no_std]
use core::fmt;
use spin::Mutex;
use syscalls::{syscall, Sysno};

pub mod exec;
pub mod io;

struct Writer {}
static WRITER: Mutex<Writer> = Mutex::new(Writer {});

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            let _ = syscall!(Sysno::write, 1, s.as_ptr(), s.len());
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    // NOTE: Locking needs to happen around `print_fmt`, not `print_str`, as the former
    // will call the latter potentially multiple times per invocation.
    let mut writer = WRITER.lock();
    fmt::Write::write_fmt(&mut *writer, args).ok();
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => { $crate::_print(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! println {
    ()          => { $crate::print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::print!("{}\n", format_args!($($t)*)); };
}
