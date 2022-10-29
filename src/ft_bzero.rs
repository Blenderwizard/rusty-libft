use crate::ft_memset::ft_memset;

pub fn ft_bzero(b: &mut [u8], len: usize) -> &mut [u8] {
	return ft_memset(b, 0, len);
}