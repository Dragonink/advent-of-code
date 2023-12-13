use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Floor {
	Ash,
	Rock,
}
impl TryFrom<u8> for Floor {
	type Error = &'static str;

	fn try_from(c: u8) -> Result<Self, Self::Error> {
		match c {
			b'.' => Ok(Self::Ash),
			b'#' => Ok(Self::Rock),
			_ => Err("invalid floor"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reflection {
	Horizontal(usize),
	Vertical(usize),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Pattern(Vec<Vec<Floor>>);
impl FromStr for Pattern {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.lines()
			.map(|s| s.bytes().map(Floor::try_from).collect::<Result<_, _>>())
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Pattern {
	fn transpose(&self) -> Self {
		let lines = self.0.len();
		let columns = self.0.get(0).map(|v| v.len()).unwrap_or_default();
		Self(
			(0..columns)
				.map(|c| (0..lines).map(|l| self.0[l][c]).collect())
				.collect(),
		)
	}

	fn find_reflection(&self) -> Option<Reflection> {
		#[cfg(not(feature = "p2"))]
		let try_each_row = |this: &Self| {
			for r in 1..this.0.len() {
				let normal = &this.0[..r];
				let mirrored = &this.0[r..];
				let rows = normal.len().min(mirrored.len());
				let normal = &normal[(normal.len() - rows)..];
				let mirrored = mirrored[..rows].iter().rev();
				debug_assert_eq!(normal.len(), mirrored.len());
				if mirrored.eq(normal) {
					return Some(r);
				}
			}
			None
		};

		#[cfg(feature = "p2")]
		let try_each_row = |this: &Self| {
			for r in 1..this.0.len() {
				let normal = &this.0[..r];
				let mirrored = &this.0[r..];
				let rows = normal.len().min(mirrored.len());
				let normal = &normal[(normal.len() - rows)..];
				let mirrored = mirrored[..rows].iter().rev();
				debug_assert_eq!(normal.len(), mirrored.len());
				if mirrored
					.flatten()
					.zip(normal.iter().flatten())
					.filter(|(m, n)| m == n)
					.count() == normal.iter().flatten().count() - 1
				{
					return Some(r);
				}
			}
			None
		};

		if let Some(l) = try_each_row(self) {
			return Some(Reflection::Horizontal(l));
		}
		let transposed = self.transpose();
		if let Some(c) = try_each_row(&transposed) {
			return Some(Reflection::Vertical(c));
		}
		None
	}
}

fn main() {
	let mut lines = std::io::stdin().lines().map(|res| res.unwrap()).peekable();

	let mut sum: usize = 0;
	while lines.peek().is_some() {
		let s = lines
			.by_ref()
			.skip_while(|s| s.is_empty())
			.take_while(|s| !s.is_empty())
			.collect::<Vec<_>>()
			.join("\n");
		if s.is_empty() {
			break;
		}
		sum += match Pattern::from_str(s.trim()).unwrap().find_reflection() {
			Some(Reflection::Horizontal(l)) => 100 * l,
			Some(Reflection::Vertical(c)) => c,
			None => panic!("{s}"),
		};
	}
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pattern_from_str() {
		const TEST: &str = include_str!("test.txt");
		let patterns = TEST.lines();

		let pattern = patterns
			.take_while(|s| !s.is_empty())
			.collect::<Vec<_>>()
			.join("\n");
		assert_eq!(
			Pattern::from_str(pattern.trim()),
			Ok(Pattern(vec![
				vec![
					Floor::Rock,
					Floor::Ash,
					Floor::Rock,
					Floor::Rock,
					Floor::Ash,
					Floor::Ash,
					Floor::Rock,
					Floor::Rock,
					Floor::Ash,
				],
				vec![
					Floor::Ash,
					Floor::Ash,
					Floor::Rock,
					Floor::Ash,
					Floor::Rock,
					Floor::Rock,
					Floor::Ash,
					Floor::Rock,
					Floor::Ash,
				],
				vec![
					Floor::Rock,
					Floor::Rock,
					Floor::Ash,
					Floor::Ash,
					Floor::Ash,
					Floor::Ash,
					Floor::Ash,
					Floor::Ash,
					Floor::Rock,
				],
				vec![
					Floor::Rock,
					Floor::Rock,
					Floor::Ash,
					Floor::Ash,
					Floor::Ash,
					Floor::Ash,
					Floor::Ash,
					Floor::Ash,
					Floor::Rock,
				],
				vec![
					Floor::Ash,
					Floor::Ash,
					Floor::Rock,
					Floor::Ash,
					Floor::Rock,
					Floor::Rock,
					Floor::Ash,
					Floor::Rock,
					Floor::Ash,
				],
				vec![
					Floor::Ash,
					Floor::Ash,
					Floor::Rock,
					Floor::Rock,
					Floor::Ash,
					Floor::Ash,
					Floor::Rock,
					Floor::Rock,
					Floor::Ash,
				],
				vec![
					Floor::Rock,
					Floor::Ash,
					Floor::Rock,
					Floor::Ash,
					Floor::Rock,
					Floor::Rock,
					Floor::Ash,
					Floor::Rock,
					Floor::Ash,
				],
			]))
		);
	}

	#[test]
	fn pattern_transpose() {
		assert_eq!(
			Pattern(vec![
				vec![Floor::Rock, Floor::Ash, Floor::Rock],
				vec![Floor::Ash, Floor::Ash, Floor::Rock],
			])
			.transpose(),
			Pattern(vec![
				vec![Floor::Rock, Floor::Ash],
				vec![Floor::Ash, Floor::Ash],
				vec![Floor::Rock, Floor::Rock],
			])
		);
	}

	#[cfg(not(feature = "p2"))]
	#[test]
	fn pattern_find_reflection() {
		const TEST: &str = include_str!("test.txt");
		let mut patterns = TEST.lines();

		let pattern_1 = patterns
			.by_ref()
			.take_while(|s| !s.is_empty())
			.collect::<Vec<_>>()
			.join("\n");
		let pattern_1 = Pattern::from_str(pattern_1.trim()).unwrap();
		assert_eq!(pattern_1.find_reflection(), Some(Reflection::Vertical(5)));

		let pattern_2 = patterns
			.by_ref()
			.skip_while(|s| s.is_empty())
			.take_while(|s| !s.is_empty())
			.collect::<Vec<_>>()
			.join("\n");
		let pattern_2 = Pattern::from_str(pattern_2.trim()).unwrap();
		assert_eq!(pattern_2.find_reflection(), Some(Reflection::Horizontal(4)));
	}

	#[cfg(feature = "p2")]
	#[test]
	fn pattern_find_reflection() {
		const TEST: &str = include_str!("test.txt");
		let mut patterns = TEST.lines();

		let pattern_1 = patterns
			.by_ref()
			.take_while(|s| !s.is_empty())
			.collect::<Vec<_>>()
			.join("\n");
		let pattern_1 = Pattern::from_str(pattern_1.trim()).unwrap();
		assert_eq!(pattern_1.find_reflection(), Some(Reflection::Horizontal(3)));

		let pattern_2 = patterns
			.by_ref()
			.skip_while(|s| s.is_empty())
			.take_while(|s| !s.is_empty())
			.collect::<Vec<_>>()
			.join("\n");
		let pattern_2 = Pattern::from_str(pattern_2.trim()).unwrap();
		assert_eq!(pattern_2.find_reflection(), Some(Reflection::Horizontal(1)));
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
