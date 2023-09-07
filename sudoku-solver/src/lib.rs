mod utils;
mod solver;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn solve(grid: &mut [u8]) -> bool {
    let mut grid_data = [[0u8; 9]; 9];
    for (i, row) in grid_data.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            *value = grid[i * 9 + j];
        }
    }

    if solver::solvable(&grid_data) {
        solver::solve(&mut grid_data);
        // Update the grid array with the mutated values
        for (i, row) in grid_data.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                grid[i * 9 + j] = *value
            }
        }
        true
    } else {
        false
    }
}
