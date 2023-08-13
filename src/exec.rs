use syscalls::{syscall, Sysno};

pub fn execute(file: &[u8], args: &[*const u8]) -> Result<usize, syscalls::Errno>{
    unsafe{syscall!(Sysno::execve,file.as_ptr(),args.as_ptr(),0)}
}