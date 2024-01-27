use rand::random;
use std::collections::HashSet;

use maze_grid::Grid;

fn binary_tree_algorithm(grid: &mut Grid) {
    for row in 0..grid.rows {
        for col in 0..grid.columns {
            let src = (row, col);
            let dst = if row == grid.rows - 1 {
                (row, col + 1)
            } else if col == grid.columns - 1 {
                (row + 1, col)
            } else {
                if random() {
                    (row + 1, col)
                } else {
                    (row, col + 1)
                }
            };

            grid.smash_wall(src, dst);
        }
    }
}

fn main() {
    let mut g = Grid{
        rows: 4,
        columns: 4,
        cell_width: 1,
        cell_height: 0,
        linked_cells: HashSet::new(),
    };

    binary_tree_algorithm(&mut g);

    println!("{}", g);
}
