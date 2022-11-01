use crate::ft_isspace::ft_isspace;
use crate::ft_isdigit::ft_isdigit;
use std::num::Wrapping;

pub fn ft_atoi(s: &str) -> i32 {
	let mut begun: bool = false;
	let mut total: Wrapping<u64> = std::num::Wrapping(0);
	let mut sign: Wrapping<i32> = std::num::Wrapping(1);
	for c in s.chars() {
		if ft_isspace(c) && begun == false {
			continue;
		} 
		if (c == '+' || c == '-') && begun == false {
			if c == '-' {
				sign = std::num::Wrapping(-1);
			}
			begun = true;
			continue;
		}
		if ft_isdigit(c) {
			begun = true;
			total = (total * std::num::Wrapping(10)) + std::num::Wrapping::<u64>(c as u64) - std::num::Wrapping::<u64>(48);
			continue;
		}
		break;
	}
	if total > std::num::Wrapping(std::i64::MAX as u64) && (sign).0 > 0 {
		return -1;
	} else if total > std::num::Wrapping(std::i64::MAX as u64) && (sign).0 < 0  {
		return 0;
	}
	return (std::num::Wrapping::<i32>((total).0 as i32) * sign).0;
}