use crate::ft_isspace::ft_isspace;
use crate::ft_isdigit::ft_isdigit;
use std::num::Wrapping;

/// Returns a signed 32 bit integer from an ascii string
/// 
/// A reimplentation of the atoi function in c, takes a string as a parameter
/// and returns the signed 32 bit integer representation. Will ignore any ascii
/// whitespace at the begining of the string, a sign can then be specified with
/// a + or - folowed by numbers until a non-number character is encountered.
/// 
/// Atoi will overflow or underflow numbers, if the parameter is greater than
/// std::i64::MAX or std::i64::MIN, then the returned value will be -1 and 0 
/// respectively.
/// 
/// # Examples
/// ```
/// // Returns 1
/// use rusty_libft::ft_atoi::ft_atoi;
/// 
/// let ret = ft_atoi("1");	
/// assert_eq!(ret, 1);
/// ```
/// ```
/// // Returns 0
/// use rusty_libft::ft_atoi::ft_atoi;
/// 
/// let ret = ft_atoi("a1");	
/// assert_eq!(ret, 0);
/// ```
/// ```
/// // Returns 1
/// use rusty_libft::ft_atoi::ft_atoi;
/// 
/// let ret = ft_atoi("    1a");	
/// assert_eq!(ret, 1);
/// ```
/// ```
/// // Returns -1
/// use rusty_libft::ft_atoi::ft_atoi;
/// 
/// let ret = ft_atoi("-1");	
/// assert_eq!(ret, -1);
/// ```
/// ```
/// // Returns -1
/// use rusty_libft::ft_atoi::ft_atoi;
/// 
/// let ret = ft_atoi("4294967295");	
/// assert_eq!(ret, -1);
/// ```
/// ```
/// // Returns 0
/// use rusty_libft::ft_atoi::ft_atoi;
/// 
/// let ret = ft_atoi("-4294967296");	
/// assert_eq!(ret, 0);
/// ```
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