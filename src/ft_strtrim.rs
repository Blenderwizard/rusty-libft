pub fn ft_strtrim(s: &str, set: &str) -> String {
	let bind: Vec<char> = set.chars().collect();
	let pat = bind.as_slice();
	format!("{}", s.trim_matches(pat))
}