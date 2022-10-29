use crate::ft_memcmp::ft_memcmp;

pub fn ft_strncmp(s1: String, s2: String, size: usize) -> i32 {
	return ft_memcmp(s1, s2, size);
}