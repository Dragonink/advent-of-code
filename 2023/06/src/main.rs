use std::{io::Read, str::FromStr};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Races(Vec<(usize, usize)>);
impl PartialEq<Vec<(usize, usize)>> for Races {
	#[inline]
	fn eq(&self, other: &Vec<(usize, usize)>) -> bool {
		self.0 == *other
	}
}
impl FromStr for Races {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		macro_rules! parse {
			($numbers:expr) => {{
				#[cfg(not(feature = "p2"))]
				{
					$numbers.map(|s| s.parse().map_err(|_err| "could not parse a number"))
				}
				#[cfg(feature = "p2")]
				{
					vec![$numbers
						.collect::<String>()
						.parse::<usize>()
						.map_err(|_err| "could not parse a number")]
					.into_iter()
				}
			}};
		}

		let mut lines = s.lines();
		let times = parse!(lines
			.next()
			.ok_or("missing times")?
			.split(':')
			.skip(1)
			.flat_map(|s| s.split_ascii_whitespace()));
		let distances = parse!(lines
			.next()
			.ok_or("missing distances")?
			.split(':')
			.skip(1)
			.flat_map(|s| s.split_ascii_whitespace()));

		times
			.zip(distances)
			.map(|(time_res, distance_res)| Ok((time_res?, distance_res?)))
			.collect::<Result<_, _>>()
			.map(Self)
	}
}
impl IntoIterator for Races {
	type Item = (usize, usize);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let product: usize = Races::from_str(&input)
		.unwrap()
		.into_iter()
		.map(|(time, distance)| {
			let time = time as f64;
			let distance = distance as f64;

			let discriminant = (time.powi(2) - 4. * distance).sqrt();
			let min = ((-time + discriminant) / -2. + 1.).floor() as usize;
			let max = ((-time - discriminant) / -2. - 1.).ceil() as usize;
			max - min + 1
		})
		.product();
	println!("{product}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(not(feature = "p2"))]
	#[test]
	fn races_from_str() {
		assert_eq!(
			Races::from_str(include_str!("test.txt")),
			Ok(Races(vec![(7, 9), (15, 40), (30, 200)]))
		);
	}

	#[cfg(feature = "p2")]
	#[test]
	fn races_from_str() {
		assert_eq!(
			Races::from_str(include_str!("test.txt")),
			Ok(Races(vec![(71530, 940200)]))
		);
	}
}
