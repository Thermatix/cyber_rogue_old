use tcod::console::*;
use tcod::Color;

use game::mapping::{Map, Tile};

use game::types::Location;

use game::entity::{Ai,Stats};

#[allow(dead_code)]
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





pub enum BlockCheck<Object,Tile> {
  Empty,
  Object(Object),
  Wall(Tile),
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
  pub stats: Option<Stats>,
  pub ai: Option<Ai>,

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
            stats: None,
            ai: None,
        }
    }

    pub fn blocked(x: i32, y: i32, map: &Map) -> BlockCheck<&Object,&Tile> {
      if map[x as usize][y as usize].blocked {
        BlockCheck::Wall(&map[x as usize][y as usize])
      } else {
        let res = map.objects.iter().position( |object| {
          object.blocks && object.pos() == (x,y)
        });
        match res {
          Some(index) => BlockCheck::Object(&map.objects[index]),
          None => BlockCheck::Empty
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
        BlockCheck::Wall(_) => false,
        BlockCheck::Object(ref obj) => {
          self.action(&obj);
          false
        },
        BlockCheck::Empty=> {
          self.set_pos((nx,ny));
          true
        }
      }
    }

    fn action(&self,obj: &Object) {
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
