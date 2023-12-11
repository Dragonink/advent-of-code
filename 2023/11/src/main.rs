use std::{collections::BTreeSet, io::Read, str::FromStr};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Map {
	galaxies: BTreeSet<(usize, usize)>,
	lines: usize,
	columns: usize,
}
impl FromStr for Map {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let lines = s.lines().count();
		let columns = s.lines().next().ok_or("input is empty")?.len();
		let galaxies = s
			.lines()
			.enumerate()
			.flat_map(|(l, line)| {
				line.as_bytes()
					.iter()
					.enumerate()
					.map(move |(c, b)| (l, c, b))
			})
			.filter_map(|(l, c, b)| (*b == b'#').then_some((l, c)))
			.collect();

		Ok(Map {
			galaxies,
			lines,
			columns,
		})
	}
}
impl Map {
	fn expand(&mut self) {
		#[cfg(not(feature = "p2"))]
		const FACTOR: usize = 1;
		#[cfg(feature = "p2")]
		const FACTOR: usize = 999_999;

		let mut empty_lines: BTreeSet<usize> = (0..self.lines).collect();
		let mut empty_columns: BTreeSet<usize> = (0..self.columns).collect();
		self.galaxies.iter().for_each(|(l, c)| {
			empty_lines.remove(l);
			empty_columns.remove(c);
		});

		self.lines += empty_lines.len() * FACTOR;
		self.columns += empty_columns.len() * FACTOR;
		self.galaxies = self
			.galaxies
			.iter()
			.map(|(l, c)| {
				let mut empty_lines = empty_lines.clone();
				empty_lines.split_off(l);
				let mut empty_columns = empty_columns.clone();
				empty_columns.split_off(c);

				(
					*l + empty_lines.len() * FACTOR,
					*c + empty_columns.len() * FACTOR,
				)
			})
			.collect();
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let mut map = Map::from_str(&input).unwrap();
	map.expand();
	let sum: usize = map
		.galaxies
		.iter()
		.enumerate()
		.flat_map(|(i, a)| map.galaxies.iter().skip(i).map(move |b| (a, b)))
		.filter(|(a, b)| a != b)
		.map(|((al, ac), (bl, bc))| al.max(bl) - al.min(bl) + ac.max(bc) - ac.min(bc))
		.sum();
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn map_from_str() {
		assert_eq!(
			Map::from_str(include_str!("test.txt")),
			Ok(Map {
				galaxies: BTreeSet::from([
					(0, 3),
					(1, 7),
					(2, 0),
					(4, 6),
					(5, 1),
					(6, 9),
					(8, 7),
					(9, 0),
					(9, 4),
				]),
				lines: 10,
				columns: 10,
			})
		);
	}

	#[cfg(not(feature = "p2"))]
	#[test]
	fn map_expand() {
		let mut map = Map::from_str(include_str!("test.txt")).unwrap();
		map.expand();
		assert_eq!(
			map,
			Map {
				galaxies: BTreeSet::from([
					(0, 4),
					(1, 9),
					(2, 0),
					(5, 8),
					(6, 1),
					(7, 12),
					(10, 9),
					(11, 0),
					(11, 5)
				]),
				lines: 12,
				columns: 13,
			}
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
