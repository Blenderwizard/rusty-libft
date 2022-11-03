/// Returns true if the character is an ascii whitespace.
/// 
/// An ascii whitespace is a Space " ", a Tab "\t", a New Line, "\n", a Carage Return
/// "\r", a Horizontal Tab "\v" or a Form Feed "\f".
pub fn ft_isspace(c: char) -> bool {
	return ((c as u8) >= 9 && (c as u8) <= 13) || (c as u8) == 32;
}