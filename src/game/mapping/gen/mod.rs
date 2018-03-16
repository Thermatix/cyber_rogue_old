use ::rand;

pub mod dungeon;
use mapping::*;
use mapping::rect::*;

pub trait MapGenerator{
  fn generate(map: Map, max_rooms: i32, room_min_size: i32, room_max_size: i32) -> (Map, Location);
}

