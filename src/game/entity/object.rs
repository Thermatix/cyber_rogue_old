pub use tcod::console::*;
pub use tcod::colors;


use tcod::colors::Color;
// use entity::*;

use game::mapping::Map;
use game::types::Location;
use game::mapping::Tile;


#[derive(Debug,PartialEq)]
pub enum Kind {
  Player,
  Npc,
  Mob,
  Boss,
  Object,
  Item,
  Fixture,
}

pub enum Block_Check<object,tile> {
  Empty,
  Object(object),
  Wall(tile),
}

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
  pub kind: Kind,

}

impl Object {
    pub fn new(init_pos: Location, char: char, color: Color, name: &str, blocks: bool, kind: Kind) -> Self {
        Self {
            x: init_pos.0,
            y: init_pos.1,
            char: char,
            color: color,
            name: name.into(),
            blocks: blocks,
            alive: true,
            kind: kind,
        }
    }

    pub fn blocked(x: i32, y: i32, map: &Map) -> Block_Check<&Object,&Tile> {
      if map[x as usize][y as usize].blocked {
        Block_Check::Wall(&map[x as usize][y as usize])
      } else {
        let res = map.objects.iter().position( |object| {
          object.blocks && object.pos() == (x,y)
        });
        match res {
          Some(index) => Block_Check::Object(&map.objects[index]),
          None => Block_Check::Empty
        }
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
    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) -> bool {
      let nx = self.x + dx;
      let ny = self.y + dy;
      match Self::blocked(nx,ny,&map) {
        Block_Check::Wall(_) => false,
        Block_Check::Object(ref obj) => {
          self.action(&obj);
          false
        },
        Block_Check::Empty=> {
          self.set_pos((nx,ny));
          true
        }
      }
    }

    pub fn action(&self,obj: &Object) {
      match obj.kind {
        Kind::Mob => {
          println!("The {} laughs at your puny efforts to attack him!", obj.name);
        },
        _ => (),
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

}
