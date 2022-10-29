use std::{
    fs::File,
    io::{self, Write},
    os::unix::io::FromRawFd,
};

pub fn ft_putstr_fd(s : &str, fd: i32) -> io::Result<()> {
	let mut f = unsafe { File::from_raw_fd(fd) };
    write!(&mut f, "{}", s)?;
    return Ok(());
}