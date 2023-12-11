// use std::collections::HashSet;
use std::{collections::HashMap};
// use std::ops::RangeBounds;
use aoc_util::parse_t::*;
use itertools::Itertools;


#[allow(dead_code)]
pub fn solve()
{
	let data = std::fs::read_to_string("data/day8.txt").unwrap();

	let (inst, data) = parse_t!(data, PassStr, "\r\n\r\n", PassStr, "").unwrap();
	let mut nodes = HashMap::new();
	for l in data.lines() {
		// println!("{}", l);
		let (node, l, r) = parse_t!(l, PassStr, " = (", PassStr, ", ", PassStr, ")").unwrap();
		nodes.insert(node, (l, r));
	}

	let mut at = "AAA";
	for (step, inst) in inst.chars().cycle().enumerate() {
		if at == "ZZZ" {
			println!("{}", step);
			break;
		}
		let _was = at;
		let node = nodes[at];
		match inst {
			'L' => at = node.0,
			'R' => at = node.1,
			_ => unreachable!(),
		}
		// println!("{}({}) -> {}", _was, inst, at);
	}

	// let mut loops = HashMap::new();
	let mut next_z = HashMap::new();
	let starts = nodes.keys().copied().filter(|n| n.ends_with('A')).collect_vec();
	for ghost in starts.iter().copied() {
		// let mut past = Vec::from([(ghost, 0, 0)]);
		let mut first_seen = HashMap::new();
		let mut past: Vec<(&str, usize, usize)> = Vec::new();
		let mut at: &str = ghost;
		// let mut last_rel = 0;
		// let mut last_step = 0;
		for (step, (rel, inst)) in inst.chars().enumerate().cycle().enumerate() {
			// last_rel = rel;
			// last_step = step;
			if at.ends_with('Z') {
				for p in past.iter().copied() {
					// println!("{:?} {:?}", p, (at, step - p.2));
					if let Some(_last) = next_z.insert((p.0, p.1), (at, step - p.2)) {
						// println!("Oh shit");
						// println!("Oh shit {:?} -> {:?}", _last, (at, step - p.2));
						// unreachable!();
						assert!(_last == (at, step - p.2));
					}
				}
				if next_z.contains_key(&(at, rel)) {
					println!("Started at {} found looping after {} steps", ghost, step);
					break;
				}
				past.clear();
			}
			past.push((at, rel, step));
			first_seen.entry((at, rel)).or_insert(step);
			// let was = at;
			let node = nodes[at];
			match inst {
				'L' => at = node.0,
				'R' => at = node.1,
				_ => unreachable!(),
			}
		}

		// println!("{:?} {} {}", first_seen[&(at, last_rel)], last_rel, last_step);
		// loops.entry(ghost).or_insert_with(||{
		// 	let loop_start = first_seen[&(at, last_rel)];
		// 	let size = last_step - loop_start;
		// 	let ends = {
		// 		let l_at = loop_start % inst.len();
		// 		// loop 
		// 		0
		// 	};
		// 	(loop_start, size, ends)
		// });
	}
	
	// println!("{:?}", next_z);

	println!("Starting heavy work...");

	let mut steps = 0usize;
	let mut rel = 0usize;
	let mut ghosts = starts.iter().map(|n| {
		let next = next_z[&(*n, 0)];
		(*n, next)
	}).collect_vec();
	// println!("{:?}", ghosts);
	loop {
		let step_size = ghosts.iter().map(|(_, (_, to_next))| to_next).min().unwrap().clone();
		let next_rel = (rel + step_size) % inst.len();

		// println!("{}", step_size);

		let mut all_z = true;
		for (node, next) in ghosts.iter_mut() {
			let n_left = next.1 - step_size;
			if n_left == 0 {
				*node = next.0;
				*next = next_z[&(*node, next_rel)];
			}
			else {
				all_z = false;
				next.1 = n_left;
			}

		}

		steps += step_size;
		rel = next_rel;
		if all_z {
			break;
		}
	}
	println!("{}", steps); // 11188774513823
}