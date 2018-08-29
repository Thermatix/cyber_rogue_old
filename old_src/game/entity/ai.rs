use tcod::map::Map as FovMap;

use game::Map;
use game::entity::Entity;
use game::entity::Manager;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ai;


pub fn move_towards(mob: &mut Entity, player: &Entity, target_x: i32, target_y: i32, map: &mut Map, ents: &mut Manager) {
  let dx = target_x - player.x;
  let dy = target_y - player.y;

  let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

  let dx = (dx as f32 / distance ).round() as i32;
  let dy = (dy as f32 / distance ).round() as i32;
  mob.move_by(dx,dy,&mut map,&mut ents);
}

pub fn ai_take_turn(mob: &mut Entity, player: &Entity, map: &mut Map, ents: &mut Manager, fov_map: &FovMap) {
  if fov_map.is_in_fov(mob.x,mob.y) {
    if mob.distance_to(player) >= 2.0 {
      let (px, py) = player.pos();
      move_towards(mob,&player,px,py,map,&mut ents);
    }
  }
}
