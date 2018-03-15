extern crate rand;

pub mod dungeon;
use map::Map;
use map::Rect;
use map::Location;
use map::room;

pub trait MapGenerator{
  fn generate(map: &mut Map, max_rooms: i32, room_min_size: i32, room_max_size: i32) -> (Map, Location);
}

