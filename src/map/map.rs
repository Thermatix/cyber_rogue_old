use ::tcod::map::{Map as FovMap, FovAlgorithm};
use tcod::console::*;
use tcod::colors::{self, Color};

use gen::MapGenerator;
use map::Tile;
use map::Rect;

use object::Object;


const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

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
      width: map_size.0,
      height: map_size.1,
      data: vec![vec![Tile::wall(); map_size.1 as usize]; map_size.0 as usize],
      rooms: vec![],
    }
  }

  pub fn generate_with<Creator>(self, max_rooms: i32, room_min_size: i32, room_max_size: i32) -> (Self,Location)
  where
    Creator:MapGenerator,
  {
    Creator::generate(self, max_rooms, room_min_size, room_max_size)
  }

  pub fn render(&mut self, con: &mut Offscreen, fov_map: &mut FovMap, recompute: bool)  {
    if recompute {
      //recompute FOV if needed (the player moved or somthing happend)
      // go through all tiles, and set their background color
      for x in 0..self.width {
          for y in 0..self.height {
              let visible = fov_map.is_in_fov(x,y);
              let wall = self.data[x as usize][y as usize].block_sight;
              let colour = match (visible,wall) {
                // outside of field of view:
                    (false, true) => COLOR_DARK_WALL,
                    (false, false) => COLOR_DARK_GROUND,
                    // inside fov:
                    (true, true) => COLOR_LIGHT_WALL,
                    (true, false) => COLOR_LIGHT_GROUND,
              };
              con.set_char_background(x, y, colour, BackgroundFlag::Set );
          }
      }

    };
  }
}

