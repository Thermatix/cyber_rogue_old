use rand;



use game::Map;
use game::entity;
use entity::{Entity, contextual::* ,rand::Rng, colors, Stats, Manager, BlockCheck};


pub fn place_entities(map: &mut Map, ents: &Manager) {
    for room in &map.rooms {
      //choose random number of monsters
      let num_monsters = rand::thread_rng().gen_range(0, ::CONFIG.max_monsters + 1);
      for _ in 0..num_monsters {
        // chose ranomd spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);
        match Entity::blocked(x,y,&map, &ents) {
          BlockCheck::Empty => {
          ents.push(if rand::random::<f32>() < 0.8 {// 80% chance of getting an orc
            let mut mob = Entity::new((x, y), '0',colors::DESATURATED_GREEN,"Orc",Block, entity::Kind::Mob );
            mob.stats = Some(Stats{max_hp: 10, hp: 10, defense: 0, power: 3});
            mob.ai = Some(entity::Ai);
            mob
          }  else {
            let mut mob = Entity::new((x, y),'T', colors::DARKER_GREEN, "Troll",Block, entity::Kind::Mob);
            mob.stats = Some(Stats{max_hp: 16, hp: 16, defense: 1, power: 4});
            mob.ai = Some(entity::Ai);
            mob
          });
          },
          _ => {

          }
        };
      }
    };


}
