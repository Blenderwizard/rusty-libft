use crate::ft_islower::ft_islower;
use crate::ft_isupper::ft_isupper;


pub fn ft_isalpha(c: char) -> bool {
	return ft_isupper(c) || ft_islower(c);
}