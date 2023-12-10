#[cfg(not(feature = "p2"))]
fn extract_calibration_value(s: &str) -> u8 {
	let first_digit = s.as_bytes()[s.find(|c: char| c.is_ascii_digit()).unwrap()];
	let last_digit = s.as_bytes()[s.rfind(|c: char| c.is_ascii_digit()).unwrap()];

	(first_digit - b'0') * 10 + (last_digit - b'0')
}

#[cfg(feature = "p2")]
fn extract_calibration_value(s: &str) -> u8 {
	use regex::Regex;
	use std::sync::OnceLock;

	static RE: OnceLock<Regex> = OnceLock::new();

	let re =
		RE.get_or_init(|| Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap());
	let digit_to_u8 = |digit| match digit {
		"0" => 0,
		"1" | "one" => 1,
		"2" | "two" => 2,
		"3" | "three" => 3,
		"4" | "four" => 4,
		"5" | "five" => 5,
		"6" | "six" => 6,
		"7" | "seven" => 7,
		"8" | "eight" => 8,
		"9" | "nine" => 9,
		_ => unreachable!(),
	};

	let mut first_digit = None;
	let mut last_digit = None;
	for start in 0..s.len() {
		if let Some(m) = re.find_at(s, start) {
			if first_digit.is_none() {
				first_digit = Some(m.as_str());
			}
			last_digit = Some(m.as_str());
		}
	}

	digit_to_u8(first_digit.unwrap()) * 10 + digit_to_u8(last_digit.unwrap())
}

fn main() {
	let sum: usize = std::io::stdin()
		.lines()
		.map(|res| extract_calibration_value(&res.unwrap()) as usize)
		.sum();
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	#[test]
	fn extract_calibration_values_p1() {
		assert_eq!(super::extract_calibration_value("1abc2"), 12);
		assert_eq!(super::extract_calibration_value("pqr3stu8vwx"), 38);
		assert_eq!(super::extract_calibration_value("a1b2c3d4e5f"), 15);
		assert_eq!(super::extract_calibration_value("treb7uchet"), 77);
	}

	#[cfg(feature = "p2")]
	#[test]
	fn extract_calibration_values_p2() {
		assert_eq!(super::extract_calibration_value("two1nine"), 29);
		assert_eq!(super::extract_calibration_value("eightwothree"), 83);
		assert_eq!(super::extract_calibration_value("abcone2threexyz"), 13);
		assert_eq!(super::extract_calibration_value("xtwone3four"), 24);
		assert_eq!(super::extract_calibration_value("4nineeightseven2"), 42);
		assert_eq!(super::extract_calibration_value("zoneight234"), 14);
		assert_eq!(super::extract_calibration_value("7pqrstsixteen"), 76);
	}
}

#[cfg(all(test, stars))]
mod stars {
	#[test]
	fn p1() {
		assert!(true);
	}

	#[test]
	fn p2() {
		assert!(true);
	}
}
