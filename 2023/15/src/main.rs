use std::{io::Read, num::Wrapping};

fn hash(s: &str) -> u8 {
	s.bytes()
		.fold(Wrapping(0), |mut ret, b| {
			ret += b;
			ret *= 17;
			ret
		})
		.0
}

#[cfg(feature = "p2")]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct LensBox<'s>(Vec<(&'s str, u8)>);
#[cfg(feature = "p2")]
impl<'s> LensBox<'s> {
	fn insert(&mut self, label: &'s str, focal: u8) {
		if let Some(stored_focal) = self.0.iter_mut().find_map(|(stored_label, stored_focal)| {
			(label == *stored_label).then_some(stored_focal)
		}) {
			*stored_focal = focal;
		} else {
			self.0.push((label, focal));
		}
	}

	fn remove(&mut self, label: &'s str) {
		if let Some(i) = self
			.0
			.iter()
			.enumerate()
			.find_map(|(i, (stored_label, _focal))| (label == *stored_label).then_some(i))
		{
			self.0.remove(i);
		}
	}
}

#[cfg(not(feature = "p2"))]
fn main() {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let sum: usize = input.trim().split(',').map(|s| hash(s) as usize).sum();
	println!("{sum}");
}

#[cfg(feature = "p2")]
fn main() {
	let mut input = String::new();
	std::io::stdin().read_to_string(&mut input).unwrap();

	let mut boxes: [LensBox; 256] = std::array::from_fn(|_| LensBox::default());
	input
		.trim()
		.split(',')
		.for_each(|s| match s.split('=').collect::<Vec<_>>().as_slice() {
			[mut label] => {
				label = label.trim_end_matches('-');
				boxes[hash(label) as usize].remove(label);
			}
			[label, focal] => {
				boxes[hash(label) as usize].insert(label, focal.parse().unwrap());
			}
			_ => unreachable!(),
		});
	let sum: usize = boxes
		.into_iter()
		.enumerate()
		.flat_map(|(i, lens_box)| {
			lens_box
				.0
				.into_iter()
				.enumerate()
				.map(move |(j, (_label, focal))| (i + 1) * (j + 1) * focal as usize)
		})
		.sum();
	println!("{sum}");
}

#[cfg(test)]
mod tests {
	#[test]
	fn hash() {
		assert_eq!(super::hash("HASH"), 52);
		assert_eq!(super::hash("rn=1"), 30);
		assert_eq!(super::hash("cm-"), 253);
		assert_eq!(super::hash("qp=3"), 97);
		assert_eq!(super::hash("cm=2"), 47);
		assert_eq!(super::hash("qp-"), 14);
		assert_eq!(super::hash("pc=4"), 180);
		assert_eq!(super::hash("ot=9"), 9);
		assert_eq!(super::hash("ab=5"), 197);
		assert_eq!(super::hash("pc-"), 48);
		assert_eq!(super::hash("pc=6"), 214);
		assert_eq!(super::hash("ot=7"), 231);
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
