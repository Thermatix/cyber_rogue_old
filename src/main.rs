pub extern crate tcod;
pub extern crate rand;

use tcod::colors;

mod game;

use game::entity::Object;
use game::entity::emplacement;
use game::mapping::*;
use self::gen;



mod sys;
use sys::*;

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





fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,  // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),

        _ => {},
    }

    false
}

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
    let mut objects = vec![Object::new(player_start.0, player_start.1, CHARA_CHAR, colors::WHITE)];

    for room in &map.rooms {
      emplacement::place_objects(&room,&mut objects)
    };
    // the list of objects with those two

    let mut previous_player_position = (-1, -1);

    let mut fov_map = render::fov_check(&map);

    while !root.window_closed() {
        // render the screen
        if previous_player_position != (objects[0].x, objects[0].y) {
          render::all(&mut root, &mut con, &objects, &mut map, &mut fov_map);
        }

        root.flush();

        // erase all objects at their old locations, before they move
        for object in &objects {
            object.clear(&mut con)
        }

        // handle keys and exit game if needed
        let player = &mut objects[0];
        previous_player_position = (player.x, player.y);
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break
        }
    }
}
