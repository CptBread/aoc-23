use std::collections::HashMap;
use aoc_util::parse_t::*;

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day2.txt").unwrap();

	let mut tot = 0;
	let limit = HashMap::from([
		("red", 12),
		("green", 13),
		("blue", 14),
	]);
	'game: for s in data.split("\r\n") {
		let (game, s) = parse_t!(s, "Game ", u32, ": ", PassStr, "").unwrap();
		for s in s.split("; ") {
			for s in s.split(", ") {
				let (n, col) = parse_t!(s, u32, " ", PassStr, "").unwrap();
				if limit.get(col).copied().unwrap_or(n) < n {
					continue 'game;
				}
			}
		}
		// println!("{}", game);
		tot += game;
	}
	println!("{:?}", tot); // 2283

	let mut tot = 0;
	for s in data.split("\r\n") {
		let mut needed = HashMap::new();
		let (_game, s) = parse_t!(s, "Game ", u32, ": ", PassStr, "").unwrap();
		for s in s.split("; ") {
			for s in s.split(", ") {
				let (n, col) = parse_t!(s, u32, " ", PassStr, "").unwrap();
				let need = needed.entry(col).or_default();
				*need = u32::max(*need, n);
			}
		}
		let pow = needed.values().fold(1, |res, v| res * (*v));
		// println!("{} {}", _game, pow);
		tot += pow;
	}
	println!("{:?}", tot); // 78669
}