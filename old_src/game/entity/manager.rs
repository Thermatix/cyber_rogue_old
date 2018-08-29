use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;

use entity::Entity;
use entity::Kind;
use nanoid;

type EntityHash = HashMap<String, Entity>;

type ID = String;

pub struct Manager {
   pub player: Entity,
   pub mobs: EntityHash,
   pub items: EntityHash,
   pub npcs: EntityHash

}


impl Index<ID> for Manager{
  type Output = Option<Entity>;

  fn index(&self, key: &ID) -> &self::Output {
      if self.mobs.contains_key(key){
          Some(&self.mobs[key])
      } else if self.items.contains_key(key){
          Some(&self.items[key])
      }else if self.npcs.contains_key(key){
          Some(&self.npcs[key])
      } else {
          None
      }
  }
}

impl IndexMut<ID> for Manager{

  fn index_mut(&mut self, key: &ID) -> Option<&mut Entity> {
      if self.mobs.contains_key(key){
          Some(&mut self.mobs.get_mut(key).unwrap())
      } else if self.items.contains_key(key){
          Some(&mut self.items.get_mut(key).unwrap())
      }else if self.npcs.contains_key(key){
          Some(&mut self.npcs.get_mut(key).unwrap())
      } else {
          None
      }
  }
}

impl Manager {
    pub fn new(player: Entity) -> Self {
        Self {
            player: player,
            mobs: EntityHash::new(),
            items: EntityHash::new(),
            npcs: EntityHash::new(),
        }
    }

    pub fn push(&mut self, entity: Entity) -> Result<(),&str>  {
        match entity.kind {
            Kind::Mob => {
                self.mobs.insert(self.generate_id(), entity);
                Ok(())
            },
            Kind::Item => {
                self.items.insert(self.generate_id(), entity);
                Ok(())
            },
            Kind::NPC => {
                self.npcs.insert(self.generate_id(), entity);
                Ok(())
            },
            _ => Err("Unsuported enetity type")
        }
    }

    fn generate_id(&self) -> ID{
        let id: ID = nanoid::generate(15);
        while self[id].is_some() {
            id = nanoid::generate(15);
        };
        id
    }
}


