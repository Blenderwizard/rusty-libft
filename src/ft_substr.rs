use crate::ft_strlen::ft_strlen;

pub fn ft_substr(s: &str, start: usize, mut len: usize) -> Option<String> {
	if start > ft_strlen(s) {
		return None;
	}
	if len == 0 {
		return None;
	}
	let mut temp = String::new();
	let (_f1, f2) = s.split_at(start);
	for c in f2.chars() {
		temp.push(c);
		len -= 1;
		if len == 0 {
			return Some(temp);
		}
	}
	Some(temp)
}
