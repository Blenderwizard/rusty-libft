pub fn ft_memchr(b: &[u8], c: u8, mut len: usize) -> Option<&[u8]> {
	let mut i = 0;
	for bit in b.iter() {
		if *bit == c {
			return Some(&b[i..b.len()]);
		}
		i += 1;
		len -= 1;
		if len == 0 {
			break ;
		}
	}
	return None;
}
