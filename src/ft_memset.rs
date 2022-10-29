pub fn ft_memset(b: &mut [u8], c: u8, mut len: usize) -> &mut [u8] {
	for d in b.iter_mut() {
		*d = c;
		len -= 1;
		if len == 0 {
			break;
		}
	}
	return b;
}