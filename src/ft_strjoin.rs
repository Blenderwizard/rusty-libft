/// Joins 2 strs together in a new allocated String.
pub fn ft_strjoin(str1: &str, str2: &str) -> String { 
	format!("{}{}", str1, str2)
}