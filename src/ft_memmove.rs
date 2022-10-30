pub fn ft_memmove<'a>(b1: &'a mut [u8], b2: &[u8], len: usize) -> &'a mut [u8] {
	if b1 == b2 || len == 0 {
		return b1;
	}
	if (b2.as_ptr() as usize) < (b1.as_ptr() as usize) {
		let mut i: usize = b2.len() - 1;
		for item in b2.iter().rev() {
			b1[i] = *item;
			i += 1;
			if len == i {
				break;
			}
		}
	} else {
		let mut i: usize = 0;
		for item in b2.iter() {
			b1[i] = *item;
			i += 1;
			if len == i {
				break;
			}
		}
	}
	return b1;
}