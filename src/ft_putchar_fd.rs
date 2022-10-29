use std::{
    fs::File,
    io::{self, Write},
    os::unix::io::FromRawFd,
};

pub fn ft_putchar_fd(c : char, fd: i32) -> io::Result<()> {
	let mut f = unsafe { File::from_raw_fd(fd) };
    write!(&mut f, "{}", c)?;
    return Ok(());
}