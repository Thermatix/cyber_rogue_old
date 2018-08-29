use tcod::map::Map as FovMap;
use tcod::console::*;
use tcod::colors::Color;


use game::mapping;
use game::entity::Entity;

use self::mapping::Tile;
use self::mapping::Rect;
use self::mapping::gen::MapGenerator;

use std::ops::Index;
use std::ops::IndexMut;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

use game::types::*;

pub struct Map {
  pub height: i32,
  pub width: i32,
  pub data: Grid<Tile>,
  pub rooms: Vec<Rect>,
  pub entities: Vec<String>,
}

impl Index<usize> for Map {
  type Output = Vec<Tile>;

  fn index(&self, x: usize) -> &Vec<Tile> {
    &self.data[x]
  }
}

impl IndexMut<usize> for Map{

  fn index_mut(&mut self, x: usize) -> &mut Vec<Tile> {
    &mut self.data[x]
  }
}

impl Map {
  pub fn new(map_size: Point) -> Self {
  Self {
      width: map_size.0,
      height: map_size.1,
      data: vec![vec![Tile::wall(); map_size.1 as usize]; map_size.0 as usize],
      rooms: vec![],
      entities: vec![]
    }
  }

  pub fn generate_with<Creator>(self, max_rooms: i32, room_min_size: i32, room_max_size: i32) -> (Self,Location)
  where
    Creator:MapGenerator,
  {
    Creator::generate(self, max_rooms, room_min_size, room_max_size)
  }

  pub fn render(&mut self, con: &mut Offscreen, fov_map: &FovMap)  {
    // go through all tiles, and set their background color
    for x in 0..self.width {
        for y in 0..self.height {
            let visible = fov_map.is_in_fov(x,y);
            let wall = self[x as usize][y as usize].block_sight;
            let explored = &mut self[x as usize][y as usize].explored;
            if visible {
              *explored = true
            }
            let colour = match (visible,wall) {
              // outside of field of view:
                  (false, true) => COLOR_DARK_WALL,
                  (false, false) => COLOR_DARK_GROUND,
                  // inside fov:
                  (true, true) => COLOR_LIGHT_WALL,
                  (true, false) => COLOR_LIGHT_GROUND,
            };
            if *explored {
              con.set_char_background(x, y, colour, BackgroundFlag::Set );
            }
        }
    };

  }
}

