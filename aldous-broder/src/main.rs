use std::collections::HashSet;

use rand::prelude::*;

use maze_grid::Grid;

fn neighbors(grid: &Grid, cell: (u32, u32)) -> Vec<(u32, u32)> {
    let mut res = Vec::new();
    for (d_row, d_col) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let neighbor_row = cell.0 as i32 + d_row;
        if neighbor_row < 0 || neighbor_row >= grid.rows as i32 {
            continue;
        }

        let neighbor_col = cell.1 as i32 + d_col;
        if neighbor_col < 0 || neighbor_col >= grid.columns as i32 {
            continue;
        }

        res.push((neighbor_row as u32, neighbor_col as u32));
    }
    res
}

fn aldous_broder_algorithm(grid: &mut Grid, rng: &mut ThreadRng) {
    let mut unvisited = HashSet::new();
    for row in 0..grid.rows {
        for col in 0..grid.columns {
            unvisited.insert((row, col));
        }
    }

    let mut current = (
        (0..grid.rows).choose(rng).unwrap(),
        (0..grid.columns).choose(rng).unwrap(),
    );

    unvisited.remove(&current);

    while !unvisited.is_empty() {
        let next = *neighbors(grid, current).choose(rng).unwrap();

        if unvisited.contains(&next) {
            grid.smash_wall(current, next);
            unvisited.remove(&next);
        }

        current = next;
    }
}

fn main() {
    let mut g = Grid::new(4, 4, 1, 0);

    let mut rng = thread_rng();
    aldous_broder_algorithm(&mut g, &mut rng);

    println!("{}", g);
}
