use entity::rand;
use entity::rand::Rng;
use entity::colors;

use game::Map;
use game::Object;


pub fn place_objects(map: &mut Map) {
    for room in &map.rooms {
      //choose random number of monsters
      let num_monsters = rand::thread_rng().gen_range(0, ::CONFIG.max_monsters + 1);
      for _ in 0..num_monsters {
        // chose ranomd spot for this monster
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);
        if !Object::blocked(x,y,&map) {
          let mut monster = if rand::random::<f32>() < 0.8 {// 80% chance of getting an orc
            Object::new((x, y), '0',"Orc", colors::DESATURATED_GREEN, true)
          }  else {
            Object::new((x, y),'T', "Goblin", colors::DARKER_GREEN,true)
          };
          map.objects.push(monster);
        };
      }
    };


}
