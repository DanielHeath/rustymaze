extern crate image;
use image::{ImageBuffer, RgbImage};
use rand::Rng;
mod rustymaze;
use rustymaze::*;


// what about a heterogenous one; thin walls, thick passages.
const MIN_PASSAGE_THICKNESS_PX: usize = 6;
const MIN_WALL_THICKNESS_PX: usize = 2;
const CELL_SIZE: usize = MIN_PASSAGE_THICKNESS_PX + MIN_WALL_THICKNESS_PX;
const MAIN_IMAGE_X: u32 = (250 * CELL_SIZE) as u32;
const MAIN_IMAGE_Y: u32 = (250 * CELL_SIZE) as u32;

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
    let entry: Region = if rand::random::<bool>() {
        let topleft = Point {
            x: ((rand::random::<usize>() % (MAIN_IMAGE_X as usize - (3 * CELL_SIZE as usize)))
                + CELL_SIZE as usize),
            y: 0,
        };
        Region {
            topleft,
            size: Point {
                x: MIN_PASSAGE_THICKNESS_PX,
                y: MIN_WALL_THICKNESS_PX,
            },
        }
    } else {
        let topleft = Point {
            x: 0,
            y: ((rand::random::<usize>() % (MAIN_IMAGE_Y as usize - (3 * CELL_SIZE as usize)))
                + CELL_SIZE as usize),
        };

        Region {
            topleft,
            size: Point {
                x: MIN_WALL_THICKNESS_PX,
                y: MIN_PASSAGE_THICKNESS_PX,
            },
        }
    };

    // exit at the opposite corner
    let exit = Region {
        topleft: Point {
            x: (MAIN_IMAGE_X as usize - entry.topleft.x),
            y: (MAIN_IMAGE_Y as usize - entry.topleft.y),
        } - entry.size,
        size: entry.size,
    };

    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut maze = Maze {
        img: ImageBuffer::new(MAIN_IMAGE_X, MAIN_IMAGE_Y),
        min_passage_width: MIN_PASSAGE_THICKNESS_PX,
        min_wall_width: MIN_WALL_THICKNESS_PX,
        size: Point {
            x: MAIN_IMAGE_X as usize,
            y: MAIN_IMAGE_Y as usize,
        },
        entry,
        exit,
    };
    maze_draw_outline(&mut maze);

    rustymaze::prim::maze_fill_prim(
        Region {
            topleft: Point { x: 0, y: 0 },
            size: maze.size,
        }
        .shrink_usize((MIN_WALL_THICKNESS_PX + MIN_PASSAGE_THICKNESS_PX) / 2),
        maze.entry,
        maze.exit,
        &mut maze,
    );
    maze.img.save("test.png").unwrap();
}
