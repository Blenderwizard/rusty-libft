/// Returns a slice where the first occurence of a byte is found, or None.
/// 
/// Iterates through a slice of bytes, up to length len or size of the buffer,
/// returning a slice containing the first occurence of the character to the end of
/// buffer or len, whichever is smaller. If no occurence exists, returns a None.
/// 
/// # Examples
/// ```
/// // Returns Some(&data[2..7])
/// use rusty_libft::ft_memchr::ft_memchr;
/// 
/// let binding = "Greetings".as_bytes().to_vec();
/// let data = binding.as_slice();
/// let result = ft_memchr(data, 'e' as u8, 7);
/// assert_eq!(result, Some(&data[2..7]));
/// ```
/// ```
/// // Returns Some(&data[2..9])
/// use rusty_libft::ft_memchr::ft_memchr;
/// 
/// let binding = "Greetings".as_bytes().to_vec();
/// let data = binding.as_slice();
/// let result = ft_memchr(data, 'e' as u8, 12);
/// assert_eq!(result, Some(&data[2..data.len()]));
/// ```
/// ```
/// // Returns None
/// use rusty_libft::ft_memchr::ft_memchr;
/// 
/// let binding = "Greetings".as_bytes().to_vec();
/// let data = binding.as_slice();
/// let result = ft_memchr(data, 'a' as u8, 12);
/// assert_eq!(result, None);
/// ```
pub fn ft_memchr(b: &[u8], c: u8, mut len: usize) -> Option<&[u8]> {
	let mut i = 0;
	let size = len;
	for bit in b.iter() {
		if *bit == c {
			if b.len() < size {
				return Some(&b[i..b.len()]);
			} else {
				return Some(&b[i..size]);
			}
		}
		i += 1;
		len -= 1;
		if len == 0 {
			break ;
		}
	}
	return None;
}
