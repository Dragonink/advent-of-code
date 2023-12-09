use std::str::FromStr;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Sequence(Vec<isize>);
impl FromStr for Sequence {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.split_ascii_whitespace()
			.map(|s| s.parse().map_err(|_err| "could not parse value"))
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Sequence {
	fn difference(&self) -> Self {
		let mut diff = Vec::with_capacity(self.0.len() - 1);

		let mut iter = self.0.iter().copied();
		let mut a = iter
			.next()
			.expect("sequence should have at least 2 elements");
		let mut b = iter
			.next()
			.expect("sequence should have at least 2 elements");
		diff.push(b - a);
		while let Some(c) = iter.next() {
			a = b;
			b = c;
			diff.push(b - a);
		}

		Self(diff)
	}

	fn extrapolate(&self) -> isize {
		if self.0.iter().all(|value| *value == 0) {
			0
		} else {
			let lower_extrapolated = self.difference().extrapolate();
			#[cfg(not(feature = "p2"))]
			{
				*self.0.last().unwrap() + lower_extrapolated
			}
			#[cfg(feature = "p2")]
			{
				*self.0.first().unwrap() - lower_extrapolated
			}
		}
	}
}

fn main() {
	let sum: isize = std::io::stdin()
		.lines()
		.map(|res| Sequence::from_str(&res.unwrap()).unwrap().extrapolate())
		.sum();
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sequence_from_str() {
		assert_eq!(
			Sequence::from_str("0 3 6 9 12 15"),
			Ok(Sequence(vec![0, 3, 6, 9, 12, 15]))
		);
		assert_eq!(
			Sequence::from_str("1 3 6 10 15 21"),
			Ok(Sequence(vec![1, 3, 6, 10, 15, 21]))
		);
		assert_eq!(
			Sequence::from_str("10 13 16 21 30 45"),
			Ok(Sequence(vec![10, 13, 16, 21, 30, 45]))
		);
	}

	#[test]
	fn sequence_difference() {
		assert_eq!(
			Sequence::from_str("0 3 6 9 12 15").unwrap().difference(),
			Sequence(vec![3; 5])
		);
		assert_eq!(
			Sequence::from_str("1 3 6 10 15 21").unwrap().difference(),
			Sequence(vec![2, 3, 4, 5, 6])
		);
		assert_eq!(
			Sequence::from_str("10 13 16 21 30 45")
				.unwrap()
				.difference(),
			Sequence(vec![3, 3, 5, 9, 15])
		);
	}

	#[cfg(not(feature = "p2"))]
	#[test]
	fn sequence_extrapolate() {
		assert_eq!(
			Sequence::from_str("0 3 6 9 12 15").unwrap().extrapolate(),
			18
		);
		assert_eq!(
			Sequence::from_str("1 3 6 10 15 21").unwrap().extrapolate(),
			28
		);
		assert_eq!(
			Sequence::from_str("10 13 16 21 30 45")
				.unwrap()
				.extrapolate(),
			68
		);
	}

	#[cfg(feature = "p2")]
	#[test]
	fn sequence_extrapolate() {
		assert_eq!(
			Sequence::from_str("0 3 6 9 12 15").unwrap().extrapolate(),
			-3
		);
		assert_eq!(
			Sequence::from_str("1 3 6 10 15 21").unwrap().extrapolate(),
			0
		);
		assert_eq!(
			Sequence::from_str("10 13 16 21 30 45")
				.unwrap()
				.extrapolate(),
			5
		);
	}

	#[cfg(star)]
	#[test]
	fn p1() {
		assert!(true);
	}
	#[cfg(star)]
	#[test]
	fn p2() {
		assert!(true);
	}
}
