use rand::prelude::*;

use maze_grid::Grid;

fn sidewinder_algorithm(grid: &mut Grid) {
    let mut rng = thread_rng();

    for row in 0..grid.rows {
        for col in 0..grid.columns {
            let move_east = if row == grid.rows - 1 {
                true
            } else if col == grid.columns - 1 {
                false
            } else {
                random()
            };

            if move_east {
                grid.smash_wall((row, col), (row, col + 1));
            } else {
                // determine the current run of cells
                let mut run_start_col = col;

                while run_start_col > 0 {
                    // a run ends if there's a wall in the way to the west of a cell…
                    if grid.has_wall((row, run_start_col), (row, run_start_col - 1)) {
                        break;
                    }

                    // …or if there's *no* wall in the way to the south
                    if !grid.has_wall((row, run_start_col), (row + 1, run_start_col)) {
                        break;
                    }

                    run_start_col -= 1;
                }

                // pick a random cell among those in the run, and smash the wall
                let smash_col = (run_start_col..=col).choose(&mut rng).unwrap();
                grid.smash_wall((row, smash_col), (row + 1, smash_col));
            }
        }
    }
}

fn main() {
    let mut g = Grid::new(4, 4, 1, 0);

    sidewinder_algorithm(&mut g);

    println!("{}", g);
}
