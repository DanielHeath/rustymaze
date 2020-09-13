use debug_print::debug_println;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
struct PrimMaze {
    cellsx: usize,
    cellsy: usize,
    cells: Vec<Cell>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cell {
    idx: usize,
    path_id: usize,
    // open directions
    top: bool,
    bot: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Neighbor {
    direction: Direction,
    cell_idx: usize,
}

impl Cell {
    fn xy(&self, cellsx: usize) -> (usize, usize, usize, usize) {
        (self.idx, self.path_id, self.idx % cellsx, self.idx / cellsx)
    }
    fn neighbors(&self, cellist_size: usize, cellsx: usize) -> Vec<Neighbor> {
        let mut result = vec![];
        // Up a row
        if self.idx >= cellsx {
            result.push(Neighbor {
                direction: Direction::Up,
                cell_idx: self.idx - cellsx,
            });
        }
        // Down a row
        if (self.idx + cellsx) < cellist_size {
            result.push(Neighbor {
                direction: Direction::Down,
                cell_idx: self.idx + cellsx,
            });
        }
        // Left a row
        if self.idx % cellsx > 0 {
            result.push(Neighbor {
                direction: Direction::Left,
                cell_idx: self.idx - 1,
            });
        }
        // Right a row
        if self.idx < cellist_size && (self.idx + 1) % cellsx > 0 {
            result.push(Neighbor {
                direction: Direction::Right,
                cell_idx: self.idx + 1,
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::prim::*;
    #[test]
    fn neighbors_works() {
        let mut base = Cell {
            idx: 4 as usize,
            path_id: 1 as usize,
            top: true,
            bot: true,
            left: true,
            right: true,
        };

        assert_eq!(
            base.neighbors(50, 3),
            vec![
                Neighbor {
                    direction: Direction::Up,
                    cell_idx: 1
                },
                Neighbor {
                    direction: Direction::Down,
                    cell_idx: 7
                },
                Neighbor {
                    direction: Direction::Left,
                    cell_idx: 3
                },
                Neighbor {
                    direction: Direction::Right,
                    cell_idx: 5
                }
            ]
        );
        assert_eq!(
            base.neighbors(50, 4),
            vec![
                Neighbor {
                    direction: Direction::Up,
                    cell_idx: 0
                },
                Neighbor {
                    direction: Direction::Down,
                    cell_idx: 8
                },
                Neighbor {
                    direction: Direction::Right,
                    cell_idx: 5
                }
            ]
        );
        assert_eq!(
            base.neighbors(50, 5),
            vec![
                Neighbor {
                    direction: Direction::Down,
                    cell_idx: 9
                },
                Neighbor {
                    direction: Direction::Left,
                    cell_idx: 3
                }
            ]
        );
        assert_eq!(
            base.neighbors(6, 6),
            vec![
                Neighbor {
                    direction: Direction::Left,
                    cell_idx: 3
                },
                Neighbor {
                    direction: Direction::Right,
                    cell_idx: 5
                }
            ]
        );
        base.idx = 3;
        assert_eq!(
            base.neighbors(8, 4),
            vec![
                Neighbor {
                    direction: Direction::Down,
                    cell_idx: 7
                },
                Neighbor {
                    direction: Direction::Left,
                    cell_idx: 2
                }
            ]
        );
        base.idx = 7;
        assert_eq!(
            base.neighbors(8, 4),
            vec![
                Neighbor {
                    direction: Direction::Up,
                    cell_idx: 3
                },
                Neighbor {
                    direction: Direction::Left,
                    cell_idx: 6
                }
            ]
        );
    }
}

pub fn maze_fill_prim(
    region: super::Region,
    entry: super::Region,
    exit: super::Region,
    maze: &mut super::Maze,
) {
    // Regions are a neat multiple of cell sizes.
    assert_eq!(
        region.size.x % (maze.min_passage_width + maze.min_wall_width),
        0
    );
    assert_eq!(
        region.size.y % (maze.min_passage_width + maze.min_wall_width),
        0
    );
    let cellsx = region.size.x / (maze.min_passage_width + maze.min_wall_width);
    let cellsy = region.size.y / (maze.min_passage_width + maze.min_wall_width);

    let mut cells = Vec::new();
    let mut idx: usize = 0;
    while idx < cellsx * cellsy {
        cells.push(Cell {
            idx,
            path_id: idx,
            top: false,
            bot: false,
            left: false,
            right: false,
        });
        idx += 1;
    }
    let mut rng = rand::thread_rng();

    let cell_count = cells.len();
    // range upper bound is exclusive, don't use cell_count-1
    let mut indicies: Vec<usize> = (0..(cell_count)).into_iter().collect();
    indicies.shuffle(&mut rng);
    for ci in indicies {
        let mut cell = cells[ci]; // grab a random cell; break all walls that lead to a different path.
        let mut neighbors = cell.neighbors(cell_count, cellsx);
        neighbors.shuffle(&mut rng);
        let mut neighborsProcessed = 0;
        for neighbor in neighbors {
            // would be good to repeat processing 1 neighbor at a time.
            if neighborsProcessed > 1 && rand::random::<bool>() {
                break
            }

            if neighbor.cell_idx >= cell_count {
                panic!(format!(
                    "Invalid neighbor index exceeds {:?} (width {:?}): {:?}",
                    cell_count, cellsx, neighbor
                ))
            }
            if cells[neighbor.cell_idx].path_id != cell.path_id {
                neighborsProcessed += 1; // a neighbor is only processed if it has a *difference*

                // Update all cells on the 'other side' path to 'this side'
                let other_path = cells[neighbor.cell_idx].path_id;

                for pi in 0..(cell_count) { // range upper bound is exclusive
                    if cells[pi].path_id == other_path {
                        let mut other_cell = cells[pi];
                        other_cell.path_id = cell.path_id;
                        cells[pi] = other_cell
                    }
                }
                let mut other_cell = cells[neighbor.cell_idx];

                // Break the wall
                match neighbor.direction {
                    Direction::Up => {
                        other_cell.bot = true;
                        cell.top = true;
                    }
                    Direction::Down => {
                        other_cell.top = true;
                        cell.bot = true;
                    }
                    Direction::Left => {
                        other_cell.right = true;
                        cell.left = true;
                    }
                    Direction::Right => {
                        other_cell.left = true;
                        cell.right = true;
                    }
                }
                cells[neighbor.cell_idx] = other_cell;
                cells[ci] = cell;
            }
        }
    }
    cells.sort_by_key(|k| k.idx);

    let cell_width = region.size.x / cellsx;
    let cell_height = region.size.y / cellsy;
    for (x32, y32, pixel) in maze.img.enumerate_pixels_mut() {
        let x = x32 as usize;
        let y = y32 as usize;
        if region.intersect(super::Point { x, y }) {
            // region-relative coordinates
            let rx = x - region.topleft.x;
            let ry = y - region.topleft.y;

            let cell_x_idx = rx / cell_width;
            let cell_x_over = rx % cell_width;
            let cell_y_idx = ry / cell_height;
            let cell_y_over = ry % cell_height;

            let cell = cells[cell_x_idx + (cell_y_idx * cellsx)];

            if (cell_x_over > maze.min_passage_width) {
                if cell.right {
                    // *pixel = image::Rgb([100, 100, 100]);
                } else if cell_x_idx < cellsx - 1 {
                    *pixel = image::Rgb([10, 10, 10]);
                }
            }

            if (cell_y_over > maze.min_passage_width) {
                if cell.bot {
                    // *pixel = image::Rgb([100, 100, 100]);
                } else if cell_y_idx < cellsy - 1 {
                    *pixel = image::Rgb([10, 10, 10]);
                }
            }
        }
    }
}
