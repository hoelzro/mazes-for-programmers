use std::collections::HashSet;
use std::fmt;

pub struct Grid {
    pub rows: u32,
    pub columns: u32,
    pub cell_width: usize, // XXX default this to 1
    pub cell_height: u32, // XXX default this to 0
    linked_cells: HashSet<((u32, u32), (u32, u32))>, // XXX default this to an empty set
}

impl Grid {
    pub fn new(rows: u32, columns: u32, cell_width: usize, cell_height: u32) -> Grid {
        Grid{
            rows: rows,
            columns: columns,
            cell_width: cell_width,
            cell_height: cell_height,
            linked_cells: HashSet::new(),
        }
    }

    fn has_wall(&self, cell_a: (u32, u32), cell_b: (u32, u32)) -> bool {
        let row_diff = (cell_a.0 as i32) - (cell_b.0 as i32);
        let col_diff = (cell_a.1 as i32) - (cell_b.1 as i32);

        assert!(row_diff.abs() + col_diff.abs() == 1);

        !(self.linked_cells.contains(&(cell_a, cell_b)) || self.linked_cells.contains(&(cell_b, cell_a)))
    }

    pub fn smash_wall(&mut self, cell_a: (u32, u32), cell_b: (u32, u32)) {
        let row_diff = (cell_a.0 as i32) - (cell_b.0 as i32);
        let col_diff = (cell_a.1 as i32) - (cell_b.1 as i32);

        assert!(row_diff.abs() + col_diff.abs() == 1);

        self.linked_cells.insert((cell_a, cell_b));
    }
}

fn cross_char(north: bool, east: bool, south: bool, west: bool) -> &'static str {
    match (north, east, south, west) {
        (false, false, false, false) => " ",
        (true, false, false, false)  => "╵",
        (true, false, false, true)   => "┘",
        (true, false, true, true)    => "┤",
        (false, true, false, false)  => "╶",
        (true, true, false, true)    => "┴",
        (true, false, true, false)   => "│",
        (true, true, false, false)   => "└",
        (false, false, true, false)  => "╷",
        (false, true, true, false)   => "┌",
        (true, true, true, false)    => "├",
        (false, false, false, true)  => "╴",
        (false, false, true, true)   => "┐",
        (false, true, false, true)   => "─",
        (false, true, true, true)    => "┬",
        (true, true, true, true)     => "┼",
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // draw the top of the grid
        write!(f, "┌")?;
        for right_col in 1..self.columns {
            let left_col = right_col - 1;
            write!(f, "{}", "─".repeat(self.cell_width))?;
            write!(f, "{}", if self.has_wall((0, left_col), (0, right_col)) { "┬" } else { "─" })?;
        }
        write!(f, "{}", "─".repeat(self.cell_width))?;
        write!(f, "┐\n")?;

        // draw rows
        for bottom_row in 1..self.rows {
            let top_row = bottom_row - 1;

            // draw spacer lines
            for _ in 0..self.cell_height {
                write!(f, "│")?;

                for right_col in 1..self.columns {
                    let left_col = right_col - 1;
                    write!(f, "{}", " ".repeat(self.cell_width))?;
                    write!(f, "{}", if self.has_wall((top_row, left_col), (top_row, right_col)) { "│" } else { " " })?;
                }
                write!(f, "{}", " ".repeat(self.cell_width))?;
                write!(f, "│\n")?;
            }

            // draw lines with walls
            write!(f, "{}", if self.has_wall((top_row, 0), (bottom_row, 0)) { "├" } else { "│" })?;
            for right_col in 1..self.columns {
                let left_col = right_col - 1;

                let has_north_wall = self.has_wall((top_row, left_col), (top_row, right_col));
                let has_east_wall = self.has_wall((top_row, right_col), (bottom_row, right_col));
                let has_south_wall = self.has_wall((bottom_row, left_col), (bottom_row, right_col));
                let has_west_wall = self.has_wall((top_row, left_col), (bottom_row, left_col));

                write!(f, "{}", (if has_west_wall { "─" } else { " " }).repeat(self.cell_width))?;
                write!(f, "{}", cross_char(has_north_wall, has_east_wall, has_south_wall, has_west_wall))?;
            }
            write!(f, "{}", (if self.has_wall((top_row, self.columns - 1), (bottom_row, self.columns - 1)) { "─" } else { " " }).repeat(self.cell_width))?;
            write!(f, "{}", if self.has_wall((top_row, self.columns - 1), (bottom_row, self.columns - 1)) { "┤\n" } else { "│\n" })?;
        }

        // draw bottom of the grid
        for _ in 0..self.cell_height {
            write!(f, "│")?;
            for right_col in 1..self.columns {
                let left_col = right_col - 1;
                write!(f, "{}", " ".repeat(self.cell_width))?;
                write!(f, "{}", if self.has_wall((self.rows - 1, left_col), (self.rows - 1, right_col)) { "│" } else { " " })?;
            }
            write!(f, "{}", " ".repeat(self.cell_width))?;
            write!(f, "│\n")?;
        }
        write!(f, "└")?;
        for right_col in 1..self.columns {
            let left_col = right_col - 1;
            write!(f, "{}", "─".repeat(self.cell_width))?;
            write!(f, "{}", if self.has_wall((self.rows - 1, left_col), (self.rows - 1, right_col)) { "┴" } else { "─" })?;
        }
        write!(f, "{}", "─".repeat(self.cell_width))?;
        write!(f, "┘")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut walls = HashSet::new();
        for bottom_row in 1..3 {
            let top_row = bottom_row - 1;
            for right_col in 1..3 {
                let left_col = right_col - 1;
                walls.insert( ((top_row, left_col), (top_row, right_col)) );
                walls.insert( ((top_row, left_col), (bottom_row, left_col)) );
            }
            walls.insert( ((top_row, 2), (bottom_row, 2)) );
        }
        for right_col in 1..3 {
            let left_col = right_col - 1;
            walls.insert( ((2, left_col), (2, right_col)) );
        }

        for (cell_a, cell_b) in walls {
            let mut g = Grid{
                rows: 3,
                columns: 3,
                cell_width: 3,
                cell_height: 1,
                linked_cells: HashSet::new(),
            };
            g.smash_wall(cell_a, cell_b);
            println!("{}", g);
            println!("");
        }
        assert_eq!(0, 1);
    }
}
