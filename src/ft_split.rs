/// Spilts a str into a Vec of Strings using a specified Delimiter.
pub fn ft_split(s: &str, c: char) -> Vec<String> {
	let items: Vec<&str> = s.split(c).collect();
	let mut ret = Vec::<String>::new();
	for item in items {
		if item != "" {
			ret.push(item.to_string());
		}
	}
	return ret;
}