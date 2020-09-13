use image::RgbImage;
use std::cmp::Ordering;
use std::ops::{Add, Sub};
pub mod prim;

// TODO: Define types for x & y coords
pub struct Maze {
  pub img: RgbImage,
  pub min_passage_width: usize,
  pub min_wall_width: usize,
  pub size: Point,
  pub entry: Region,
  pub exit: Region,
}

// todo: store grid alignment, instead of using a random const?
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
  pub x: usize,
  pub y: usize,
}

impl std::ops::Add<Point> for Point {
  type Output = Point;
  fn add(self, _rhs: Point) -> Point {
    Point {
      x: self.x + _rhs.x,
      y: self.y + _rhs.y,
    }
  }
}
impl std::ops::Add<usize> for Point {
  type Output = Point;
  fn add(self, _rhs: usize) -> Point {
    Point {
      x: self.x + _rhs,
      y: self.y + _rhs,
    }
  }
}
impl std::ops::Sub<Point> for Point {
  type Output = Point;
  fn sub(self, _rhs: Point) -> Point {
    Point {
      x: self.x - _rhs.x,
      y: self.y - _rhs.y,
    }
  }
}

impl std::ops::Sub<usize> for Point {
  type Output = Point;
  fn sub(self, _rhs: usize) -> Point {
    Point {
      x: self.x - _rhs,
      y: self.y - _rhs,
    }
  }
}

impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
    if self.x == other.x && self.y == other.y {
      return Some(Ordering::Equal);
    }
    if self.x > other.x && self.y > other.y {
      return Some(Ordering::Greater);
    }
    if self.x < other.x && self.y < other.y {
      return Some(Ordering::Less);
    }
    None
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Region {
  pub topleft: Point,
  pub size: Point,
}
impl Region {
  pub fn bot_right(&self) -> Point {
    self.topleft + self.size
  }

  pub fn intersect(&self, point: Point) -> bool {
    point > self.topleft && point < self.bot_right()
  }

  pub fn shrink(&self, point: Point) -> Region {
    Region {
      topleft: self.topleft + point,
      size: self.size - point - point,
    }
  }
  pub fn shrink_usize(&self, amt: usize) -> Region {
    Region {
      topleft: self.topleft + amt,
      size: self.size - amt - amt,
    }
  }
}

pub fn maze_draw_outline(maze: &mut Maze) {
  let outline_thickness = (maze.min_wall_width + maze.min_passage_width) / 2;

  for (x32, y32, pixel) in maze.img.enumerate_pixels_mut() {
    let x = x32 as usize;
    let y = y32 as usize;
    let r = (0.3 * x as f32) as u8;
    let b = (0.3 * y as f32) as u8;
    let mut fill = image::Rgb([r, 0, b]);
    if x <= outline_thickness
        || x >= maze.size.x - outline_thickness
        || y <= outline_thickness
        || y >= maze.size.y - outline_thickness
    {
      fill = image::Rgb([0, 0, 0]);
    }
    if maze.entry.intersect(Point { x, y }) {
      fill = image::Rgb([244, 244, 0]);
    }
    if maze.exit.intersect(Point { x, y }) {
      fill = image::Rgb([0, 244, 244]);
    }
    *pixel = fill
  }
}
