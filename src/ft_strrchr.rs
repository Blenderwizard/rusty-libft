pub fn ft_strrchr(s: &str, c: char) -> Option<&str> {
	let mut i = s.len();
	for sit in s.chars().rev() {
		if sit == c {
			return Some(&s[i-1..s.len()]);
		}
		i -= 1;
	}
	return None;
}
