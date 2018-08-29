use tcod::console::*;
use tcod::Color;

use game::mapping::{Map, Tile};

use game::types::Location;

use game::entity::{Ai,Stats, Manager};

#[allow(dead_code)]
#[derive(Debug,Clone,PartialEq)]
pub enum Kind {
  Player,
  NPC,
  Mob,
  Boss,
  Object,
  Item,
  Fixture,
}




pub enum BlockCheck<Entity,Tile> {
  Empty,
  Entity(Entity),
  Wall(Tile),
}

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Debug,Clone)]
pub struct  Entity{
  pub x: i32,
  pub y: i32,
  pub char: char,
  pub color: Color,
  pub name: String,
  pub blocks: bool,
  pub alive: bool,
  pub kind: Kind,
  pub stats: Option<Stats>,
  pub ai: Option<Ai>,

}

impl  Entity {
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

    pub fn blocked<'e, 'm>(x: i32, y: i32, map: &'m Map, ents: &'e Manager) -> BlockCheck<&'e Entity,&'m Tile> {
      if map[x as usize][y as usize].blocked {
        BlockCheck::Wall(&map[x as usize][y as usize])
      } else {
        match map.entities.iter().find( |id| {
            let &Some(entity) = &ents[id.to_string()];
             entity.blocks && entity.pos() == (x,y)
        }) {
            Some(id) => {
                let &Some(entity) = &ents[id.to_string()];
                BlockCheck::Entity(&entity)
            },
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
    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map, ents: &Manager) -> bool {
      let nx = self.x + dx;
      let ny = self.y + dy;
      match Self::blocked(nx,ny,&map,&ents) {
        BlockCheck::Wall(_) => false,
        BlockCheck::Entity(ref obj) => {
          self.action(&obj);
          false
        },
        BlockCheck::Empty=> {
          self.set_pos((nx,ny));
          true
        }
      }
    }

    pub fn distance_to(&self, other: &Entity) -> f32 {
      let dx = other.x - self.x;
      let dy = other.y - self.y;
      ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    fn action(&self,obj: &Entity) {
      match obj.kind {
        Kind::Mob => {
          println!("The {} laughs at the {}s efforts to attack them!", obj.name,self.name);
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
