// use std::collections::HashSet;
use std::collections::HashMap;
// use std::ops::RangeBounds;
use aoc_util::parse_t::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank{
	Card,
	Pair,
	TwoPair,
	Three,
	Full,
	Four,
	Five,
}

#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day7.txt").unwrap();

	let mut plays = Vec::new();
	for l in data.lines() {
		let (mut hand, bet) = parse_t!(l, Chars<5>, " ", u64, "").unwrap();
		let mut set = HashMap::<char, u32>::new();

		for c in hand.iter_mut() {
			*c = match *c {
				'A' => 'Z',
				'K' => 'X',
				'Q' => 'W',
				'J' => 'U',
				'T' => 'T',
				c => c,
			}
		}

		for c in hand.iter().copied() {
			*set.entry(c).or_default() += 1;
		}
		let n = *set.values().max().unwrap();

		let rank = match (set.len(), n) {
			(1, _) => Rank::Five,
			(2, 4) => Rank::Four,
			(2, 3) => Rank::Full,
			(2, _) => unreachable!(),
			(3, 3) => Rank::Three,
			(3, 2) => Rank::TwoPair,
			(3, _) => unreachable!(),
			(4, 2) => Rank::Pair,
			(4, _) => unreachable!(),
			(5, 1) => Rank::Card,
			_ => unreachable!(),
		};
		println!("{:?} {:?} {}", hand, rank, bet);
		plays.push((rank, hand, bet));
	}
	plays.sort_by(|(l_rank, l_hand, _), (r_rank, r_hand, _)| {
		l_rank.cmp(r_rank).then(l_hand.cmp(r_hand))
	});
	// println!("{:?}", plays);
	let mut tot = 0;
	for (idx, (_, _, bet)) in plays.iter().enumerate() {
		tot += (idx as u64 + 1) * bet;
	}
	println!("{:?}", tot);


	let mut plays = Vec::new();
	for l in data.lines() {
		let (mut hand, bet) = parse_t!(l, Chars<5>, " ", u64, "").unwrap();
		let mut set = HashMap::<char, u32>::new();

		for c in hand.iter_mut() {
			*c = match *c {
				'A' => 'Z',
				'K' => 'X',
				'Q' => 'W',
				'J' => '0',
				'T' => 'T',
				c => c,
			}
		}

		for c in hand.iter().copied() {
			*set.entry(c).or_default() += 1;
		}
		let j = set.remove(&'0').unwrap_or(0);
		let n = *set.values().max().unwrap_or(&0) + j;

		let rank = match (set.len(), n) {
			(0, _) => Rank::Five, // All J hand
			(1, _) => Rank::Five,
			(2, 4) => Rank::Four,
			(2, 3) => Rank::Full,
			(2, _) => unreachable!(),
			(3, 3) => Rank::Three,
			(3, 2) => Rank::TwoPair,
			(3, _) => unreachable!(),
			(4, 2) => Rank::Pair,
			(4, _) => unreachable!(),
			(5, 1) => Rank::Card,
			_ => unreachable!(),
		};
		println!("{:?} {:?} {}", hand, rank, bet);
		plays.push((rank, hand, bet));
	}
	plays.sort_by(|(l_rank, l_hand, _), (r_rank, r_hand, _)| {
		l_rank.cmp(r_rank).then(l_hand.cmp(r_hand))
	});
	// println!("{:?}", plays);

	let mut tot = 0;
	for (idx, (_, _, bet)) in plays.iter().enumerate() {
		tot += (idx as u64 + 1) * bet;
	}
	println!("{:?}", tot);
}