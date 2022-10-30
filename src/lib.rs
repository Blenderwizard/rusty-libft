pub mod ft_atoi;
pub mod ft_bzero;
pub mod ft_isalnum;
pub mod ft_isalpha;
pub mod ft_isascii;
pub mod ft_isdigit;
pub mod ft_islower;
pub mod ft_isprint;
pub mod ft_isspace;
pub mod ft_isupper;
pub mod ft_itoa;
pub mod ft_memchr;
pub mod ft_memcmp;
pub mod ft_memcpy;
pub mod ft_memmove;
pub mod ft_memset;
pub mod ft_putchar_file;
pub mod ft_putchar;
pub mod ft_putendl_file;
pub mod ft_putendl;
pub mod ft_putnbr_file;
pub mod ft_putnbr;
pub mod ft_putstr_file;
pub mod ft_putstr;
pub mod ft_split;
pub mod ft_strchr;
pub mod ft_striteri;
pub mod ft_strjoin;
pub mod ft_strlcat;
pub mod ft_strlcpy;
pub mod ft_strlen;
pub mod ft_strmapi;
pub mod ft_strncmp;
pub mod ft_strnstr;
pub mod ft_strrchr;
pub mod ft_strtrim;
pub mod ft_substr;
pub mod ft_tolower;
pub mod ft_toupper;

#[cfg(test)]
mod atoi_funct {}

#[cfg(test)]
mod is_functs {
	use crate::ft_isalnum::ft_isalnum;
	use crate::ft_isalpha::ft_isalpha;
	use crate::ft_isascii::ft_isascii;
	use crate::ft_isdigit::ft_isdigit;
	use crate::ft_islower::ft_islower;
	use crate::ft_isprint::ft_isprint;
	use crate::ft_isspace::ft_isspace;
	use crate::ft_isupper::ft_isupper;
	
	#[test]
	fn test_ft_isalnum() {
		let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456780!@#$%^&*(),.<>/?;':\"\\[]{}-=_+ \n\t\r`~";
		for i in a.chars() {
			let result = ft_isalnum(i);
			assert_eq!(result, i.is_ascii_alphanumeric());
		}
	}

	#[test]
	fn test_ft_isalpha() {
		let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456780!@#$%^&*(),.<>/?;':\"\\[]{}-=_+ \n\t\r`~";
		for i in a.chars() {
			let result = ft_isalpha(i);
			assert_eq!(result, i.is_ascii_alphabetic());
		}
	}

	#[test]
	fn test_ft_isascii() {
		let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456780!@#$%^&*(),.<>/?;':\"\\[]{}-=_+ \n\t\r`~";
		for i in a.chars() {
			let result = ft_isascii(i);
			assert_eq!(result, i.is_ascii());
		}
	}

	#[test]
	fn test_ft_isdigit() {
		let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456780!@#$%^&*(),.<>/?;':\"\\[]{}-=_+ \n\t\r`~";
		for i in a.chars() {
			let result = ft_isdigit(i);
			assert_eq!(result, i.is_ascii_digit());
		}
	}

	#[test]
	fn test_ft_islower() {
		let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456780!@#$%^&*(),.<>/?;':\"\\[]{}-=_+ \n\t\r`~";
		for i in a.chars() {
			let result = ft_islower(i);
			assert_eq!(result, i.is_ascii_lowercase());
		}
	}

	#[test]
	fn test_ft_isprint() {
		let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456780!@#$%^&*(),.<>/?;':\"\\[]{}-=_+ \n\t\r`~";
		for i in a.chars() {
			let result = ft_isprint(i);
			assert_eq!(result, i.is_ascii_graphic() || i == ' ');
		}
	}

	#[test]
	fn test_ft_isspace() {
		let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456780!@#$%^&*(),.<>/?;':\"\\[]{}-=_+ \n\t\r`~";
		for i in a.chars() {
			let result = ft_isspace(i);
			assert_eq!(result, i.is_ascii_whitespace());
		}
	}

	#[test]
	fn test_ft_isupper() {
		let a = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456780!@#$%^&*(),.<>/?;':\"\\[]{}-=_+ \n\t\r`~";
		for i in a.chars() {
			let result = ft_isupper(i);
			assert_eq!(result, i.is_ascii_uppercase());
		}
	}
}

#[cfg(test)]
mod itoa_funct {
	use crate::ft_atoi::ft_atoi;
	// /* 1 */ check(ft_atoi((e + "1").c_str()) == 1); showLeaks();
	// /* 2 */ check(ft_atoi((e + "a1").c_str()) == 0); showLeaks();
	// /* 3 */ check(ft_atoi((e + "--1").c_str()) == 0); showLeaks();
	// /* 4 */ check(ft_atoi((e + "++1").c_str()) == 0); showLeaks();
	// /* 5 */ check(ft_atoi((e + "+1").c_str()) == 1); showLeaks();
	// /* 6 */ check(ft_atoi((e + "-1").c_str()) == -1); showLeaks();
	// /* 7 */ check(ft_atoi((e + "0").c_str()) == 0); showLeaks();
	// /* 8 */ check(ft_atoi((e + "+42lyon").c_str()) == 42); showLeaks();
	// /* 9 */ check(ft_atoi((e + "+101").c_str()) == 101); showLeaks();
	// /* 10 */ check(ft_atoi((e + to_string(INT_MAX)).c_str()) == INT_MAX); showLeaks();
	// /* 11 */ check(ft_atoi((e + to_string(INT_MIN)).c_str()) == INT_MIN); showLeaks();
	// /* 12 */ check(ft_atoi("-+42") == 0); showLeaks();
	// /* 13 */ check(ft_atoi("+-42") == 0); showLeaks();
	// /* 14 */ check(ft_atoi((string("+") + e + "42").c_str()) == 0); showLeaks();
	// /* 15 */ check(ft_atoi((string("-") + e + "42").c_str()) == 0); showLeaks();
	// /* 16 */ check(ft_atoi((string("1") + e + "42").c_str()) == 1); showLeaks();
	// /* 17 */ check(ft_atoi((string("-1") + e + "42").c_str()) == -1); showLeaks();
	
	#[test]
	fn test_ft_atoi() {
		{
			let ret = ft_atoi("1");
			assert_eq!(ret, 1);
		}
		{
			let ret = ft_atoi("a1");
			assert_eq!(ret, 0);
		}
		{
			let ret = ft_atoi("--1");
			assert_eq!(ret, 0);
		}
		{
			let ret = ft_atoi("++1");
			assert_eq!(ret, 0);
		}
		{
			let ret = ft_atoi("-1");
			assert_eq!(ret, -1);
		}
		{
			let ret = ft_atoi("+1");
			assert_eq!(ret, 1);
		}
		{
			let ret = ft_atoi("0");
			assert_eq!(ret, 0);
		}
		{
			let ret = ft_atoi("+101rust");
			assert_eq!(ret, 101);
		}
		{
			let ret = ft_atoi("+-101rust");
			assert_eq!(ret, 0);
		}
		{
			let ret = ft_atoi("-+101rust");
			assert_eq!(ret, 0);
		}
		{
			let ret = ft_atoi("               100");
			assert_eq!(ret, 100);
		}
		{
			let ret = ft_atoi("               -123");
			assert_eq!(ret, -123);
		}
		{
			let ret = ft_atoi("               345");
			assert_eq!(ret, 345);
		}
	}
}

#[cfg(test)]
mod put_functs {}

#[cfg(test)]
mod mem_functs {
	use crate::ft_memcmp::ft_memcmp;
	use crate::ft_memset::ft_memset;
	
	#[test]
	fn easy_ft_memcmp() {
		let binding1 = "Greetings".as_bytes().to_vec();
		let cmp1 = binding1.as_slice();
		let binding2 = "Greetings".as_bytes().to_vec();
		let cmp2 = binding2.as_slice();
		let result = ft_memcmp(&cmp1, &cmp2, 9);
		assert_eq!(result, 0);
		
		let binding1 = "Greetings".as_bytes().to_vec();
		let cmp1 = binding1.as_slice();
		let binding2 = "Greeting".as_bytes().to_vec();
		let cmp2 = binding2.as_slice();
		let result = ft_memcmp(&cmp1, &cmp2, 9);
		assert_eq!(result, 115);

	
		let binding1 = "Greetings".as_bytes().to_vec();
		let cmp1 = binding1.as_slice();
		let binding2 = "Greeting".as_bytes().to_vec();
		let cmp2 = binding2.as_slice();
		let result = ft_memcmp(&cmp1, &cmp2, 8);
		assert_eq!(result, 0);
	}

	#[test]
	fn easy_ft_memset() {
		let mut test: [u8; 3] = ['c' as u8, 'a' as u8, 't' as u8];
		let leng = test.len();
		let result = ft_memset(&mut test, 'o' as u8, leng);
		let expected: [u8; 3] = ['o' as u8, 'o' as u8, 'o' as u8];
		assert_eq!(result, &expected);
		assert_eq!(&test, &expected);

		let mut test = vec!['c' as u8, 'a' as u8, 't' as u8];
		let result = ft_memset(&mut test, 'o' as u8, 1);
		let expected = vec!['o' as u8, 'a' as u8, 't' as u8];
		assert_eq!(result, &expected);
		assert_eq!(&test, &expected);

		let mut test = "Hello this is a very long string that I wrote for a test".as_bytes().to_vec();
		let leng = test.len();
		let result = ft_memset(&mut test, 'o' as u8, leng);
		let expected = "oooooooooooooooooooooooooooooooooooooooooooooooooooooooo".as_bytes().to_vec();
		assert_eq!(result, &expected);
		assert_eq!(&test, &expected);
	}
}

#[cfg(test)]
mod split_funct {}

#[cfg(test)]
mod str_functs {
	use crate::ft_strlen::ft_strlen;

	#[test]
	fn ascii_ft_strlen() {
		let a = "This is a test";
		let result = ft_strlen(a);
		assert_eq!(result, a.len());

		let a = "";
		let result = ft_strlen(&a);
		assert_eq!(result, a.len());

		let a = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
		let result = ft_strlen(&a);
		assert_eq!(result, a.len());
	}

	#[test]
	fn unicode_ft_strlen() {
		let a = "邏 樂 洛 烙 珞 落";
		let result = ft_strlen(&a);
		assert_eq!(result, a.len());

		let a = "A✄ ✆ E✇ ✈ D 0";
		let result = ft_strlen(&a);
		assert_eq!(result, a.len());

		let a = "ણતથદધનપફબଉଊଋଌଏଐଓଔକ ୁ ୃ ୈ ୌ ୖ ୗஊஎஏஐஒஓ௯௰௱௲ఆఇఈఉఊహ ా ి ె ే ై ొ ో ౌ ్ౡഹ ാ ി ീ ു ൂ ൃ െ േะ ั า ี ึ ื ็ ้ ๊ ๋ ์ ํ ๎༼ ༽ႽႾႿჀḂḃḄḔḕḖḳḴḵṄṅṠṡṿἇἈἑἤἮἴἹ";
		let result = ft_strlen(&a);
		assert_eq!(result, a.len());
	}
}

#[cfg(test)]
mod to_functs {
	use crate::ft_tolower::ft_tolower;
	use crate::ft_toupper::ft_toupper;

	#[test]
	fn alphabet_ft_toupper() {
		let a = "abcdefghijklmnopqrstuvwxyz";
		for i in a.chars() {
			let result = ft_toupper(i);
			assert_eq!(result, i.to_ascii_uppercase());
		}
	}

	#[test]
	fn symbols_ft_toupper() {
		let a = "!@#$%^&*()123456789";
		for i in a.chars() {
			let result = ft_toupper(i);
			assert_eq!(result, i.to_ascii_uppercase());
		}
	}

	#[test]
	fn alphabet_ft_tolower() {
		let a = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
		for i in a.chars() {
			let result = ft_tolower(i);
			assert_eq!(result, i.to_ascii_lowercase());
		}
	}

	#[test]
	fn symbols_ft_tolower() {
		let a = "!@#$%^&*()123456789";
		for i in a.chars() {
			let result = ft_tolower(i);
			assert_eq!(result, i.to_ascii_lowercase());
		}
	}
}
