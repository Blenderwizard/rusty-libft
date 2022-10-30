pub fn ft_strmapi(s: &str, f: fn(usize, &str) -> &str) -> String {
	let mut ret = String::new();
	let data: Vec<_> = s.split("").collect();
	let mut i: usize = 0;
	for c in data {
		if c == "" {
			continue;
		}
		ret.push_str(f(i, c));
		i += 1;
	}
	return ret;
}