use std::collections::HashMap;
use std::collections::VecDeque;
use aoc_util::array2d::*;
use itertools::Itertools;
use vek::num_traits::Float;


#[allow(dead_code)]
pub fn solve()
{
	let mut data = Array2D::load_file("data/day11.txt", |c| (c == '#', false));

	let mut galaxies = Vec::new();
	let mut idx = 0;
	let mut col_has_g = vec![false;data.width];
	for y in 0..data.height {
		let row_start = idx; 
		let mut has_g = false;
		for x in 0..data.width {
			if data.data[idx].0 {
				galaxies.push(Pos::new(x, y));
				has_g = true;
				col_has_g[x] = true;
			}
			idx += 1;
		}
		if !has_g {
			for i in row_start..idx {
				data.data[i].1 = true;
			}
		}
	}

	let mut idx = 0;
	for _y in 0..data.height {
		for x in 0..data.width {
			if !col_has_g[x] {
				data.data[idx].1 = true;
			}
			idx += 1;
		}
	}

	let mut tot = 0;
	let mut tot2 = 0;
	for pair in galaxies.iter().combinations(2) {
		let g0 = pair[0];
		let g1 = pair[1];

		let mut dist = 0;
		let mut dist2 = 0;
		let xs = g0.x.min(g1.x);
		let xe = g0.x.max(g1.x);
		for x in xs..xe {
			let exp = data.get(Pos::new(x, g0.y)).unwrap().1;
			dist += if exp {2} else {1};
			dist2 += if exp {1000000} else {1};
		}

		let ys = g0.y.min(g1.y);
		let ye = g0.y.max(g1.y);
		for y in ys..ye {
			let exp = data.get(Pos::new(g0.x, y)).unwrap().1;
			dist += if exp {2} else {1};
			dist2 += if exp {1000000} else {1};
		}
		// println!("{:?}", dist);
		tot += dist as u64;
		tot2 += dist2 as u64;
	}

	println!("{:?}", tot); // 10276166
	println!("{:?}", tot2); // 598693078798
}