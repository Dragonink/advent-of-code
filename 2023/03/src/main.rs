use regex::Regex;
use std::{collections::HashMap, io::Read};

fn list_numbers(s: &str) -> HashMap<(usize, usize), usize> {
	let re = Regex::new(r"\d+").unwrap();

	s.lines()
		.enumerate()
		.flat_map(|(l, s)| {
			re.find_iter(s).flat_map(move |m| {
				let value: usize = m.as_str().parse().unwrap();
				m.range().map(move |c| ((l, c), value))
			})
		})
		.collect()
}

fn main() {
	let mut input = String::new();
	std::io::stdin().lock().read_to_string(&mut input).unwrap();
	let numbers = list_numbers(&input);

	#[cfg(not(feature = "p2"))]
	let filter_map = |values: Vec<usize>| Some(values);
	#[cfg(feature = "p2")]
	let filter_map = |values: Vec<usize>| (values.len() == 2).then(|| [values[0] * values[1]]);

	let sum: usize = input
		.lines()
		.enumerate()
		.flat_map(|(l, s)| {
			s.bytes()
				.enumerate()
				.filter(|(_, b)| !(*b as char).is_ascii_digit() && *b != b'.')
				.map(|(c, _)| {
					let mut values = Vec::new();
					let try_push = |values: &mut Vec<usize>, l, c| {
						if let Some(val) = numbers.get(&(l, c)) {
							values.push(*val);
						}
					};
					let try_push_other_line = |values: &mut Vec<usize>, l, c| {
						if let Some(val) = numbers.get(&(l, c)) {
							values.push(*val);
						} else {
							try_push(values, l, c.saturating_sub(1));
							try_push(values, l, c.saturating_add(1));
						}
					};

					try_push(&mut values, l, c.saturating_sub(1));
					try_push(&mut values, l, c.saturating_add(1));
					try_push_other_line(&mut values, l.saturating_sub(1), c);
					try_push_other_line(&mut values, l.saturating_add(1), c);

					values
				})
				.filter_map(filter_map)
				.flatten()
				.collect::<Vec<_>>()
		})
		.sum();
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn list_numbers() {
		assert_eq!(
			super::list_numbers(include_str!("test.txt")),
			HashMap::from([
				((0, 0), 467),
				((0, 1), 467),
				((0, 2), 467),
				((0, 5), 114),
				((0, 6), 114),
				((0, 7), 114),
				((2, 2), 35),
				((2, 3), 35),
				((2, 6), 633),
				((2, 7), 633),
				((2, 8), 633),
				((4, 0), 617),
				((4, 1), 617),
				((4, 2), 617),
				((5, 7), 58),
				((5, 8), 58),
				((6, 2), 592),
				((6, 3), 592),
				((6, 4), 592),
				((7, 6), 755),
				((7, 7), 755),
				((7, 8), 755),
				((9, 1), 664),
				((9, 2), 664),
				((9, 3), 664),
				((9, 5), 598),
				((9, 6), 598),
				((9, 7), 598),
			])
		);
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
