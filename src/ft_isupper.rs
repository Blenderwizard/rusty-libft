/// Returns true if the character is a uppercase ascii character.
pub fn ft_isupper(c: char) -> bool {
	match c {
		'A' ..='Z' => true,
		_ => false,
	}
}