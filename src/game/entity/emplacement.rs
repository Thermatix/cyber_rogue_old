use rand;
use entity::rand::Rng;
use entity::colors;



use game::Map;
use game::Object;
use game::object::BlockCheck;
use game::entity;
use game::entity::Stats;


pub fn place_objects(map: &mut Map) {
    for room in &map.rooms {
      //choose random number of monsters
      let num_monsters = rand::thread_rng().gen_range(0, ::CONFIG.max_monsters + 1);
      for _ in 0..num_monsters {
        // chose ranomd spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);
        match Object::blocked(x,y,&map) {
          BlockCheck::Empty => {
          let mut mob: Object;
          if rand::random::<f32>() < 0.8 {// 80% chance of getting an orc
            mob =  Object::new((x, y), '0',colors::DESATURATED_GREEN,"Orc", true, entity::Kind::Mob );
            mob.stats = Some(Stats{max_hp: 10, hp: 10, defense: 0, power: 3});
            mob.ai = Some(entity::Ai);
          }  else {
            mob = Object::new((x, y),'T', colors::DARKER_GREEN, "Goblin", true, entity::Kind::Mob);
            mob.stats = Some(Stats{max_hp: 16, hp: 16, defense: 1, power: 4});
            mob.ai = Some(entity::Ai);

          };
          map.objects.push(mob);
          },
          _ => {

          }
        };
      }
    };


}
