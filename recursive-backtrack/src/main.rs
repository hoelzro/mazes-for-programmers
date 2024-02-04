use std::collections::{HashSet, VecDeque};

use rand::prelude::*;

use maze_grid::{Grid, neighbors, random_cell};

fn recursive_backtrack_algorithm(grid: &mut Grid, rng: &mut ThreadRng) {
    let mut unvisited = (0..grid.rows).flat_map(|row| (0..grid.columns).map(move |col| (row, col))).collect::<HashSet<_>>();
    let mut path = VecDeque::new();

    // pick a random starting point, mark it as visited, and add to the path stack
    let first_cell = random_cell(grid, rng);
    unvisited.remove(&first_cell);
    path.push_back(first_cell);

    while !unvisited.is_empty() {
        let current = path.back().unwrap();
        let unvisited_neighbors = neighbors(grid, current).into_iter().filter(|n| unvisited.contains(n)).collect::<Vec<_>>();

        // if we have no unvisited neighbors, backtrack and try again
        if unvisited_neighbors.is_empty() {
            path.pop_back();
            continue;
        }

        // otherwise, choose an unvisited neighbor at random, link them, and continue onward from that neighbor
        let next = unvisited_neighbors.choose(rng).unwrap();
        grid.smash_wall(*current, *next);
        unvisited.remove(next);
        path.push_back(*next);
    }
}

fn main() {
    let mut g = Grid::new(4, 4, 1, 0);

    let mut rng = thread_rng();
    recursive_backtrack_algorithm(&mut g, &mut rng);

    println!("{}", g);
}
