use std::collections::HashSet;

use rand::prelude::*;

use maze_grid::Grid;

fn random_cell(grid: &Grid, rng: &mut ThreadRng) -> (u32, u32) {
    (
        (0..grid.rows).choose(rng).unwrap(),
        (0..grid.columns).choose(rng).unwrap(),
    )
}

fn neighbors(grid: &Grid, cell: &(u32, u32)) -> Vec<(u32, u32)> {
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

fn random_neighbor(grid: &Grid, rng: &mut ThreadRng, cell: &(u32, u32)) -> (u32, u32) {
    return *neighbors(grid, cell).choose(rng).unwrap();
}

fn wilsons_algorithm(grid: &mut Grid, rng: &mut ThreadRng) {
    let mut unvisited: HashSet<(u32, u32)> = HashSet::new();
    for row in 0..grid.rows {
        for col in 0..grid.columns {
            unvisited.insert((row, col));
        }
    }

    // choose a random cell and mark it as visited
    unvisited.remove(&random_cell(grid, rng));

    while !unvisited.is_empty() {
        // starting at an unvisited cell…
        let mut path: Vec<(u32, u32)> = Vec::new();
        path.push(*unvisited.iter().choose(rng).unwrap());

        // …do a loop-erased random walk until you encounter a visited cell
        loop {
            let current = path.last().unwrap();
            if !unvisited.contains(current) {
                break
            }

            let next: (u32, u32) = random_neighbor(grid, rng, current);

            // as part of the loop-erased random walk, if you find the next cell in the path
            // already, remove the loop
            // XXX hopefully I did this right
            if let Some(existing_position) = path.iter().position(|&cell| cell == next) {
                path = path[..existing_position].to_vec();
            }

            path.push(next);
        }

        // add the path to the maze
        for (from, to) in path.iter().zip(path[1..].iter()) {
            grid.smash_wall(*from, *to);
        }

        // mark each cell in the path as visited
        for cell in path {
            unvisited.remove(&cell);
        }
    }
}

fn main() {
    let mut g = Grid::new(4, 4, 1, 0);

    let mut rng = thread_rng();
    wilsons_algorithm(&mut g, &mut rng);

    println!("{}", g);
}
