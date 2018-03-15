use gen::MapGenerator;
use map::Tile;
use map::Rect;

pub type Layer = Vec<Vec<Tile>>;
pub type Location = (i32,i32);
pub type Point = (i32,i32);

pub struct Map {
  pub height: i32,
  pub width: i32,
  pub data: Layer,
  pub rooms: Vec<Rect>,
}

impl Map {
  pub fn new(map_size: Point) -> Self {
    Map {
      height: map_size.0,
      width: map_size.1,
      data: vec![vec![Tile::wall(); map_size.0 as usize]; map_size.1 as usize],
      rooms: vec![],
    }
  }
  pub fn generate_with<Creator>(&mut self, max_rooms: i32, room_min_size: i32, room_max_size: i32) -> (Self,Location)
  where
    Creator:MapGenerator,
  {
    Creator::generate(self, max_rooms, room_min_size, room_max_size)
  }
}

