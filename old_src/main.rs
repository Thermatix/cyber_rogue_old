pub extern crate tcod;
pub extern crate rand;
extern crate nanoid;

use tcod::colors;

use tcod::*;

mod game;

use game::entity::*;
use game::mapping::*;
use game::entity::contextual::*;

use self::gen;

mod sys;
use sys::*;
use self::controls::PlayerAction;

const CONFIG: config::Options = config::Options {
// actual size of the window
  screen_width:     80,
  screen_height:    50,
  fps_limit:        20,
//parameters for dungeon generator
  room_min_size:    6,
  room_max_size:    10,
  room_max_no:      30,
  fov_light_walls:  true,
  torch_radius:     10,
  max_monsters:     3,
};

// size of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const CHARA_CHAR: char = '@';
const CLEAR_CHAR: char = ' ';






fn main() {
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(CONFIG.screen_width, CONFIG.screen_height)
        .title("Cyber Rogue")
        .init();
    tcod::system::set_fps(CONFIG.fps_limit);
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    // generate map (at this point it's not drawn to the screen)
    let (mut map, player_start) = Map::new((MAP_WIDTH, MAP_HEIGHT)).generate_with::<gen::dungeon::Basic>(CONFIG.room_max_no, CONFIG.room_min_size, CONFIG.room_max_size);
    // create entities representing the player
    // place the player inside the first room
    let mut ent_manager = Manager::new(Entity::new(player_start, CHARA_CHAR,  colors::WHITE,"" ,Block, Kind::Player));
    ent_manager.player.stats = Some(Stats{max_hp: 30, hp: 30, defense: 2, power: 5});
    

    emplacement::place_entities(&mut map, &mut ent_manager);
    // the list of entities with those two

    let mut previous_player_position = (-1, -1);

    let mut fov_map = render::fov_check(&map);

    while !root.window_closed() {
        // render the screen
        if previous_player_position != (ent_manager.player.x, ent_manager.player.y) {
          render::all(&mut root, &mut con,  &map, &mut ent_manager, &mut fov_map);
        }
        {
          // let obj_len = map.entities.len();
          // for mobId in map.entities.iter_mut() {
          //   if ent_manager[mobId].ai.is_some() {
          //     entity::ai::ai_take_turn(&mut ent_manager[mobId],&ent_manager.player,&map,&fov_map);
          //     render::all(&mut root, &mut con,  &mut map, &mut fov_map);
          //   }
          // }
        }

        root.flush();

        // erase all entities at their old locations, before they move
        render::clear_entities(&map, &mut ent_manager , &mut con);

        previous_player_position = (ent_manager.player.x, ent_manager.player.y);
        // handle keys and exit game if needed
        // build entity manager and only store references to those entities in the map entities,
        // perhaps use hashmap to store and a vec of keys to hose entities
        match controls::handle_keys(&mut root, &mut map, &mut ent_manager) {
          PlayerAction::TookTurn => {
            for id in &map.entities {
                let Some(object) = ent_manager[id.to_string()];
                println!("The {} growls!", object.name);
            }
          },
          PlayerAction::DidntTakeTurn => {

          }
          PlayerAction::Exit => break,
        }
    }
}
