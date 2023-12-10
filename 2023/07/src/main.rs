use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
	#[cfg(feature = "p2")]
	Joker,
	Two = 2,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	#[cfg(not(feature = "p2"))]
	Jack,
	Queen,
	King,
	Ace,
}
impl FromStr for Card {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			#[cfg(feature = "p2")]
			"J" => Ok(Self::Joker),
			"2" => Ok(Self::Two),
			"3" => Ok(Self::Three),
			"4" => Ok(Self::Four),
			"5" => Ok(Self::Five),
			"6" => Ok(Self::Six),
			"7" => Ok(Self::Seven),
			"8" => Ok(Self::Eight),
			"9" => Ok(Self::Nine),
			"T" => Ok(Self::Ten),
			#[cfg(not(feature = "p2"))]
			"J" => Ok(Self::Jack),
			"Q" => Ok(Self::Queen),
			"K" => Ok(Self::King),
			"A" => Ok(Self::Ace),
			_ => Err("invalid card"),
		}
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum HandType {
	#[default]
	HighCard,
	OnePair,
	TwoPairs,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
	order: [Card; 5],
	counts: HashMap<Card, usize>,
}
impl FromStr for Hand {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut order = Vec::with_capacity(5);
		let mut counts = HashMap::new();

		for b in s.bytes() {
			let card = Card::from_str(&(b as char).to_string())?;
			order.push(card);
			*counts.entry(card).or_insert(0) += 1;
		}
		#[cfg(feature = "p2")]
		if counts.get(&Card::Joker).copied().unwrap_or_default() < 5 {
			if let Some(jokers) = counts.remove(&Card::Joker) {
				*counts.values_mut().max().unwrap() += jokers;
			}
		}

		Ok(Self {
			order: order
				.try_into()
				.map_err(|_err| "the hand does not contain exactly 5 cards")?,
			counts,
		})
	}
}
impl Hand {
	fn hand_type(&self) -> HandType {
		match self.counts.len() {
			1 => HandType::FiveOfAKind,
			2 => self
				.counts
				.values()
				.find(|count| **count == 4)
				.map_or(HandType::FullHouse, |_| HandType::FourOfAKind),
			3 => self
				.counts
				.values()
				.find(|count| **count == 3)
				.map_or(HandType::TwoPairs, |_| HandType::ThreeOfAKind),
			4 => HandType::OnePair,
			5 => HandType::HighCard,
			_ => unreachable!(),
		}
	}
}
impl PartialOrd for Hand {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		self.hand_type()
			.cmp(&other.hand_type())
			.then_with(|| self.order.cmp(&other.order))
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandBid(Hand, usize);
impl FromStr for HandBid {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut values = s.split_ascii_whitespace();
		let hand = values
			.next()
			.ok_or("missing hand")
			.and_then(Hand::from_str)?;
		let bid = values
			.next()
			.ok_or("missing bid")
			.and_then(|value| value.parse().map_err(|_err| "could not parse bid"))?;

		Ok(Self(hand, bid))
	}
}
impl PartialOrd for HandBid {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
impl Ord for HandBid {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0)
	}
}

fn main() {
	let mut bids = std::io::stdin()
		.lines()
		.map(|res| HandBid::from_str(&res.unwrap()).unwrap())
		.collect::<Vec<_>>();
	bids.sort();
	let total: usize = bids
		.into_iter()
		.enumerate()
		.map(|(rank, bid)| bid.1 * (rank + 1))
		.sum();
	println!("{total}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hand_from_str() {
		assert_eq!(
			Hand::from_str("AAAAA"),
			Ok(Hand {
				order: [Card::Ace; 5],
				counts: HashMap::from([(Card::Ace, 5)])
			})
		);
		assert_eq!(
			Hand::from_str("AA8AA"),
			Ok(Hand {
				order: [Card::Ace, Card::Ace, Card::Eight, Card::Ace, Card::Ace],
				counts: HashMap::from([(Card::Ace, 4), (Card::Eight, 1)])
			})
		);
		assert_eq!(
			Hand::from_str("23332"),
			Ok(Hand {
				order: [Card::Two, Card::Three, Card::Three, Card::Three, Card::Two],
				counts: HashMap::from([(Card::Two, 2), (Card::Three, 3)])
			})
		);
		assert_eq!(
			Hand::from_str("TTT98"),
			Ok(Hand {
				order: [Card::Ten, Card::Ten, Card::Ten, Card::Nine, Card::Eight],
				counts: HashMap::from([(Card::Ten, 3), (Card::Nine, 1), (Card::Eight, 1)])
			})
		);
		assert_eq!(
			Hand::from_str("23432"),
			Ok(Hand {
				order: [Card::Two, Card::Three, Card::Four, Card::Three, Card::Two],
				counts: HashMap::from([(Card::Two, 2), (Card::Three, 2), (Card::Four, 1)])
			})
		);
		assert_eq!(
			Hand::from_str("A23A4"),
			Ok(Hand {
				order: [Card::Ace, Card::Two, Card::Three, Card::Ace, Card::Four],
				counts: HashMap::from([
					(Card::Ace, 2),
					(Card::Two, 1),
					(Card::Three, 1),
					(Card::Four, 1)
				])
			})
		);
		assert_eq!(
			Hand::from_str("23456"),
			Ok(Hand {
				order: [Card::Two, Card::Three, Card::Four, Card::Five, Card::Six],
				counts: HashMap::from([
					(Card::Two, 1),
					(Card::Three, 1),
					(Card::Four, 1),
					(Card::Five, 1),
					(Card::Six, 1),
				])
			})
		);
	}

	#[test]
	fn hand_type() {
		assert_eq!(
			Hand::from_str("AAAAA").unwrap().hand_type(),
			HandType::FiveOfAKind
		);
		assert_eq!(
			Hand::from_str("AA8AA").unwrap().hand_type(),
			HandType::FourOfAKind
		);
		assert_eq!(
			Hand::from_str("23332").unwrap().hand_type(),
			HandType::FullHouse
		);
		assert_eq!(
			Hand::from_str("TTT98").unwrap().hand_type(),
			HandType::ThreeOfAKind
		);
		assert_eq!(
			Hand::from_str("23432").unwrap().hand_type(),
			HandType::TwoPairs
		);
		assert_eq!(
			Hand::from_str("A23A4").unwrap().hand_type(),
			HandType::OnePair
		);
		assert_eq!(
			Hand::from_str("23456").unwrap().hand_type(),
			HandType::HighCard
		);

		#[cfg(feature = "p2")]
		assert_eq!(
			Hand::from_str("QJJQ2").unwrap().hand_type(),
			HandType::FourOfAKind
		);
	}

	#[test]
	fn hand_ord() {
		assert!(Hand::from_str("33332").unwrap() > Hand::from_str("2AAAA").unwrap());
		assert!(Hand::from_str("77888").unwrap() > Hand::from_str("77788").unwrap());

		#[cfg(feature = "p2")]
		assert!(Hand::from_str("JKKK2").unwrap() < Hand::from_str("QQQQ2").unwrap());
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
