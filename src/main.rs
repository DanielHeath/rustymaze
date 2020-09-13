extern crate image;
use std::ops::{Add, Sub};
use std::cmp::Ordering;
use image::{ImageBuffer, RgbImage};
use rand::Rng;
// what about a heterogenous one; thin walls, thick passages.
const MIN_PASSAGE_THICKNESS_PX: usize = 6;
const MIN_WALL_THICKNESS_PX: usize = 2;
const CELL_SIZE: usize = MIN_PASSAGE_THICKNESS_PX + MIN_WALL_THICKNESS_PX;
const MAIN_IMAGE_X: u32 = (15 * CELL_SIZE) as u32;
const MAIN_IMAGE_Y: u32 = (8 * CELL_SIZE) as u32;
// TODO: Define types for x & y coords
struct Maze {
    img: RgbImage,
    min_passage_width: usize,
    min_wall_width: usize,
    size: Point,
    entry: Region,
    exit: Region
}

// how to model a maze give you want to be able to do 'strange stuff'?
// how about a 1px minimum feature size (so 'in theory' a 1px wide tunnel is enough)
// but in practice you use bigger ones for all generation algs
// that way you can mix different styles.
// main issue is how to join edges if they don't 100% line up between styles?
// one rule would be to make strategies parametric over borders and require entrances
// between zones to snap to a grid while allowing joints.
// That *should* work.
// for debugging, can draw a background color indicating the style.
// each strategy needs to know how to generate/render itself
// can a zone be non-square? yes, but only on the grid, with grid-size-wide edges.
// that's a bit shit; can it be better?

// draw separate 'room', 'traversability' and 'line-of-sight' rasters
// strategies should avoid drawing exterior walls; instead, assume they are being blitted
// over a fully-walled-out zone (no traversability, no line of sight, all wall-zones).
// an after-pass can fill in wall cells with detail pixels, possibly even based on the strategy in use for that pixel

fn main() {
//         entry: Region{topleft: ingress, size: entry_size},
        // exit: Region{topleft: egress - entry_size, size: entry_size},

    let entry: Region = if rand::random::<bool>() {
        let topleft = Point{
            x: ((rand::random::<usize>() % (MAIN_IMAGE_X as usize - (3 * CELL_SIZE as usize))) + CELL_SIZE as usize),
            y: 0,
        };
        Region{topleft, size: Point {
            x: MIN_PASSAGE_THICKNESS_PX,
            y: MIN_WALL_THICKNESS_PX,

        }}
    } else {
        let topleft = Point{
            x: 0,
            y: ((rand::random::<usize>() % (MAIN_IMAGE_Y as usize - (3 * CELL_SIZE as usize))) + CELL_SIZE as usize),
        };

        Region{topleft, size: Point {
            x: MIN_WALL_THICKNESS_PX,
            y: MIN_PASSAGE_THICKNESS_PX,
        }}
    };

    // exit at the opposite corner
    let exit = Region{
        topleft: Point{
            x: (MAIN_IMAGE_X  as usize - entry.topleft.x),
            y: (MAIN_IMAGE_Y  as usize - entry.topleft.y),
        } - entry.size,
        size: entry.size
    };


    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut maze = Maze{
        img: ImageBuffer::new(MAIN_IMAGE_X, MAIN_IMAGE_Y),
        min_passage_width: MIN_PASSAGE_THICKNESS_PX,
        min_wall_width: MIN_WALL_THICKNESS_PX,
        size: Point{x: MAIN_IMAGE_X as usize, y: MAIN_IMAGE_Y as usize},
        entry,
        exit,
    };
    maze_draw_outline(&mut maze);
    maze_fill_prim(
        Region{topleft: Point{x: 0, y: 0}, size: maze.size}.shrink_usize(MIN_WALL_THICKNESS_PX),
        maze.entry,
        maze.exit,
        &mut maze,
    );
    maze.img.save("test.png").unwrap();
}

// todo: store grid alignment, instead of using a random const?
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point {
        Point{x: self.x + _rhs.x, y: self.y + _rhs.y}
    }
}
impl std::ops::Add<usize> for Point {
    type Output = Point;
    fn add(self, _rhs: usize) -> Point {
        Point{x: self.x + _rhs, y: self.y + _rhs}
    }
}
impl std::ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, _rhs: Point) -> Point {
        Point{x: self.x - _rhs.x, y: self.y - _rhs.y}
    }
}
impl std::ops::Sub<usize> for Point {
    type Output = Point;
    fn sub(self, _rhs: usize) -> Point {
        Point{x: self.x - _rhs, y: self.y - _rhs}
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y {
            return Some(Ordering::Equal)
        }
        if self.x > other.x && self.y > other.y {
            return Some(Ordering::Greater)
        }
        if self.x < other.x && self.y < other.y {
            return Some(Ordering::Less)
        }
        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Region {
    topleft: Point,
    size: Point,
}
impl Region {
    fn bot_right(&self) -> Point {
        self.topleft + self.size
    }

    fn intersect(&self, point: Point) -> bool {
        point > self.topleft && point < self.bot_right()
    }

    fn shrink(&self, point: Point) -> Region {
        Region{
            topleft: self.topleft + point,
            size: self.size - point - point,
        }
    }
    fn shrink_usize(&self, amt: usize) -> Region {
        Region{
            topleft: self.topleft + amt,
            size: self.size - amt - amt,
        }
    }
}

fn maze_draw_outline(maze: &mut Maze) {
    for (x32, y32, pixel) in maze.img.enumerate_pixels_mut() {
        let x = x32 as usize;
        let y = y32 as usize;
        let r = (0.3 * x as f32) as u8 + 50;
        let b = (0.3 * y as f32) as u8 + 50;
        let mut fill = image::Rgb([r, 0, b]);
        if !maze.entry.intersect(Point{x, y}) &&
           !maze.exit.intersect(Point{x, y}) && (
             x < maze.min_wall_width ||
             x > maze.size.x - (maze.min_wall_width) ||
             y < maze.min_wall_width ||
             y > maze.size.y - (maze.min_wall_width)
          ) {
            fill = image::Rgb([0, 0, 0]);
        }
        if maze.entry.intersect(Point{x, y}) {
            fill = image::Rgb([244, 244, 0]);
        }
        if maze.exit.intersect(Point{x, y}) {
            fill = image::Rgb([0, 244, 244]);
        }
        *pixel = fill
    }

}

fn maze_fill_prim(
    region: Region,
    entry: Region,
    exit: Region,
    maze: &mut Maze,
) {
    let cellsx = region.size.x / CELL_SIZE;
    let cellsy = region.size.y / CELL_SIZE;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Cell {
        idx: usize,
        top: bool,
        bot: bool,
        left: bool,
        right: bool,
    }
    let mut cells = Vec::new();
    let mut idx: usize = 0;
    let mut paths = Vec::new();
    while idx < cellsx * cellsy {
        idx += 1;
        cells.push(Cell{idx, top: false, bot: false, left: false, right: false});
        paths.push(vec![idx]);
    }

    while paths.len() > 1 {
        // pick a cell at random; roll around the list it has no neighbors in other paths
        let pathIdx = rand::random::<usize>() % paths.len();
        let startCellIdxInPath = rand::random::<usize>() % paths[pathIdx].len();


        let startCellIdx = paths[pathIdx][startCellIdxInPath];
        let neighbors = vec![
            startCellIdx - cellsx,
            startCellIdx + cellsx,
        ];
        if startCellIdx % cellsx != 0 {
            neighbors.push(startCellIdx - 1)
        }

        if startCellIdx - 1 % cellsx != 0 {
            neighbors.push(startCellIdx + 1)
        }
        neighbors.retain(|&x| x > 0)

        paths.remove(pathIdx)
        panic!("wtf")
    }
    // initially, every cell is its own path
    // let paths = vec![vec![&mut Cell{top: false,bot: false,left: false,right: false}]; cellsx * cellsy];
    // let path = vec![Cell{top: false,bot: false,left: false,right: false};0]
    // // cells[x + (y * cellsx)]
    // while let idx = rand::random::<usize>() % (cellsx * cellsy) as usize {

    // }
    // Mark all walls as closed.
    // Select a room from the set of rooms, and add it to the "path".
    // Add the four walls of the room to the "wall list". This is the list that we keep processing until it is empty.
    // While the wall list is not empty:
    //     Select a wall.
    //     Find the rooms adjacent to the wall.
    //     If there are two adjacent rooms, and they are not already connected*:
    //         Mark the wall as "Open".
    //         Add all unvisited rooms to the path.
    //         Add the walls adjacent to each unvisited room to the wall list.
    //     Remove the wall from the wall list.


    for (x32, y32, pixel) in maze.img.enumerate_pixels_mut() {
        let x = x32 as usize;
        let y = y32 as usize;
        if region.intersect(Point{x, y}) {
            // TODO: assume pre-fill with black
            let mut fill = image::Rgb([0, 0, 0]);
            *pixel = fill
        }

    }
}