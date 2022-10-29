use crate::ft_memset::ft_memset;

pub fn ft_bzero(b: &mut Vec<u8>, len: usize) -> &mut Vec<u8> {
	return ft_memset(b, 0, len);
}