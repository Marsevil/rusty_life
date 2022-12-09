use ndarray::prelude::*;

pub type Size = (usize, usize);

/// This structure is used to encapsulate the board of the game
pub struct Board {
	board: Array2<u8>
}

impl Board {
	/// Create a new board from size
	pub fn new(size: &Size) -> Self {
		let mut size = size.clone();
		size.0 += 2;
		size.1 += 2;
		Self {
			board: Array2::zeros(size),
		}
	}

	/// Create a board from an array
	pub fn from(arr: Array2<u8>) -> Self {
		let mut dim = arr.dim().clone();
		dim.0 += 2;
		dim.1 += 2;
		let mut new_array = Array2::<u8>::zeros(dim);
		new_array.slice_mut(s![1..-1, 1..-1]).assign(&arr);
		Self {
			board: new_array,
		}
	}

	/// Permit access & edition of the internal array
	pub fn get_array_mut(&mut self) -> ArrayViewMut2<u8> {
		self.board.slice_mut(s![1..-1, 1..-1])
	}

	/// Permit access to the internal array
	pub fn get_array(&self) -> ArrayView2<u8> {
		self.board.slice(s![1..-1, 1..-1])
	}

	pub fn update(&mut self) {
		let mut arr = self.board.view_mut();
		let mut dim = arr.dim().clone();
		dim.0 -= 2;
		dim.1 -= 2;
		let mut neighbours = Array2::<u8>::zeros(dim);

		// Compute neighbours
		// Avoid array iteration by using empty borders.
		neighbours += &arr.slice(s![0..-2, 0..-2]);
		neighbours += &arr.slice(s![0..-2, 1..-1]);
		neighbours += &arr.slice(s![0..-2, 2..]);

		neighbours += &arr.slice(s![1..-1, 0..-2]);
		// Skip the current cell
		neighbours += &arr.slice(s![1..-1, 2..]);

		neighbours += &arr.slice(s![2.., 0..-2]);
		neighbours += &arr.slice(s![2.., 1..-1]);
		neighbours += &arr.slice(s![2.., 2..]);

		let mut board_view = arr.slice_mut(s![1..-1, 1..-1]);
		board_view.zip_mut_with(&neighbours, |y, &n| {
			*y = ((n == 3) || (n == 2 && *y > 0)) as u8;
		})
	}
}
