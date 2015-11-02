#[macro_use]
extern crate lazy_static;

extern crate nalgebra;

use nalgebra::DMat;

lazy_static! {

	// taken from https://github.com/seqan/seqan/blob/master/include%2Fseqan%2Fscore%2Fscore_matrix_data.h#L327
	static ref ARRAY: [i32;729]=[
		 4, -2,  0, -2, -1, -2,  0, -2, -1, -1, -1, -1, -1, -2,  0, -1, -1, -1,  1,  0,  0,  0, -3, -2, -1,  0, -4,
		-2,  4, -3,  4,  1, -3, -1,  0, -3, -4,  0, -4, -3,  3, -1, -2,  0, -1,  0, -1, -1, -3, -4, -3,  1, -1, -4,
		 0, -3,  9, -3, -4, -2, -3, -3, -1, -1, -3, -1, -1, -3, -2, -3, -3, -3, -1, -1, -2, -1, -2, -2, -3, -2, -4,
		-2,  4, -3,  6,  2, -3, -1, -1, -3, -4, -1, -4, -3,  1, -1, -1,  0, -2,  0, -1, -1, -3, -4, -3,  1, -1, -4,
		-1,  1, -4,  2,  5, -3, -2,  0, -3, -3,  1, -3, -2,  0, -1, -1,  2,  0,  0, -1, -1, -2, -3, -2,  4, -1, -4,
		-2, -3, -2, -3, -3,  6, -3, -1,  0,  0, -3,  0,  0, -3, -1, -4, -3, -3, -2, -2, -1, -1,  1,  3, -3, -1, -4,
		 0, -1, -3, -1, -2, -3,  6, -2, -4, -4, -2, -4, -3,  0, -1, -2, -2, -2,  0, -2, -1, -3, -2, -3, -2, -1, -4,
		-2,  0, -3, -1,  0, -1, -2,  8, -3, -3, -1, -3, -2,  1, -1, -2,  0,  0, -1, -2, -1, -3, -2,  2,  0, -1, -4,
		-1, -3, -1, -3, -3,  0, -4, -3,  4,  3, -3,  2,  1, -3, -1, -3, -3, -3, -2, -1, -1,  3, -3, -1, -3, -1, -4,
		-1, -4, -1, -4, -3,  0, -4, -3,  3,  3, -3,  3,  2, -3, -1, -3, -3, -3, -2, -1, -1,  2, -3, -1, -3, -1, -4,
		-1,  0, -3, -1,  1, -3, -2, -1, -3, -3,  5, -2, -1,  0, -1, -1,  1,  2,  0, -1, -1, -2, -3, -2,  1, -1, -4,
		-1, -4, -1, -4, -3,  0, -4, -3,  2,  3, -2,  4,  2, -3, -1, -3, -2, -2, -2, -1, -1,  1, -2, -1, -3, -1, -4,
		-1, -3, -1, -3, -2,  0, -3, -2,  1,  2, -1,  2,  5, -2, -1, -2,  0, -1, -1, -1, -1,  1, -1, -1, -1, -1, -4,
		-2,  3, -3,  1,  0, -3,  0,  1, -3, -3,  0, -3, -2,  6, -1, -2,  0,  0,  1,  0, -1, -3, -4, -2,  0, -1, -4,
		 0, -1, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -2, -1, -1,  0,  0, -1, -1, -2, -1, -1, -1, -4,
		-1, -2, -3, -1, -1, -4, -2, -2, -3, -3, -1, -3, -2, -2, -2,  7, -1, -2, -1, -1, -2, -2, -4, -3, -1, -2, -4,
		-1,  0, -3,  0,  2, -3, -2,  0, -3, -3,  1, -2,  0,  0, -1, -1,  5,  1,  0, -1, -1, -2, -2, -1,  3, -1, -4,
		-1, -1, -3, -2,  0, -3, -2,  0, -3, -3,  2, -2, -1,  0, -1, -2,  1,  5, -1, -1, -1, -3, -3, -2,  0, -1, -4,
		 1,  0, -1,  0,  0, -2,  0, -1, -2, -2,  0, -2, -1,  1,  0, -1,  0, -1,  4,  1,  0, -2, -3, -2,  0,  0, -4,
		 0, -1, -1, -1, -1, -2, -2, -2, -1, -1, -1, -1, -1,  0,  0, -1, -1, -1,  1,  5,  0,  0, -2, -2, -1,  0, -4,
		 0, -1, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -2, -1, -1,  0,  0, -1, -1, -2, -1, -1, -1, -4,
		 0, -3, -1, -3, -2, -1, -3, -3,  3,  2, -2,  1,  1, -3, -1, -2, -2, -3, -2,  0, -1,  4, -3, -1, -2, -1, -4,
		-3, -4, -2, -4, -3,  1, -2, -2, -3, -3, -3, -2, -1, -4, -2, -4, -2, -3, -3, -2, -2, -3, 11,  2, -3, -2, -4,
		-2, -3, -2, -3, -2,  3, -3,  2, -1, -1, -2, -1, -1, -2, -1, -3, -1, -2, -2, -2, -1, -1,  2,  7, -2, -1, -4,
		-1,  1, -3,  1,  4, -3, -2,  0, -3, -3,  1, -3, -1,  0, -1, -1,  3,  0,  0, -1, -1, -2, -3, -2,  4, -1, -4,
		 0, -1, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -2, -1, -1,  0,  0, -1, -1, -2, -1, -1, -1, -4,
		-4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,  1
	];

	static ref MAT: DMat<i32> = DMat::from_col_vec(27, 27, &*ARRAY);
}

pub fn blosum62(a: u8, b: u8) -> i32 {	
	let a = if a!=42 {
		(a-65) as usize
	} else {26};	

	let b = if b!=42 {
		(b-65) as usize
	} else {26};	

	MAT[(a as usize, b as usize)]
}