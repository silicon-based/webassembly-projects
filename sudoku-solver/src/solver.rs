pub type Grid = [[u8; 9]; 9];

#[inline]
fn find_empty_cell(grid: &Grid) -> Option<(usize, usize)> {
    for (idx_row, row) in grid.iter().enumerate() {
        if let Some(idx_column) = row.iter().position(|x| x.eq(&0u8)) {
            return Some((idx_row, idx_column));
        }
    }
    None
}


#[inline]
fn check_validity(
    t_idx_row: usize,
    t_idx_col: usize,
    t_num: &u8,
    grid: &Grid) -> bool {
    if grid[t_idx_row].contains(t_num) {
        return false;
    }

    if grid.iter().any(|r| r[t_idx_col] == *t_num) {
        return false;
    }

    let square_row_start: usize = (t_idx_row / 3) * 3;
    let square_column_start: usize = (t_idx_col / 3) * 3;
    !grid
        .iter()
        .skip(square_row_start)
        .take(3)
        .any(|r| r
             .iter()
             .skip(square_column_start)
             .take(3)
             .any(|x| x == t_num))
}

pub fn solve(grid: &mut Grid) -> bool {
    let continuing_task = find_empty_cell(grid);
    match continuing_task {
        None => return true,
        Some((index_row, index_cell)) => {
            for i in 1..=9 {
                if check_validity(index_row, index_cell, &(i as u8), grid) {
                    grid[index_row][index_cell] = i as u8;
                    if solve(grid) {
                        return true;
                    };
                    grid[index_row][index_cell] = 0u8
                }
            }
        }
    }
    false
}

pub fn solvable(grid: &Grid) -> bool {
    for (index_row, row) in grid.iter().enumerate() {
        for (index_col, t_num) in row
            .iter()
            .enumerate()
            .filter(|(_, c)| *c != &0u8) {
                if grid[index_row]
                        .iter()
                        .filter(|x| x == &t_num)
                        .count() > 1 {
                    return false
                }

                if grid.iter()
                    .map(|r| r[index_col])
                    .filter(|x| x == t_num)
                    .count() >= 2 {
                    return false
                }

                let square_row_start: usize = (index_row / 3) * 3;
                let square_column_start: usize = (index_col / 3) * 3;
                let mut box_number_count: u8 = 0;
                for row in grid.iter().skip(square_row_start).take(3){
                    for cell in row.iter().skip(square_column_start).take(3) {
                        if cell.eq(t_num) {
                            box_number_count += 1;
                            if box_number_count == 2 {
                                return false
                            }
                        }
                    }
                }
            };
    }
    true
}

