use rusty_life::life_api::*;
use ndarray::prelude::*;

#[test]
fn test_new() {
	let size = (9, 5);
	let board = Board::new(&size);

	assert_eq!(board.get_array(), Array2::<u8>::zeros(size));
}

#[test]
fn test_from() {
	let arr = array![
		[1, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 1, 1],
		[0, 0, 1, 0, 1, 0],
		[0, 1, 1, 1, 0, 0],
		[0, 0, 1, 0, 0, 0],
		[0, 0, 0, 0, 0, 0]
	];

	let board = Board::from(arr.clone());

	assert_eq!(board.get_array(), arr);
}

#[test]
fn test_update() {
	let arr = array![
		[1, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 1, 1],
		[0, 0, 1, 0, 1, 0],
		[0, 1, 1, 1, 0, 0],
		[0, 0, 1, 0, 0, 0],
		[0, 0, 0, 0, 0, 0]
	];

	let mut board = Board::from(arr);
	board.update();

	let attempted_array = array![
		[0, 0, 0, 0, 0, 0],
		[0, 0, 0, 1, 1, 1],
		[0, 1, 1, 0, 1, 1],
		[0, 1, 0, 0, 0, 0],
		[0, 1, 1, 1, 0, 0],
		[0, 0, 0, 0, 0, 0]
	];

	assert_eq!(board.get_array(), attempted_array);
}