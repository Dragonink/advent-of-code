use std::{io::Read, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
	Up,
	Left,
	Down,
	Right,
}
impl TryFrom<u8> for Direction {
	type Error = &'static str;

	fn try_from(c: u8) -> Result<Self, Self::Error> {
		match c {
			b'U' | b'3' => Ok(Self::Up),
			b'L' | b'2' => Ok(Self::Left),
			b'D' | b'1' => Ok(Self::Down),
			b'R' | b'0' => Ok(Self::Right),
			_ => Err("invalid direction"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PlanStep {
	direction: Direction,
	length: usize,
}
impl FromStr for PlanStep {
	type Err = &'static str;

	#[cfg(not(feature = "p2"))]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut data = s.split_whitespace();
		let direction = data.next().ok_or("missing direction")?.as_bytes()[0].try_into()?;
		let length = data
			.next()
			.ok_or("missing length")?
			.parse()
			.map_err(|_err| "could not parse the length")?;
		Ok(Self { direction, length })
	}

	#[cfg(feature = "p2")]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let direction = s.as_bytes()[s.len() - 2].try_into()?;
		let length = usize::from_str_radix(&s[(s.len() - 2 - 5)..(s.len() - 2)], 16)
			.map_err(|_err| "could not parse the length")?;
		Ok(Self { direction, length })
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Plan(Vec<PlanStep>);
impl FromStr for Plan {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.lines()
			.map(PlanStep::from_str)
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl Plan {
	#[allow(dead_code)]
	fn map(&self) -> Vec<Vec<bool>> {
		let mut pos = (0_isize, 0_isize);
		let (line_range, column_range) =
			self.0
				.iter()
				.fold((0..=0, 0..=0), |(line_range, column_range), step| {
					match step.direction {
						Direction::Up => {
							pos.0 -= step.length as isize;
						}
						Direction::Left => {
							pos.1 -= step.length as isize;
						}
						Direction::Down => {
							pos.0 += step.length as isize;
						}
						Direction::Right => {
							pos.1 += step.length as isize;
						}
					}
					(
						pos.0.min(*line_range.start())..=pos.0.max(*line_range.end()),
						pos.1.min(*column_range.start())..=pos.1.max(*column_range.end()),
					)
				});

		let mut pos = (
			0.min(*line_range.start()).abs() as usize,
			0.min(*column_range.start()).abs() as usize,
		);
		let mut ret = vec![vec![false; column_range.count()]; line_range.count()];
		ret[pos.0][pos.1] = true;
		self.0.iter().for_each(|step| match step.direction {
			Direction::Up => {
				for l in (pos.0.saturating_sub(step.length)..pos.0).rev() {
					pos.0 = l;
					ret[pos.0][pos.1] = true;
				}
			}
			Direction::Left => {
				for c in (pos.1.saturating_sub(step.length)..pos.1).rev() {
					pos.1 = c;
					ret[pos.0][pos.1] = true;
				}
			}
			Direction::Down => {
				for l in (pos.0 + 1)..=(pos.0 + step.length) {
					pos.0 = l;
					ret[pos.0][pos.1] = true;
				}
			}
			Direction::Right => {
				for c in (pos.1 + 1)..=(pos.1 + step.length) {
					pos.1 = c;
					ret[pos.0][pos.1] = true;
				}
			}
		});
		ret
	}

	fn dug_area(&self) -> usize {
		let mut pos = (0_isize, 0_isize);
		let (area, perimeter) =
			self.0
				.iter()
				.fold((0_isize, 0_usize), |(mut area, perimeter), step| {
					let new_pos = match step.direction {
						Direction::Up => (pos.0 - step.length as isize, pos.1),
						Direction::Left => (pos.0, pos.1 - step.length as isize),
						Direction::Down => (pos.0 + step.length as isize, pos.1),
						Direction::Right => (pos.0, pos.1 + step.length as isize),
					};
					area = area + pos.0 * new_pos.1 - pos.1 * new_pos.0;
					pos = new_pos;
					(area, perimeter + step.length)
				});
		(area.unsigned_abs() + perimeter) / 2 + 1
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let area = Plan::from_str(&input).unwrap().dug_area();
	println!("{area}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(not(feature = "p2"))]
	#[test]
	fn plan_from_str() {
		assert_eq!(
			Plan::from_str(include_str!("test.txt")),
			Ok(Plan(vec![
				PlanStep {
					direction: Direction::Right,
					length: 6,
				},
				PlanStep {
					direction: Direction::Down,
					length: 5,
				},
				PlanStep {
					direction: Direction::Left,
					length: 2,
				},
				PlanStep {
					direction: Direction::Down,
					length: 2,
				},
				PlanStep {
					direction: Direction::Right,
					length: 2,
				},
				PlanStep {
					direction: Direction::Down,
					length: 2,
				},
				PlanStep {
					direction: Direction::Left,
					length: 5,
				},
				PlanStep {
					direction: Direction::Up,
					length: 2,
				},
				PlanStep {
					direction: Direction::Left,
					length: 1,
				},
				PlanStep {
					direction: Direction::Up,
					length: 2,
				},
				PlanStep {
					direction: Direction::Right,
					length: 2,
				},
				PlanStep {
					direction: Direction::Up,
					length: 3,
				},
				PlanStep {
					direction: Direction::Left,
					length: 2,
				},
				PlanStep {
					direction: Direction::Up,
					length: 2,
				},
			]))
		);
	}

	#[cfg(feature = "p2")]
	#[test]
	fn plan_from_str() {
		assert_eq!(
			Plan::from_str(include_str!("test.txt")),
			Ok(Plan(vec![
				PlanStep {
					direction: Direction::Right,
					length: 461937,
				},
				PlanStep {
					direction: Direction::Down,
					length: 56407,
				},
				PlanStep {
					direction: Direction::Right,
					length: 356671,
				},
				PlanStep {
					direction: Direction::Down,
					length: 863240,
				},
				PlanStep {
					direction: Direction::Right,
					length: 367720,
				},
				PlanStep {
					direction: Direction::Down,
					length: 266681,
				},
				PlanStep {
					direction: Direction::Left,
					length: 577262,
				},
				PlanStep {
					direction: Direction::Up,
					length: 829975,
				},
				PlanStep {
					direction: Direction::Left,
					length: 112010,
				},
				PlanStep {
					direction: Direction::Down,
					length: 829975,
				},
				PlanStep {
					direction: Direction::Left,
					length: 491645,
				},
				PlanStep {
					direction: Direction::Up,
					length: 686074,
				},
				PlanStep {
					direction: Direction::Left,
					length: 5411,
				},
				PlanStep {
					direction: Direction::Up,
					length: 500254,
				},
			]))
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
