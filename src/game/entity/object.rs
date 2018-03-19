
use super::*;
use tcod::colors::Color;
// use entity::*;

use game::types::Location;

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Debug)]
pub struct Object {
  pub  x: i32,
  pub  y: i32,
  pub  char: char,
  pub  color: Color,
  pub name: String,
  pub blocks: bool,
  pub alive: bool,

}

impl Object {
    pub fn new(init_pos: Location, char: char, name: &str, color: Color, blocks: bool) -> Self {
        Object {
            x: init_pos.0,
            y: init_pos.1,
            char: char,
            color: color,
            name: name.into(),
            blocks: blocks,
            alive: false,
        }
    }

    pub fn pos(&self) -> Location {
      (self.x, self.y)
    }

    pub fn set_pos(&mut self, pos: Location) {
      self.x = pos.0;
      self.y = pos.1;
    }

    /// move by the given amount, if the destination is not blocked
    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
      let nx = self.x + dx;
      let ny = self.y + dy;
      if !Self::blocked((nx,ny),&map) {
        self.set_pos((nx, ny));
      };
    }

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    /// Erase the character that represents this object
    pub fn clear(&self, con: &mut Console) {
      //figure out better way to inject clear char
        con.put_char(self.x, self.y, ::CLEAR_CHAR, BackgroundFlag::None);
    }

    fn blocked(pos: Location, map: &Map) -> bool {
      if map[pos.0 as usize][pos.1 as usize].blocked {
        true
      } else {
        map.objects.iter().any( |object| {
          object.blocks && object.pos() == pos
        })
      }

    }
}
