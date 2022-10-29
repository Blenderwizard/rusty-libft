use crate::ft_memchr::ft_memchr;

pub fn ft_strchr(s: &str, c: u8) -> Option<&str> {
	let res = ft_memchr(s.as_bytes(), c, s.len());

	match res {
        Some(inner)	=> Some(std::str::from_utf8(&inner).unwrap()),
        None		=> return None,
    }
}
