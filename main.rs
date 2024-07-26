extern crate array2;

use array2::Array2;
use csc411_image::{GrayImage, Read};
use std::env;

pub fn is_sudoku_valid(grid: &Array2<u16>) -> bool {
    // Check rows
    for row in 0..9 {
        // Boolean array acts as "values seen so far".
        // Default value is false.
        let mut row_values = [false; 9];
        for col in 0..9 {
            let value = grid.get(row, col).unwrap();
            // If value is 0, it's an empty cell, so we ignore it.
            // Otherwise, if the value at row_values[value - 1] was changed to true,
            // then we've seen this value before, so the sudoku is invalid.
            if *value == 0 || row_values[(*value - 1) as usize] {
                return false;
            }
            row_values[(*value - 1) as usize] = true;
        }
    }

    // Check columns
    for col in 0..9 {
        let mut col_values = [false; 9];
        for row in 0..9 {
            let value = grid.get(row, col).unwrap();
            if *value == 0 || col_values[(*value - 1) as usize] {
                return false;
            }
            col_values[(*value - 1) as usize] = true;
        }
    }

    // Check sub-grids
    // Row_start and col_start loops are the coordinates of the top-left corner of the subgrid.
    for row_start in (0..9).step_by(3) {
        for col_start in (0..9).step_by(3) {
            let mut subgrid_values = [false; 9];
            // Row_offset and col_offset loops indices the subgrid.
            for row_offset in 0..3 {
                for col_offset in 0..3 {
                    let value = grid.get(row_start + row_offset, col_start + col_offset).unwrap();
                    if *value == 0 || subgrid_values[(*value - 1) as usize] {
                        return false;
                    }
                    subgrid_values[(*value - 1) as usize] = true;
                }
            }
        }
    }

    true
}

fn main() {
    let mut sudo_data = Array2::new(9, 9);
    let input = env::args().nth(1);
    let pgm = GrayImage::read(input.as_deref()).unwrap();
    for (i, gray) in pgm.pixels.iter().enumerate() {
        let col = i % 9;
        let row = i / 9;
        sudo_data.insert_row_mjr(row, col, gray.value);
    }
    let result = is_sudoku_valid(&sudo_data);
    // Exit with 0 if the sudoku is valid but since result is bool, !result is needed.
    std::process::exit(!result as i32);
}
