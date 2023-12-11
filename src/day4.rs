use std::collections::HashSet;
use std::collections::HashMap;
use aoc_util::parse_t::*;

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day4.txt").unwrap();

	let mut tot = 0;

	// let mut last_card
	let mut copies: HashMap<u32, u32> = HashMap::new();
	for s in data.split("\r\n") {
		let (card, mut num_s, mut tar_s) = parse_t!(s, "Card ", Trim<u32>, ": ", PassStr, " | ", PassStr, "").unwrap();
		let mut targets = HashSet::new();
		let mut found = 0u32;

		let cur_copies = copies.entry(card).or_default();
		*cur_copies += 1;
		let cur_copies = *cur_copies;
		
		// println!("card {}: {} | {}", card, num_s, tar_s);
		while !tar_s.is_empty() {
			tar_s = tar_s.trim();

			let mut _num = 0;
			if let Some((n, rest)) = tar_s.split_once(" ") {
				_num = n.parse::<u32>().unwrap();
				tar_s = rest;
			}
			else {
				_num = tar_s.parse::<u32>().unwrap();
				tar_s = "";
			}
			targets.insert(_num);
		}
		while !num_s.is_empty() {
			num_s = num_s.trim();

			let mut _num = 0u32;
			if let Some((n, rest)) = num_s.split_once(" ") {
				_num = n.parse::<u32>().unwrap();
				num_s = rest;
			}
			else {
				_num = num_s.parse::<u32>().unwrap();
				num_s = "";
			}
			if targets.contains(&_num) {
				found += 1;
			}
		}

		for i in 1..=found {
			*copies.entry(card + i).or_default() += cur_copies;
		}

		let score = match found {
			1 => 1,
			v if v > 1 => 2u32.pow(v - 1),
			_ => 0,
		};
		// println!("{} scores {} {}", card, score, found);
		tot += score;
	}
	println!("{}", tot); // 22193

	let tot: u32 = copies.values().sum();
	println!("{}", tot); // 5625994
}