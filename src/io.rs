use core::fmt::{Display, Write};
use syscalls::{syscall, Sysno};

#[derive(Debug)]
pub struct Error();

impl Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

impl core::error::Error for Error {}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
}

pub struct File(usize);

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if let Ok(num_read) = unsafe { syscall!(Sysno::read, self.0, buf.as_mut_ptr(), buf.len()) }
        {
            Ok(num_read)
        } else {
            Err(Error())
        }
    }
}

impl File {
    pub fn open(name: &str) -> Result<Self, Error> {
        if let Ok(fd) = unsafe { syscall!(Sysno::open, name.as_ptr(), 0) } {
            Ok(Self(fd))
        } else {
            Err(Error())
        }
    }
    pub fn close(&self) {
        unsafe {
            let _ = syscall!(Sysno::close, self.0);
        }
    }
}

pub struct Utf8String<'a>(pub &'a [u8]);

impl core::fmt::Display for Utf8String<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for c in self.0 {
            if let Some(c) = char::from_u32(*c as u32) {
                let _ = f.write_char(c);
            }
        }
        Ok(())
    }
}
