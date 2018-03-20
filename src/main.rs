pub extern crate tcod;
pub extern crate rand;


use tcod::colors;

mod game;

use game::entity::*;
use game::mapping::*;

use self::gen;

mod sys;
use sys::*;
use sys::controls::PlayerAction;

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
    // create object representing the player
    // place the player inside the first room
    map.objects.push(Object::new(player_start, CHARA_CHAR,  colors::WHITE,"" ,true, Kind::Player));
    map.objects[0].stats = Some(Stats{max_hp: 30, hp: 30, defense: 2, power: 5});

    emplacement::place_objects(&mut map);
    // the list of objects with those two

    let mut previous_player_position = (-1, -1);

    let mut fov_map = render::fov_check(&map);

    while !root.window_closed() {
        // render the screen
        if previous_player_position != (map.objects[0].x, map.objects[0].y) {
          render::all(&mut root, &mut con,  &mut map, &mut fov_map);
        }

        root.flush();

        // erase all objects at their old locations, before they move
        render::clear_objects(&mut map, &mut con);

        previous_player_position = (map.objects[0].x, map.objects[0].y);
        // handle keys and exit game if needed
        match controls::handle_keys(&mut root, &mut map) {
          PlayerAction::TookTurn => {
            for object in &map.objects {

              if object.kind != Kind::Player {
                // println!("The {} growls!", object.name);
              }
            }
          },
          PlayerAction::DidntTakeTurn => {

          }
          PlayerAction::Exit => break,
        }
    }
}
