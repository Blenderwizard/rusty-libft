pub fn ft_strnstr<'a>(s1: &'a str, s2: &'a str, len: usize) -> Option<&'a str>  {
	let found = s1.find(s2);
	if found.is_some() {
		let pos = found.unwrap();
		if pos >= len {
			return None;
		}
		let (_f1, f2) = s1.split_at(pos);
		return Some(f2);
	}
	return None;
}
