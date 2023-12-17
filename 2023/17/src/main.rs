use pathfinding::prelude::dijkstra;
use std::{convert::Infallible, io::Read, num::NonZeroUsize, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
	Up,
	Left,
	Down,
	Right,
}
impl Direction {
	fn clockwise(&self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Left => Self::Up,
			Self::Down => Self::Left,
			Self::Right => Self::Down,
		}
	}

	fn counter_clockwise(&self) -> Self {
		match self {
			Self::Up => Self::Left,
			Self::Left => Self::Down,
			Self::Down => Self::Right,
			Self::Right => Self::Up,
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Map(Vec<Vec<u8>>);
impl FromStr for Map {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self(
			s.lines()
				.map(|s| s.bytes().map(|c| c - b'0').collect())
				.collect(),
		))
	}
}
impl Map {
	fn adjacents(
		&self,
		(l, c): (usize, usize),
		dir: Direction,
		dist_min: NonZeroUsize,
		dist_max: NonZeroUsize,
	) -> Vec<(usize, (usize, usize), Direction)> {
		let mut ret = Vec::with_capacity(2 * (dist_max.get() - dist_min.get() + 1));

		let mut adjacents_in_direction = |dir: Direction| {
			let mut dist = 0;
			(1..=dist_max.get()).for_each(|target_dist| {
				let (target_l, target_c) = match dir {
					Direction::Up if l >= target_dist => (l - target_dist, c),
					Direction::Left if c >= target_dist => (l, c - target_dist),
					Direction::Down if l + target_dist < self.0.len() => (l + target_dist, c),
					Direction::Right if c + target_dist < self.0[0].len() => (l, c + target_dist),
					_ => {
						return;
					}
				};
				dist += self.0[target_l][target_c] as usize;
				if target_dist >= dist_min.get() {
					ret.push((dist, (target_l, target_c), dir));
				}
			});
		};
		if l != self.0.len() - 1 || c != self.0[0].len() - 1 {
			adjacents_in_direction(dir.clockwise());
			adjacents_in_direction(dir.counter_clockwise());
		}

		ret
	}

	fn best_path(&self, dist_min: NonZeroUsize, dist_max: NonZeroUsize) -> usize {
		dijkstra(
			&(0, 0, vec![Direction::Down, Direction::Right]),
			|state| {
				let (l, c, dirs) = state.clone();
				dirs.into_iter()
					.flat_map(move |dir| self.adjacents((l, c), dir, dist_min, dist_max))
					.map(|(dist, (l, c), dir)| ((l, c, vec![dir]), dist))
			},
			|(l, c, _dirs)| *l == self.0.len() - 1 && *c == self.0[0].len() - 1,
		)
		.unwrap()
		.1
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	#[cfg(not(feature = "p2"))]
	let (dist_min, dist_max) = (NonZeroUsize::new(1).unwrap(), NonZeroUsize::new(3).unwrap());
	#[cfg(feature = "p2")]
	let (dist_min, dist_max) = (
		NonZeroUsize::new(4).unwrap(),
		NonZeroUsize::new(10).unwrap(),
	);

	let best = Map::from_str(&input).unwrap().best_path(dist_min, dist_max);
	println!("{best}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn map_from_str() {
		assert_eq!(
			Map::from_str(include_str!("test.txt")),
			Ok(Map(vec![
				vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
				vec![3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
				vec![3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
				vec![3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
				vec![4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
				vec![1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
				vec![4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
				vec![3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
				vec![4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
				vec![4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
				vec![1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
				vec![2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
				vec![4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
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
