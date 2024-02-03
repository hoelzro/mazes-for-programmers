use std::collections::HashSet;

use rand::prelude::*;

use maze_grid::{Grid, neighbors, random_cell};

fn hunt_kill_algorithm(grid: &mut Grid, rng: &mut ThreadRng) {
    let mut unvisited = HashSet::new();
    for row in 0..grid.rows {
        for col in 0..grid.columns {
            unvisited.insert((row, col));
        }
    }

    // pick a random starting point
    let mut current = random_cell(grid, rng);
    unvisited.remove(&current);

    while !unvisited.is_empty() {
        let mut unvisited_neighbors = Vec::new();
        for n in neighbors(grid, &current) {
            if unvisited.contains(&n) {
                unvisited_neighbors.push(n);
            }
        }

        // if the current cell has unvisited neighbors, choose one at random,
        // link them, and continue onward from that neighbor
        if let Some(next) = unvisited_neighbors.choose(rng) {
            grid.smash_wall(current, *next);
            unvisited.remove(&next);
            current = *next;
        } else {
            // otherwise, hunt for an unvisited cell with a visited neighbor
            'hunt: for row in 0..grid.rows {
                for col in 0..grid.columns {
                    let candidate = (row, col);
                    if unvisited.contains(&candidate) {
                        let mut visited_neighbors = Vec::new();
                        for n in neighbors(grid, &candidate) {
                            if !unvisited_neighbors.contains(&n) {
                                visited_neighbors.push(n);
                            }
                        }

                        if visited_neighbors.len() > 0 {
                            // once you've found an unvisited cell with at least one visited
                            // neighbor, make it the current node after linking to a random visited neighbor
                            grid.smash_wall(candidate, *visited_neighbors.choose(rng).unwrap());
                            unvisited.remove(&candidate);
                            current = candidate;
                            break 'hunt;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut g = Grid::new(4, 4, 1, 0);

    let mut rng = thread_rng();
    hunt_kill_algorithm(&mut g, &mut rng);

    println!("{}", g);
}
