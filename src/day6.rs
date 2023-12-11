// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::ops::RangeBounds;

#[allow(dead_code)]
pub fn solve()
{
	// let data = std::fs::read_to_string("data/day6.txt").unwrap();

	// let target = [(7, 9), (15, 40), (30, 200),];
	let target = [(53, 313), (89, 1090), (76, 1214), (98, 1201),];

	let mut res = 1;
	for (time, t_dist) in target.iter().copied() {
		let mut opt = 0;
		for hold in 0..time {
			let our_dist = hold * (time - hold);
			if our_dist > t_dist {
				opt += 1;
			}
		}
		println!("{:?}", opt);
		res *= opt;
	}
	println!("{:?}", res);

	// let (time, t_dist) = (71530u64, 940200u64);
	// 13.14
	// 71517.83
	// let min  = 14;
	// let max = 71517;

	// let (time, t_dist) = (53897698u64, 313109012141201u64);
	// let max = x * (time - x) = x*time - x^2 
	// wa x * (53897698 - x) = 313109012141201
	// ~6623213.7
	// ~47274484.3
	let min  = 6_623_214;
	let max = 47_274_484;

	// 40651270 to low 

	println!("{}", max - min);
}