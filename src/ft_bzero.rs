use crate::ft_memset::ft_memset;


/// Takes an aray of bytes and sets each of the individual byte to zero.
/// 
/// This function in c is used for initializing ram, it doesn't have much use
/// inside of rust.
pub fn ft_bzero(b: &mut [u8], len: usize) -> &mut [u8] {
	return ft_memset(b, 0, len);
}