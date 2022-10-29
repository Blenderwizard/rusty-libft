use crate::ft_memcmp::ft_memcmp;

pub fn ft_strncmp(s1: &str, s2: &str, size: usize) -> i32 {
	return ft_memcmp(s1.as_bytes(), s2.as_bytes(), size);
}