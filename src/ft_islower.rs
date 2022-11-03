/// Returns true if the character is a lowercase ascii character.
pub fn ft_islower(c: char) -> bool {
	return c as u8 >= 97 && (c as u8) <= 122;
}