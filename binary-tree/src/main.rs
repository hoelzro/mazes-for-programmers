use rand::prelude::*;

use maze_grid::Grid;

fn binary_tree_algorithm(grid: &mut Grid, rng: &mut ThreadRng) {
    for row in 0..grid.rows {
        for col in 0..grid.columns {
            let src = (row, col);
            let dst = if row == grid.rows - 1 {
                (row, col + 1)
            } else if col == grid.columns - 1 {
                (row + 1, col)
            } else {
                if rng.gen() {
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
    let mut g = Grid::new(4, 4, 1, 0);

    let mut rng = thread_rng();
    binary_tree_algorithm(&mut g, &mut rng);

    println!("{}", g);
}
