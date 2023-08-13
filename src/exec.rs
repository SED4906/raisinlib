use syscalls::{syscall, Sysno};

pub fn execute(file: &[u8], args: &[*const u8]) -> Result<usize, syscalls::Errno>{
    unsafe{syscall!(Sysno::execve,file.as_ptr(),args.as_ptr(),0)}
}

pub fn spawn(buf: &[u8], args: &[*const u8]) -> Result<usize, syscalls::Errno> {
    let pid = unsafe{syscall!(Sysno::fork)}?;
    if pid == 0 {
        execute(&buf, &args)
    } else {
        Ok(pid)
    }
}