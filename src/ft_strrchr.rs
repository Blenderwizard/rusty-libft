pub fn ft_memchr(s: &str, c: u8) -> Option<&str> {
	let mut i = 0;
	for sit in s.chars().rev() {
		if sit as u8 == c {
			return Some(&s[i..s.len()]);
		}
		i += 1;
	}
	return None;
}
