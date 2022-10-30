pub fn ft_memcpy<'a>(b1: &'a mut [u8], b2: &[u8], len: usize) -> &'a mut [u8] {
	let mut i: usize = 0;
	if len == 0 {
		return  b1;
	}
	for item in b2.iter() {
		b1[i] = *item;
		i += 1;
		if len == i {
			break;
		}
	}
	return b1;
}