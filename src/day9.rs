// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::ops::RangeBounds;
use aoc_util::parse_t::*;
use itertools::Itertools;


#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day9.txt").unwrap();
	let mut tot = 0;
	let mut tot2 = 0;
	for l in data.lines() {
		let values = parse_t!(l, Seperated<i32, ' '>, "").unwrap();
		// println!("{:?}", values);
		let mut last_nums = Vec::from([*values.last().unwrap()]);
		let mut first_nums = Vec::from([*values.first().unwrap()]);
		let mut curr = values;
		loop {
			// println!("{:?}", curr);
			let diffs = curr.iter().copied().tuple_windows::<(i32, i32)>().map(|(v0, v1)| v1 - v0).collect_vec();
			if diffs.iter().all(|v| *v == 0) {
				let lsum = last_nums.iter().copied().sum::<i32>();
				tot += lsum;
				let rsum = first_nums.iter().rev().fold(0, |diff, v| {
					// println!("{} {} {}", v, diff, v - diff);
					v - diff
				});
				tot2 += rsum;
				println!("{} {}", lsum, rsum);
				break;
			}
			else {
				last_nums.push(*diffs.last().unwrap());
				first_nums.push(*diffs.first().unwrap());
				curr = diffs;
			}
		}
	}
	println!("{}", tot);
	println!("{}", tot2);
}