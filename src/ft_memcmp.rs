pub fn ft_memcmp(b1: & Vec<u8>, b2: & Vec<u8>, mut len: usize) -> i32 {
	if b1.len() > b2.len() && b2.len() < len {
		return b1[b2.len()].into()
	} else if b1.len() < b2.len() && b1.len() < len {
		return b2[b1.len()].into()
	}
	for (b1it, b2it) in b1.iter().zip(b2.iter()) {
		if *b1it != *b2it {
			return (*b1it - *b2it).into();
		}
		len -= 1;
		if len == 0 {
			break ;
		}
	}
	return 0;
}