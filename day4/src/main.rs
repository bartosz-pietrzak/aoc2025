use std::error::Error;
use std::fs;

type Err = Box<dyn Error>;

struct Grid {
    grid_x_size: usize,
    grid_y_size: usize,
    items: Vec<i8>,
    window_x_size: usize,
    window_y_size: usize,
    max_neighbours: usize,
}

impl Grid {
    fn new(
        grid_x_size: usize,
        grid_y_size: usize,
        window_x_size: usize,
        window_y_size: usize,
        max_neighbours: usize,
        items: Vec<i8>,
    ) -> Self {
        Self {
            grid_x_size,
            grid_y_size,
            window_x_size,
            window_y_size,
            max_neighbours,
            items,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.grid_x_size + x
    }

    fn get_xy_index(&self, idx: usize) -> (usize, usize) {
        (
            idx.rem_euclid(self.grid_x_size),
            idx.saturating_div(self.grid_y_size),
        )
    }

    fn get(&self, x: isize, y: isize) -> i8 {
        if x < 0 || y < 0 {
            return 0;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.grid_x_size || y >= self.grid_y_size || x < 0 || y < 0 {
            return 0;
        } else {
            self.items[self.get_index(x as usize, y as usize)]
        }
    }

    fn count_neighbours(&mut self, idx: usize) -> u32 {
        let (x_window_center, y_window_center) = self.get_xy_index(idx);
        let window_x_bound: isize = self.window_x_size.saturating_sub(1) as isize / 2;
        let window_y_bound: isize = self.window_y_size.saturating_sub(1) as isize / 2;
        for i_x in -window_x_bound..=window_x_bound {
            for i_y in -window_y_bound..=window_y_bound {
                let item = self.get(i_x + x_window_center, i_y + y_window_center);
                if i_x != 0 || i_y != 0 {
                    sum += item;
                } else if item == 0 {
                    sum += self.max_neighbours as u32; //this is not a roll
                }
            }
        }
        sum
    }

    fn retrieve_items(&mut self) -> Vec<usize> {
        let mut items_retrieved = Vec::new();
        for i_y in 0..self.grid_y_size {
            for i_x in 0..self.grid_x_size {
                let is_a_roll = if self.get(i_x.try_into().unwrap(), i_y.try_into().unwrap()) > 0 {
                    true
                } else {
                    false
                };
                if is_a_roll {
                    if self.count_neighbours(i_x.try_into().unwrap(), i_y.try_into().unwrap())
                        < self.max_neighbours.try_into().unwrap()
                    {
                        items_retrieved.push(self.get_index(i_x, i_y))
                    }
                }
            }
        }
        items_retrieved
    }

    fn remove_items(&mut self, items_to_remove: Vec<usize>) {
        items_to_remove.iter().for_each(|i| self.items[*i] = 0);
    }
}

fn main() -> Result<(), Err> {
    let path = "input";
    let file = fs::read_to_string(path)?;
    let window_size = 3;
    let max_neighbours = 4;
    let lines = file.lines().collect::<Vec<&str>>();
    let grid_x_size = lines[0].len();
    let grid_y_size = lines.len();
    let rows: Vec<i8> = file
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '@' => 1,
            '.' => 0,
            _ => -1, // invalid
        })
        .collect();
    let mut grid = Grid::new(
        grid_x_size,
        grid_y_size,
        window_size,
        window_size,
        max_neighbours,
        rows,
    );
    let mut rolls_available_to_remove = grid.retrieve_items();
    let mut total_removed = rolls_available_to_remove.len();
    println!("part 1 {}", rolls_available_to_remove.len());
    while rolls_available_to_remove.len() > 0 {
        grid.remove_items(rolls_available_to_remove);
        rolls_available_to_remove = grid.retrieve_items();
        total_removed += rolls_available_to_remove.len();
    }
    println!("part 2 {}", total_removed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let rows = vec![9, 1, 1, 1, 0, 1, 1, 5, 8];
        let grid_x_size = 3;
        let grid_y_size = 3;
        let grid = Grid::new(grid_x_size, grid_y_size, 3, 3, 4, rows);
        assert_eq!(grid.get(1, 1), 0);
        assert_eq!(grid.get(0, 0), 9);
        assert_eq!(grid.get(2, 2), 8);
        assert_eq!(grid.get(1, 2), 5);
        assert_eq!(grid.get(3, 3), 0);
        assert_eq!(grid.get(-1, -1), 0);
    }

    #[test]
    fn test_count_neighbours() {
        let rows = vec![
            0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1,
            1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1,
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1,
            1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0,
        ];
        let grid_x_size = 10;
        let grid_y_size = 10;
        let mut grid = Grid::new(grid_x_size, grid_y_size, 3, 3, 4, rows);
        assert_eq!(grid.count_neighbours(2, 0), 3);
        assert_eq!(grid.count_neighbours(2, 2), 6);
        assert_eq!(grid.count_neighbours(7, 0), 4);
        assert_eq!(grid.count_neighbours(1, 1), 6);
        assert_eq!(grid.count_neighbours(8, 9), 2);
    }
}
