use crate::ft_isspace::ft_isspace;
use crate::ft_isdigit::ft_isdigit;

pub fn ft_atoi(s: &str) -> i32 {
	let mut begun: bool = false;
	let mut total: u64 = 0;
	let mut sign: i8 = 1;
	for c in s.chars() {
		if ft_isspace(c) && !begun {
			continue;
		} 
		if c == '+' || c == '-' && !begun {
			if c == '-' {
				sign = -1;
			}
			begun = true;
			continue;
		}
		if ft_isdigit(c) {
			begun = true;
			total = (total * 10) + c.to_digit(10).unwrap() as u64;
		}
	}
	if total > std::i64::MAX as u64 && sign > 0 {
		return -1;
	} else if total > std::i64::MAX as u64 && sign < 0 {
		return 0;
	}
	return (total * sign as u64) as i32;
}