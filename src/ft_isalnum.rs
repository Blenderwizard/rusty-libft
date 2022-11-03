use crate::ft_isdigit::ft_isdigit;
use crate::ft_isalpha::ft_isalpha;

/// Returns true if a char is an alphanumeric ascii character.
pub fn ft_isalnum(c: char) -> bool {
	return ft_isalpha(c) || ft_isdigit(c);
}