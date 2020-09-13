

struct PrimMaze {
    cellsx: usize;
    cellsy: usize;
    cells:
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