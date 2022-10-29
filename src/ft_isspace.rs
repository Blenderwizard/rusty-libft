pub fn ft_isspace(c: char) -> bool {
	return ((c as u8) >= 9 && (c as u8) <= 13) || (c as u8) == 32;
}