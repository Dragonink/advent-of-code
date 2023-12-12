use rayon::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
	Operational,
	Damaged,
	Unknown,
}
impl TryFrom<u8> for State {
	type Error = &'static str;

	fn try_from(c: u8) -> Result<Self, Self::Error> {
		match c {
			b'.' => Ok(Self::Operational),
			b'#' => Ok(Self::Damaged),
			b'?' => Ok(Self::Unknown),
			_ => Err("invalid state"),
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Row {
	row: Vec<State>,
	damaged_groups: Vec<usize>,
}
impl FromStr for Row {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut data = s.split_ascii_whitespace();
		let row = data
			.next()
			.ok_or("missing row")?
			.bytes()
			.map(State::try_from)
			.collect::<Result<_, _>>()?;
		let damaged_groups = data
			.next()
			.ok_or("missing damaged groups")?
			.split(',')
			.map(|n| n.parse().map_err(|_err| "could not parse group length"))
			.collect::<Result<_, _>>()?;

		Ok(Self {
			row,
			damaged_groups,
		})
	}
}
impl Row {
	#[cfg(feature = "p2")]
	fn unfold(&mut self) {
		self.row = self
			.row
			.iter()
			.chain([&State::Unknown])
			.chain(&self.row)
			.chain([&State::Unknown])
			.chain(&self.row)
			.chain([&State::Unknown])
			.chain(&self.row)
			.chain([&State::Unknown])
			.chain(&self.row)
			.copied()
			.collect();

		let len = self.damaged_groups.len();
		for _ in 1..5 {
			self.damaged_groups.extend_from_within(..len);
		}
	}

	fn is_arrangement_valid(&self, row: &[State]) -> bool {
		let mut row = row.iter().peekable();
		self.damaged_groups.iter().all(|&group| {
			row.by_ref()
				.skip_while(|state| **state == State::Operational)
				.take_while(|state| **state == State::Damaged)
				.count() == group
		}) && row.all(|state| *state == State::Operational)
	}

	fn arrangements(&self) -> impl ParallelIterator<Item = Vec<State>> + '_ {
		let unknowns = self
			.row
			.iter()
			.enumerate()
			.filter_map(|(i, state)| (*state == State::Unknown).then_some(i))
			.collect::<Vec<_>>();
		dbg!(unknowns.len());
		assert!(unknowns.len() < 128);
		(0..(1_u128 << unknowns.len()))
			.into_par_iter()
			.map(move |combi| {
				let mut row = self.row.clone();
				unknowns.iter().enumerate().for_each(|(k, &i)| {
					row[i] = if combi & 1 << k != 0 {
						State::Damaged
					} else {
						State::Operational
					};
				});
				row
			})
			.filter(|row| self.is_arrangement_valid(row))
	}
}

fn main() {
	#[cfg(not(feature = "p2"))]
	fn map_lines(rows: impl Iterator<Item = Row>) -> impl Iterator<Item = usize> {
		rows.map(|row| row.arrangements().count())
	}

	#[cfg(feature = "p2")]
	fn map_lines(rows: impl Iterator<Item = Row>) -> impl Iterator<Item = usize> {
		rows.enumerate().map(|(r, mut row)| {
			row.unfold();
			let start = std::time::Instant::now();
			let ret = row.arrangements().count();
			eprintln!("{r}: {}s", start.elapsed().as_secs());
			ret
		})
	}

	let sum: usize = map_lines(
		std::io::stdin()
			.lines()
			.map(|res| Row::from_str(&res.unwrap()).unwrap()),
	)
	.sum();
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn row_from_str() {
		assert_eq!(
			Row::from_str("???.### 1,1,3"),
			Ok(Row {
				row: vec![
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Operational,
					State::Damaged,
					State::Damaged,
					State::Damaged,
				],
				damaged_groups: vec![1, 1, 3],
			})
		);
		assert_eq!(
			Row::from_str(".??..??...?##. 1,1,3"),
			Ok(Row {
				row: vec![
					State::Operational,
					State::Unknown,
					State::Unknown,
					State::Operational,
					State::Operational,
					State::Unknown,
					State::Unknown,
					State::Operational,
					State::Operational,
					State::Operational,
					State::Unknown,
					State::Damaged,
					State::Damaged,
					State::Operational,
				],
				damaged_groups: vec![1, 1, 3],
			})
		);
		assert_eq!(
			Row::from_str("?#?#?#?#?#?#?#? 1,3,1,6"),
			Ok(Row {
				row: vec![
					State::Unknown,
					State::Damaged,
					State::Unknown,
					State::Damaged,
					State::Unknown,
					State::Damaged,
					State::Unknown,
					State::Damaged,
					State::Unknown,
					State::Damaged,
					State::Unknown,
					State::Damaged,
					State::Unknown,
					State::Damaged,
					State::Unknown,
				],
				damaged_groups: vec![1, 3, 1, 6],
			})
		);
		assert_eq!(
			Row::from_str("????.#...#... 4,1,1"),
			Ok(Row {
				row: vec![
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Operational,
					State::Damaged,
					State::Operational,
					State::Operational,
					State::Operational,
					State::Damaged,
					State::Operational,
					State::Operational,
					State::Operational,
				],
				damaged_groups: vec![4, 1, 1],
			})
		);
		assert_eq!(
			Row::from_str("????.######..#####. 1,6,5"),
			Ok(Row {
				row: vec![
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Operational,
					State::Damaged,
					State::Damaged,
					State::Damaged,
					State::Damaged,
					State::Damaged,
					State::Damaged,
					State::Operational,
					State::Operational,
					State::Damaged,
					State::Damaged,
					State::Damaged,
					State::Damaged,
					State::Damaged,
					State::Operational,
				],
				damaged_groups: vec![1, 6, 5],
			})
		);
		assert_eq!(
			Row::from_str("?###???????? 3,2,1"),
			Ok(Row {
				row: vec![
					State::Unknown,
					State::Damaged,
					State::Damaged,
					State::Damaged,
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Unknown,
					State::Unknown,
				],
				damaged_groups: vec![3, 2, 1],
			})
		);
	}

	#[test]
	fn row_is_arrangement_valid() {
		assert!(Row::from_str("?###???????? 3,2,1")
			.unwrap()
			.is_arrangement_valid(&[
				State::Operational,
				State::Damaged,
				State::Damaged,
				State::Damaged,
				State::Operational,
				State::Damaged,
				State::Damaged,
				State::Operational,
				State::Damaged,
				State::Operational,
				State::Operational,
				State::Operational,
				State::Operational,
			]));
		assert!(Row::from_str("?###???????? 3,2,1")
			.unwrap()
			.is_arrangement_valid(&[
				State::Operational,
				State::Damaged,
				State::Damaged,
				State::Damaged,
				State::Operational,
				State::Damaged,
				State::Damaged,
				State::Operational,
				State::Operational,
				State::Operational,
				State::Operational,
				State::Operational,
				State::Damaged,
			]));
		assert!(!Row::from_str("?###???????? 3,2,1")
			.unwrap()
			.is_arrangement_valid(&[
				State::Operational,
				State::Damaged,
				State::Damaged,
				State::Damaged,
				State::Damaged,
				State::Operational,
				State::Damaged,
				State::Operational,
				State::Operational,
				State::Damaged,
				State::Operational,
				State::Operational,
				State::Operational,
			]));
	}

	#[test]
	fn row_arrangements() {
		assert_eq!(
			Row::from_str("???.### 1,1,3")
				.unwrap()
				.arrangements()
				.count(),
			1
		);
		assert_eq!(
			Row::from_str(".??..??...?##. 1,1,3")
				.unwrap()
				.arrangements()
				.count(),
			4
		);
		assert_eq!(
			Row::from_str("?#?#?#?#?#?#?#? 1,3,1,6")
				.unwrap()
				.arrangements()
				.count(),
			1
		);
		assert_eq!(
			Row::from_str("????.#...#... 4,1,1")
				.unwrap()
				.arrangements()
				.count(),
			1
		);
		assert_eq!(
			Row::from_str("????.######..#####. 1,6,5")
				.unwrap()
				.arrangements()
				.count(),
			4
		);
		assert_eq!(
			Row::from_str("?###???????? 3,2,1")
				.unwrap()
				.arrangements()
				.count(),
			10
		);
	}

	#[cfg(feature = "p2")]
	#[test]
	fn row_unfold_arrangements() {
		let mut row = Row::from_str("???.### 1,1,3").unwrap();
		row.unfold();
		assert_eq!(row.arrangements().count(), 1);

		row = Row::from_str(".??..??...?##. 1,1,3").unwrap();
		row.unfold();
		// assert_eq!(row.arrangements().count(), 16384);

		row = Row::from_str("?#?#?#?#?#?#?#? 1,3,1,6").unwrap();
		row.unfold();
		// assert_eq!(row.arrangements().count(), 1);

		row = Row::from_str("????.#...#... 4,1,1").unwrap();
		row.unfold();
		assert_eq!(row.arrangements().count(), 16);

		row = Row::from_str("????.######..#####. 1,6,5").unwrap();
		row.unfold();
		assert_eq!(row.arrangements().count(), 2500);

		row = Row::from_str("?###???????? 3,2,1").unwrap();
		row.unfold();
		// assert_eq!(row.arrangements().count(), 506250);
	}

	#[cfg(feature = "p2")]
	#[test]
	fn row_unfold() {
		let mut row = Row::from_str("???.### 1,1,3").unwrap();
		row.unfold();
		assert_eq!(
			row,
			Row::from_str("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3")
				.unwrap()
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
		assert!(false);
	}
}
