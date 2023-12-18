use cached::proc_macro::cached;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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
}

#[cached]
fn count_arrangements(row: Row) -> usize {
	let Row {
		row,
		damaged_groups,
	} = row;
	if damaged_groups.is_empty() {
		!row.contains(&State::Damaged) as _
	} else if row.is_empty() {
		damaged_groups.is_empty() as _
	} else {
		let mut ret = 0;

		if row[0] != State::Damaged {
			ret += count_arrangements(Row {
				row: row[1..].to_vec(),
				damaged_groups: damaged_groups.clone(),
			});
		}
		if row[0] != State::Operational
			&& (damaged_groups[0] <= row.len()
				&& !row[..damaged_groups[0]].contains(&State::Operational)
				&& (damaged_groups[0] == row.len() || row[damaged_groups[0]] != State::Damaged))
		{
			ret += count_arrangements(Row {
				row: row
					.get((damaged_groups[0] + 1)..)
					.unwrap_or_default()
					.to_vec(),
				damaged_groups: damaged_groups[1..].to_vec(),
			});
		}

		ret
	}
}

fn main() {
	#[cfg(not(feature = "p2"))]
	fn map_lines(rows: impl Iterator<Item = Row>) -> impl Iterator<Item = usize> {
		rows.map(count_arrangements)
	}

	#[cfg(feature = "p2")]
	fn map_lines(rows: impl Iterator<Item = Row>) -> impl Iterator<Item = usize> {
		rows.map(|mut row| {
			row.unfold();
			count_arrangements(row)
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

	#[test]
	fn count_arrangements() {
		assert_eq!(
			super::count_arrangements(Row::from_str("???.### 1,1,3").unwrap()),
			1
		);
		assert_eq!(
			super::count_arrangements(Row::from_str(".??..??...?##. 1,1,3").unwrap()),
			4
		);
		assert_eq!(
			super::count_arrangements(Row::from_str("?#?#?#?#?#?#?#? 1,3,1,6").unwrap()),
			1
		);
		assert_eq!(
			super::count_arrangements(Row::from_str("????.#...#... 4,1,1").unwrap()),
			1
		);
		assert_eq!(
			super::count_arrangements(Row::from_str("????.######..#####. 1,6,5").unwrap()),
			4
		);
		assert_eq!(
			super::count_arrangements(Row::from_str("?###???????? 3,2,1").unwrap()),
			10
		);
	}

	#[cfg(feature = "p2")]
	#[test]
	fn row_unfold_count_arrangements() {
		let mut row = Row::from_str("???.### 1,1,3").unwrap();
		row.unfold();
		assert_eq!(super::count_arrangements(row), 1);

		row = Row::from_str(".??..??...?##. 1,1,3").unwrap();
		row.unfold();
		assert_eq!(super::count_arrangements(row), 16384);

		row = Row::from_str("?#?#?#?#?#?#?#? 1,3,1,6").unwrap();
		row.unfold();
		assert_eq!(super::count_arrangements(row), 1);

		row = Row::from_str("????.#...#... 4,1,1").unwrap();
		row.unfold();
		assert_eq!(super::count_arrangements(row), 16);

		row = Row::from_str("????.######..#####. 1,6,5").unwrap();
		row.unfold();
		assert_eq!(super::count_arrangements(row), 2500);

		row = Row::from_str("?###???????? 3,2,1").unwrap();
		row.unfold();
		assert_eq!(super::count_arrangements(row), 506250);
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
