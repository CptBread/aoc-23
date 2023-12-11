use std::collections::HashMap;
use std::collections::VecDeque;
use aoc_util::array2d::*;
use itertools::Itertools;


#[allow(dead_code)]
pub fn solve()
{
	let mut data = Array2D::load_file("data/day10.txt", |c| {
		match c {
			'7' => '┐',
			'J' => '┘',
			'F' => '┌',
			'L' => '└',
			_ => c,
		}
	});
	let mut visited = HashMap::new();
	let mut to_visit = VecDeque::new();
	let start = data.idx_to_pos(data.data.iter().copied().find_position(|c| *c == 'S').unwrap().0);
	
	let mut maybe_add_around_start = |xoff, yoff, allow: Vec<char>| {
		if let Some(p) = data.pos_offset(start, xoff, yoff) {
			let c = data.get(p).unwrap();
			if allow.contains(c) {
				to_visit.push_front((p, c, 1, start))
			}
		}
	};
	
	maybe_add_around_start(0 - 1, 0, vec!['┌', '└', '-']);
	maybe_add_around_start(0 + 1, 0, vec!['┐', '┘', '-']);
	maybe_add_around_start(0, 0 - 1, vec!['┐', '┌', '|']);
	maybe_add_around_start(0, 0 + 1, vec!['┘', '└', '|']);

	while let Some((pos, c, step, from)) = to_visit.pop_back() {
		let mut stop = false;
		visited.entry(pos).and_modify(|v| {
			println!("{} {} {}", v, step, pos);
			assert!(*v >= step);
			stop = true;
		}).or_insert(step);

		if stop {
			let mut idx = 0;
			data.print(|v| {
				let pos = data.idx_to_pos(idx);
				idx += 1;
				if !visited.contains_key(&pos) {
					return '.';
				}
				*v
			});
			break;
		}

		// let mut idx = 0;
		// data.print(|v| {
		// 	let pos = data.idx_to_pos(idx);
		// 	idx += 1;
		// 	if visited.contains_key(&pos) {
		// 		return '*';
		// 	}
		// 	*v
		// });
		
		let neigh = match c {
			'┐' => [data.pos_offset(pos, 0 - 1, 0), data.pos_offset(pos, 0, 0 + 1)],
			'┘' => [data.pos_offset(pos, 0 - 1, 0), data.pos_offset(pos, 0, 0 - 1)],
			'┌' => [data.pos_offset(pos, 0 + 1, 0), data.pos_offset(pos, 0, 0 + 1)],
			'└' => [data.pos_offset(pos, 0 + 1, 0), data.pos_offset(pos, 0, 0 - 1)],
			'|' => [data.pos_offset(pos, 0, 0 - 1), data.pos_offset(pos, 0, 0 + 1)],
			'-' => [data.pos_offset(pos, 0 + 1, 0), data.pos_offset(pos, 0 - 1, 0)],
			'S' => [None, None],
			c => unreachable!("{} {} {}", c, pos, from)
		};
		for n in neigh.iter().filter_map(|v| *v) {
			if n != from {
				to_visit.push_front((n, data.get(n).unwrap(), step + 1, pos));
			}
		}
	}

	// This could fail if S is in one of the corners because we expand S into a cross
	let mut big = Array2D::new(data.width * 3, data.height * 3, '.');
	data.for_each_mut(|p, c| {
		if visited.contains_key(&p) || *c == 'S' {
			// l, r, u, d
			let add = match c {
				'S' => [true, true, true, true],
				'┐' => [true, false, false, true],
				'┘' => [true, false, true, false],
				'┌' => [false, true, false, true],
				'└' => [false, true, true, false],
				'|' => [false, false, true, true],
				'-' => [true, true, false, false],
				_ => unreachable!()
			};
			let center = p * 3 + Pos::new(1, 1);
			*big.get_mut(center).unwrap() = *c;
			for (idx, p) in 
				big.neighbours(center).iter().enumerate().zip_eq(add)
				.filter_map(|((idx, v), add)| if add {v.map(|v| (idx, v))} else {None}) 
			{
				let pipe = if idx > 1 {'|'} else {'-'};
				*big.get_mut(p).unwrap() = pipe;
			}
		}
	});

	// big.print(|c| *c);

	*big.get_mut(Pos::new(0, 0)).unwrap() = 'O';
	let mut to_visit = VecDeque::from([Pos::new(0, 0)]);	
	while let Some(pos) = to_visit.pop_back() {
		for n in big.neighbours(pos).iter().filter_map(|v| *v) {
			let c = big.get_mut(n).unwrap();
			if *c == '.' {
				*c = 'O';
				to_visit.push_front(n);
			}
		}
	}
	big.print(|c| *c);

	let mut small = Array2D::new(data.width, data.height, '.');
	let mut count = 0;
	big.for_each_mut(|p, v| {
		if p.x % 3 == 1 && p.y % 3 == 1 {
		// if p.x % 3 == 1 && p.y % 3 == 1 && *v == '.' {
			*small.get_mut(p / 3).unwrap() = *v;
			// count += 1;
		}
	});
	small.print(|c| *c);

	println!("{}", count);
}