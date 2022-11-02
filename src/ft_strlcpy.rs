use crate::ft_memcpy::ft_memcpy;
use crate::ft_strlen::ft_strlen;

pub fn ft_strlcpy(dst: &mut str, src: &str, size: usize) -> usize {
	let src_len = ft_strlen(src);

	if src_len < size {
		unsafe { ft_memcpy(dst.as_bytes_mut(), src.as_bytes(), src_len) };
	} else if size != 0 {
		unsafe { ft_memcpy(dst.as_bytes_mut(), src.as_bytes(), size - 1) };
	}
	return src_len;
}
