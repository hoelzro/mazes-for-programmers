use std::collections::HashSet;

use rand::prelude::*;

use maze_grid::{Grid, random_cell, random_neighbor};

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
