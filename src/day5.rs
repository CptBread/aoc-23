// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::ops::RangeBounds;
use aoc_util::parse_t::*;

#[allow(dead_code)]
pub fn solve()
{
	let timer =std::time::Instant::now();
	
	let data = std::fs::read_to_string("data/day5.txt").unwrap();

	let (mut ids, rest) = parse_t!(data, "seeds: ", Seperated<u64, ' '>, "\r\n\r\n", PassStr, "").unwrap();

	let mut id_ranges = Vec::new();
	for chunk in ids.chunks(2) {
		id_ranges.push(chunk[0]..(chunk[0]+chunk[1]));
	}


	let mut next = Vec::new();
	for group in rest.split("\r\n\r\n") {
		if !next.is_empty() {
			ids.append(&mut next);
			// println!("{:?}", ids);
		}
		for line in group.split("\r\n") {
			if line.contains(':') {
				continue;
			}
			let (next_start, id_start, num) = parse_t!(line, u64, " ", u64, " ", u64, "").unwrap();
			ids.retain(|v| {
				if let Some(v) = v.checked_sub(id_start) {
					if v < num {
						next.push(next_start + v);
						return false;
					}
				}
				true
			});
		}
	}
	if !next.is_empty() {
		ids.append(&mut next);
		println!("{:?}", ids);
	}
	println!("{:?}", ids.iter().min()); // 57075758

	let clear_empty = |vec: &mut Vec::<core::ops::Range<u64>>| {
		vec.retain(|v| {
			v.start != v.end
		});
	};


	// println!("{:?}", id_ranges);
	let mut next = Vec::new();
	for group in rest.split("\r\n\r\n") {
		// if let Some(m) = id_ranges.iter().map(|v| v.start).min() {
		// 	println!("{:?}", m - 1);
		// }
		for line in group.split("\r\n") {
			if line.contains(':') {
				continue;
			}
			let (next_start, id_start, num) = parse_t!(line, u64, " ", u64, " ", u64, "").unwrap();
			let source_range = id_start..(id_start + num);
			// println!("{:?} -> {:?}", source_range, next_start..(next_start + num));
			let mut extra = Vec::new();
			id_ranges.retain_mut(|v| {
				let cover_start = source_range.contains(&v.start);
				let cover_end = source_range.contains(&v.end);

				let start = if cover_start {(v.start - source_range.start) + next_start} else {next_start};
				let end = if cover_end {(v.end - source_range.start) + next_start} else {next_start + num};

				if cover_start && cover_end {
					// println!("org {:?}", v);
					next.push(start..end);
					// println!("ca {:?} - no left", next.last().unwrap());
					return false;
				}
				else if cover_start {
					// println!("org {:?}", v);
					next.push(start..end);
					*v = source_range.end..v.end;
					// println!("cs {:?} *{:?}", next.last().unwrap(), *v);
				}
				else if cover_end {
					// println!("org {:?}", v);
					next.push(start..end);
					*v = v.start..id_start;
					// println!("ce {:?} *{:?}", next.last().unwrap(), *v);
				}
				else if v.contains(&id_start) {
					// println!("org {:?}", v);
					next.push(start..end);
					extra.push(source_range.end..v.end);
					*v = v.start..id_start;
					// println!("cm *{:?} {:?} *{:?}", *v, next.last().unwrap(), extra.last().unwrap());
				}
				true
			});
			clear_empty(&mut extra);
			id_ranges.append(&mut extra);
			// // Error check
			// id_ranges.iter().for_each(|v| {
			// 	assert!(v.start <= v.end);
			// });
			// next.iter().for_each(|v| {
			// 	assert!(v.start <= v.end);
			// });
		}
		if !next.is_empty() {
			clear_empty(&mut id_ranges);
			clear_empty(&mut next);
			id_ranges.append(&mut next);
			println!("{:?}", id_ranges);
			// println!("NEXT! <------------------------");
		}
	}

	
	println!("{:?}", id_ranges.iter().map(|v| v.start).min()); // 31161857
	println!("{}", timer.elapsed().as_secs_f32());
}