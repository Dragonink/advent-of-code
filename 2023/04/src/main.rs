use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ScratchCard {
	winning: HashSet<u8>,
	random: HashSet<u8>,
}
impl FromStr for ScratchCard {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut numbers = s
			.split(':')
			.nth(1)
			.ok_or("missing card numbers")?
			.split('|');
		let parse_numbers = |s: &str| {
			s.split_ascii_whitespace()
				.map(|s| s.parse().map_err(|_err| "could not parse a number"))
				.collect::<Result<_, _>>()
		};

		let winning = numbers
			.next()
			.ok_or("missing the winning numbers")
			.and_then(parse_numbers)?;
		let random = numbers
			.next()
			.ok_or("missing the random numbers")
			.and_then(parse_numbers)?;

		Ok(Self { winning, random })
	}
}
impl ScratchCard {
	fn winning_random_numbers(&self) -> impl '_ + Iterator<Item = u8> {
		self.random.intersection(&self.winning).copied()
	}
}

fn main() {
	#[cfg(not(feature = "p2"))]
	fn map_lines(cards: impl Iterator<Item = ScratchCard>) -> impl Iterator<Item = usize> {
		cards.map(|card| {
			let count = card.winning_random_numbers().count();
			if count == 0 {
				0
			} else {
				1 << (count - 1)
			}
		})
	}

	#[cfg(feature = "p2")]
	fn map_lines(cards: impl Iterator<Item = ScratchCard>) -> impl Iterator<Item = usize> {
		cards
			.enumerate()
			.fold(Vec::new(), |mut card_counts, (i, card)| {
				let increment_count = |counts: &mut Vec<usize>, i, by| {
					if let Some(count) = counts.get_mut(i) {
						*count += by;
					} else {
						counts.push(by);
					}
				};

				increment_count(&mut card_counts, i, 1);
				let by = card_counts[i];
				for j in 1..=card.winning_random_numbers().count() {
					increment_count(&mut card_counts, i + j, by);
				}

				card_counts
			})
			.into_iter()
	}

	let sum: usize = map_lines(
		std::io::stdin()
			.lines()
			.map(|res| ScratchCard::from_str(&res.unwrap()).unwrap()),
	)
	.sum();
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn scratchcard_from_str() {
		assert_eq!(
			ScratchCard::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
			Ok(ScratchCard {
				winning: HashSet::from([41, 48, 83, 86, 17]),
				random: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
			})
		);
		assert_eq!(
			ScratchCard::from_str("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
			Ok(ScratchCard {
				winning: HashSet::from([13, 32, 20, 16, 61]),
				random: HashSet::from([61, 30, 68, 82, 17, 32, 24, 19]),
			})
		);
		assert_eq!(
			ScratchCard::from_str("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
			Ok(ScratchCard {
				winning: HashSet::from([1, 21, 53, 59, 44]),
				random: HashSet::from([69, 82, 63, 72, 16, 21, 14, 1]),
			})
		);
		assert_eq!(
			ScratchCard::from_str("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
			Ok(ScratchCard {
				winning: HashSet::from([41, 92, 73, 84, 69]),
				random: HashSet::from([59, 84, 76, 51, 58, 5, 54, 83]),
			})
		);
		assert_eq!(
			ScratchCard::from_str("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
			Ok(ScratchCard {
				winning: HashSet::from([87, 83, 26, 28, 32]),
				random: HashSet::from([88, 30, 70, 12, 93, 22, 82, 36]),
			})
		);
		assert_eq!(
			ScratchCard::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
			Ok(ScratchCard {
				winning: HashSet::from([31, 18, 13, 56, 72]),
				random: HashSet::from([74, 77, 10, 23, 35, 67, 36, 11]),
			})
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
