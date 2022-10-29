use std::{
    fs::File,
    io::{self, Write},
    os::unix::io::FromRawFd,
};

pub fn ft_putchar(c : char, fd: i32) -> io::Result<()> {
	let mut f = unsafe { File::from_raw_fd(3) };
    write!(&mut f, "Hello, world!")?;
    return Ok(());
}