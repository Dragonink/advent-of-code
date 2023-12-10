use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Draw {
	red: Option<usize>,
	green: Option<usize>,
	blue: Option<usize>,
}
impl FromStr for Draw {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.split(',')
			.try_fold(Self::default(), |mut draw, color_count| {
				let mut color_count = color_count.trim().split_ascii_whitespace();
				let count = color_count
					.next()
					.ok_or("missing the color count")?
					.parse()
					.map_err(|_err| "could not parse the color count")?;
				match color_count.next().ok_or("missing the color")? {
					"red" => {
						draw.red = Some(count);
					}
					"green" => {
						draw.green = Some(count);
					}
					"blue" => {
						draw.blue = Some(count);
					}
					_ => {
						return Err("invalid cube color");
					}
				}

				Ok(draw)
			})
	}
}
impl PartialOrd for Draw {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl Ord for Draw {
	fn cmp(&self, other: &Self) -> Ordering {
		let cmp_usize_pair = |(lhs, rhs): (usize, usize)| lhs.cmp(&rhs);

		let red = self
			.red
			.zip(other.red)
			.map_or(Ordering::Less, cmp_usize_pair);
		let green = self
			.green
			.zip(other.green)
			.map_or(Ordering::Less, cmp_usize_pair);
		let blue = self
			.blue
			.zip(other.blue)
			.map_or(Ordering::Less, cmp_usize_pair);
		match (red, green, blue) {
			(Ordering::Equal, Ordering::Equal, Ordering::Equal) => Ordering::Equal,
			(Ordering::Greater, _, _) | (_, Ordering::Greater, _) | (_, _, Ordering::Greater) => {
				Ordering::Greater
			}
			_ => Ordering::Less,
		}
	}
}
#[cfg(feature = "p2")]
impl Draw {
	#[inline]
	fn power(&self) -> usize {
		self.red.unwrap_or(1) * self.green.unwrap_or(1) * self.blue.unwrap_or(1)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
	id: usize,
	draws: Vec<Draw>,
}
impl FromStr for Game {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut game = s.split(':');
		let id = game
			.next()
			.ok_or("missing the game ID")?
			.trim()
			.trim_start_matches("Game ")
			.parse()
			.map_err(|_err| "could not parse the game ID")?;
		let draws = game
			.next()
			.ok_or("missing the game draws")?
			.split(';')
			.map(Draw::from_str)
			.collect::<Result<_, _>>()?;

		Ok(Self { id, draws })
	}
}
impl Game {
	fn merge_max_draws(&self) -> Option<Draw> {
		self.draws.iter().cloned().reduce(|mut max, draw| {
			if draw.red > max.red {
				max.red = draw.red;
			}
			if draw.green > max.green {
				max.green = draw.green;
			}
			if draw.blue > max.blue {
				max.blue = draw.blue;
			}

			max
		})
	}
}

#[allow(dead_code)]
const LIMIT: Draw = Draw {
	red: Some(12),
	green: Some(13),
	blue: Some(14),
};

fn main() {
	#[cfg(not(feature = "p2"))]
	fn map_lines(games: impl Iterator<Item = Game>) -> impl Iterator<Item = usize> {
		games
			.map(|game| (game.id, game.merge_max_draws().unwrap()))
			.filter_map(|(id, max)| (max <= LIMIT).then_some(id))
	}

	#[cfg(feature = "p2")]
	fn map_lines(games: impl Iterator<Item = Game>) -> impl Iterator<Item = usize> {
		games.map(|game| game.merge_max_draws().unwrap().power())
	}

	let sum: usize = map_lines(
		std::io::stdin()
			.lines()
			.map(|res| Game::from_str(&res.unwrap()).unwrap()),
	)
	.sum();
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	use super::*;

	const DRAW_1_1: Draw = Draw {
		blue: Some(3),
		red: Some(4),
		green: None,
	};
	const DRAW_1_2: Draw = Draw {
		red: Some(1),
		green: Some(2),
		blue: Some(6),
	};
	const DRAW_1_3: Draw = Draw {
		green: Some(2),
		red: None,
		blue: None,
	};
	const DRAW_2_1: Draw = Draw {
		blue: Some(1),
		green: Some(2),
		red: None,
	};
	const DRAW_2_2: Draw = Draw {
		green: Some(3),
		blue: Some(4),
		red: Some(1),
	};
	const DRAW_2_3: Draw = Draw {
		green: Some(1),
		blue: Some(1),
		red: None,
	};
	const DRAW_3_1: Draw = Draw {
		green: Some(8),
		blue: Some(6),
		red: Some(20),
	};
	const DRAW_3_2: Draw = Draw {
		blue: Some(5),
		red: Some(4),
		green: Some(13),
	};
	const DRAW_3_3: Draw = Draw {
		green: Some(5),
		red: Some(1),
		blue: None,
	};
	const DRAW_4_1: Draw = Draw {
		green: Some(1),
		red: Some(3),
		blue: Some(6),
	};
	const DRAW_4_2: Draw = Draw {
		green: Some(3),
		red: Some(6),
		blue: None,
	};
	const DRAW_4_3: Draw = Draw {
		green: Some(3),
		blue: Some(15),
		red: Some(14),
	};
	const DRAW_5_1: Draw = Draw {
		red: Some(6),
		blue: Some(1),
		green: Some(3),
	};
	const DRAW_5_2: Draw = Draw {
		blue: Some(2),
		red: Some(1),
		green: Some(2),
	};

	const MAX_DRAW_1: Draw = Draw {
		red: Some(4),
		green: Some(2),
		blue: Some(6),
	};
	const MAX_DRAW_2: Draw = Draw {
		red: Some(1),
		green: Some(3),
		blue: Some(4),
	};
	const MAX_DRAW_3: Draw = Draw {
		red: Some(20),
		green: Some(13),
		blue: Some(6),
	};
	const MAX_DRAW_4: Draw = Draw {
		red: Some(14),
		green: Some(3),
		blue: Some(15),
	};
	const MAX_DRAW_5: Draw = Draw {
		red: Some(6),
		green: Some(3),
		blue: Some(2),
	};

	#[test]
	fn draw_ord() {
		assert!(
			Draw::default()
				< Draw {
					red: Some(0),
					green: None,
					blue: None,
				}
		);
		assert!(DRAW_1_1 <= MAX_DRAW_1);
		assert!(DRAW_1_2 <= MAX_DRAW_1);
		assert!(DRAW_1_3 <= MAX_DRAW_1);
		assert!(DRAW_2_1 <= MAX_DRAW_2);
		assert!(DRAW_2_2 <= MAX_DRAW_2);
		assert!(DRAW_2_3 <= MAX_DRAW_2);
		assert!(DRAW_3_1 <= MAX_DRAW_3);
		assert!(DRAW_3_2 <= MAX_DRAW_3);
		assert!(DRAW_3_3 <= MAX_DRAW_3);
		assert!(DRAW_4_1 <= MAX_DRAW_4);
		assert!(DRAW_4_2 <= MAX_DRAW_4);
		assert!(DRAW_4_3 <= MAX_DRAW_4);
		assert!(DRAW_5_1 <= MAX_DRAW_5);
		assert!(DRAW_5_2 <= MAX_DRAW_5);
		assert!(MAX_DRAW_1 <= LIMIT);
		assert!(MAX_DRAW_2 <= LIMIT);
		assert!(MAX_DRAW_3 >= LIMIT);
		assert!(MAX_DRAW_4 >= LIMIT);
		assert!(MAX_DRAW_5 <= LIMIT);
	}

	#[cfg(feature = "p2")]
	#[test]
	fn draw_power() {
		assert_eq!(MAX_DRAW_1.power(), 48);
		assert_eq!(MAX_DRAW_2.power(), 12);
		assert_eq!(MAX_DRAW_3.power(), 1560);
		assert_eq!(MAX_DRAW_4.power(), 630);
		assert_eq!(MAX_DRAW_5.power(), 36);
	}

	#[test]
	fn game_from_str() {
		assert_eq!(
			Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
			Ok(Game {
				id: 1,
				draws: vec![DRAW_1_1, DRAW_1_2, DRAW_1_3],
			})
		);
		assert_eq!(
			Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
			Ok(Game {
				id: 2,
				draws: vec![DRAW_2_1, DRAW_2_2, DRAW_2_3],
			})
		);
		assert_eq!(
			Game::from_str(
				"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
			),
			Ok(Game {
				id: 3,
				draws: vec![DRAW_3_1, DRAW_3_2, DRAW_3_3],
			})
		);
		assert_eq!(
			Game::from_str(
				"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
			),
			Ok(Game {
				id: 4,
				draws: vec![DRAW_4_1, DRAW_4_2, DRAW_4_3],
			})
		);
		assert_eq!(
			Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
			Ok(Game {
				id: 5,
				draws: vec![DRAW_5_1, DRAW_5_2],
			})
		);
	}

	#[test]
	fn game_merge_max_draws() {
		assert_eq!(
			Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
				.unwrap()
				.merge_max_draws(),
			Some(MAX_DRAW_1)
		);
		assert_eq!(
			Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
				.unwrap()
				.merge_max_draws(),
			Some(MAX_DRAW_2)
		);
		assert_eq!(
			Game::from_str(
				"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
			)
			.unwrap()
			.merge_max_draws(),
			Some(MAX_DRAW_3)
		);
		assert_eq!(
			Game::from_str(
				"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
			)
			.unwrap()
			.merge_max_draws(),
			Some(MAX_DRAW_4)
		);
		assert_eq!(
			Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
				.unwrap()
				.merge_max_draws(),
			Some(MAX_DRAW_5)
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
