use crate::ft_memchr::ft_memchr;

/// Returns a slice where the first occurence of a char is found, or None.
/// 
/// Iterates through a str, up to length len or size of the str, returning a 
/// slice containing the first occurence of the character to the end of buffer 
/// or len, whichever is smaller. If no occurence exists, returns a None.
/// 
/// # Examples
/// ```
/// // Returns Some("eetings")
/// use rusty_libft::ft_strchr::ft_strchr;
/// 
/// let data = "Greetings";
/// let result = ft_strchr(data, 'e');
/// assert_eq!(result, Some("eetings"));
/// ```
/// ```
/// // Returns None
/// use rusty_libft::ft_strchr::ft_strchr;
/// 
/// let data = "Greetings";
/// let result = ft_strchr(data, 'a');
/// assert_eq!(result, None);
/// ```
/// ```
/// // Returns Some("Greetings")
/// use rusty_libft::ft_strchr::ft_strchr;
/// 
/// let data = "Greetings";
/// let result = ft_strchr(data, 'G');
/// assert_eq!(result, Some("Greetings"));
/// ```
pub fn ft_strchr(s: &str, c: char) -> Option<&str> {
	let res = ft_memchr(s.as_bytes(), c as u8, s.len());

	match res {
        Some(inner)	=> Some(std::str::from_utf8(&inner).unwrap()),
        None		=> return None,
    }
}
