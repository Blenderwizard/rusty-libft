pub fn ft_isupper(c: char) -> bool {
	match c {
		'A' ..='Z' => true,
		_ => false,
	}
}