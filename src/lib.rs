pub mod ft_atoi;
pub mod ft_bzero;
pub mod ft_isalnum;
pub mod ft_isalpha;
pub mod ft_isascii;
pub mod ft_isdigit;
pub mod ft_islower;
pub mod ft_isprint;
pub mod ft_isupper;
pub mod ft_memchr;
pub mod ft_memcmp;
pub mod ft_memcpy;
pub mod ft_memmove;
pub mod ft_memset;
pub mod ft_strchr;
pub mod ft_strlcat;
pub mod ft_strlcpy;
pub mod ft_strlen;
pub mod ft_strncmp;
pub mod ft_strnstr;
pub mod ft_strrchr;
pub mod ft_tolower;
pub mod ft_toupper;
// use ft_atoi::ft_atoi;
// use ft_bzero::ft_bzero;
// use ft_isalnum::ft_isalnum;
// use ft_isalpha::ft_isalpha;
// use ft_isascii::ft_isascii;
// use ft_isdigit::ft_isdigit;
// use ft_islower::ft_islower;
// use ft_isprint::ft_isprint;
// use ft_isupper::ft_isupper;
// use ft_memchr::ft_memchr;
// use ft_memcmp::ft_memcmp;
// use ft_memcpy::ft_memcpy;
// use ft_memmove::ft_memmove;
// use ft_strchr::ft_strchr;
// use ft_strlcat::ft_strlcat;
// use ft_strlcpy::ft_strlcpy;
// use ft_strncmp::ft_strncmp;
// use ft_strnstr::ft_strnstr;
// use ft_strrchr::ft_strrchr;

#[cfg(test)]
mod mem_functs {
	use crate::ft_memcmp::ft_memcmp;
	use crate::ft_memset::ft_memset;
	
	#[test]
	fn easy_ft_memcmp() {
		let cmp1 = "Greetings".as_bytes().to_vec();
		let cmp2 = "Greetings".as_bytes().to_vec();
		let result = ft_memcmp(&cmp1, &cmp2, 9);
		assert_eq!(result, 0);
		let cmp1 = "Greetings".as_bytes().to_vec();
		let cmp2 = "Greeting".as_bytes().to_vec();
		let result = ft_memcmp(&cmp1, &cmp2, 9);
		assert_eq!(result, 115);
		let cmp1 = "Greetings".as_bytes().to_vec();
		let cmp2 = "Greeting".as_bytes().to_vec();
		let result = ft_memcmp(&cmp1, &cmp2, 8);
		assert_eq!(result, 0);
	}

	#[test]
	fn easy_ft_memset() {
		let mut test = vec!['c' as u8, 'a' as u8, 't' as u8];
		let mut leng = test.len();
		let mut result = ft_memset(&mut test, 'o' as u8, leng);
		let mut expected = vec!['o' as u8, 'o' as u8, 'o' as u8];
		assert_eq!(result, &expected);
		assert_eq!(&test, &expected);
		test = vec!['c' as u8, 'a' as u8, 't' as u8];
		result = ft_memset(&mut test, 'o' as u8, 1);
		expected = vec!['o' as u8, 'a' as u8, 't' as u8];
		assert_eq!(result, &expected);
		assert_eq!(&test, &expected);
		test = "Hello this is a very long string that I wrote for a test".as_bytes().to_vec();
		leng = test.len();
		result = ft_memset(&mut test, 'o' as u8, leng);
		expected = "oooooooooooooooooooooooooooooooooooooooooooooooooooooooo".as_bytes().to_vec();
		assert_eq!(result, &expected);
		assert_eq!(&test, &expected);
	}
}

#[cfg(test)]
mod str_functs {
	use crate::ft_strlen::ft_strlen;

	#[test]
	fn ascii_ft_strlen() {
		let mut a = "This is a test";
		let mut result = ft_strlen(a.to_string());
		assert_eq!(result, a.len());
		a = "";
		result = ft_strlen(a.to_string());
		assert_eq!(result, a.len());
		a = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
		result = ft_strlen(a.to_string());
		assert_eq!(result, a.len());
	}

	#[test]
	fn unicode_ft_strlen() {
		let mut a = "邏 樂 洛 烙 珞 落";
		let mut result = ft_strlen(a.to_string());
		assert_eq!(result, a.len());
		a = "A✄ ✆ E✇ ✈ D 0";
		result = ft_strlen(a.to_string());
		assert_eq!(result, a.len());
		a = "ણતથદધનપફબଉଊଋଌଏଐଓଔକ ୁ ୃ ୈ ୌ ୖ ୗஊஎஏஐஒஓ௯௰௱௲ఆఇఈఉఊహ ా ి ె ే ై ొ ో ౌ ్ౡഹ ാ ി ീ ു ൂ ൃ െ േะ ั า ี ึ ื ็ ้ ๊ ๋ ์ ํ ๎༼ ༽ႽႾႿჀḂḃḄḔḕḖḳḴḵṄṅṠṡṿἇἈἑἤἮἴἹ";
		result = ft_strlen(a.to_string());
		assert_eq!(result, a.len());
	}
}

#[cfg(test)]
mod to_functs {
	use crate::ft_tolower::ft_tolower;
	use crate::ft_toupper::ft_toupper;

	#[test]
	fn alphabet_ft_toupper() {
		let a = "abcdefghijklmnopqrstuvwxyz".to_string();
		for i in a.chars() {
			let result = ft_toupper(i);
			assert_eq!(result, i.to_ascii_uppercase());
		}
	}

	#[test]
	fn symbols_ft_toupper() {
		let a = "!@#$%^&*()123456789".to_string();
		for i in a.chars() {
			let result = ft_toupper(i);
			assert_eq!(result, i.to_ascii_uppercase());
		}
	}

	#[test]
	fn alphabet_ft_tolower() {
		let a = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
		for i in a.chars() {
			let result = ft_tolower(i);
			assert_eq!(result, i.to_ascii_lowercase());
		}
	}

	#[test]
	fn symbols_ft_tolower() {
		let a = "!@#$%^&*()123456789".to_string();
		for i in a.chars() {
			let result = ft_tolower(i);
			assert_eq!(result, i.to_ascii_lowercase());
		}
	}
}
