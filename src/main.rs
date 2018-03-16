extern crate tcod;

mod object;
mod map;

use object::Object;
use map::*;


use tcod::console::*;
use tcod::colors::{self, Color};
use ::tcod::map::{Map as FovMap, FovAlgorithm};

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// size of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

//parameters for dungeon generator
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

const LIMIT_FPS: i32 = 20;  // 20 frames-per-second maximum


const CHARA_CHAR: char = '@';
const CLEAR_CHAR: char = ' ';


const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;




  fn fov_check(map: &Map, fov_map: &mut FovMap) {
    for x in 0..map.width {
      for y in 0..map.height {
        fov_map.set(x,y,
                    !map.data[x as usize][y as usize].block_sight,
                    !map.data[x as usize][y as usize].blocked);
      }
    }
  }


fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &mut Map, fov_map: &mut FovMap, fov_recompute: bool) {
    // go through all tiles, and set their background color
  fov_map.compute_fov(objects[0].x, objects[0].y, TORCH_RADIUS, FOV_LIGHT_WALLS,FOV_ALGO);

  map.render(con, fov_map, fov_recompute);

    // draw all objects in the list
    for object in objects {
        if fov_map.is_in_fov(object.x, object.y) {
            object.draw(con);
        }
    }

    // blit the contents of "con" to the root console
    blit(con, (0, 0), (map.width, map.height), root, (0, 0), 1.0, 1.0);
}

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
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Cyber Rogue")
        .init();
    tcod::system::set_fps(LIMIT_FPS);
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    // generate map (at this point it's not drawn to the screen)
    let (mut map, player_start) = Map::new((MAP_WIDTH, MAP_HEIGHT)).generate_with::<gen::dungeon::Basic>(MAX_ROOMS,ROOM_MIN_SIZE, ROOM_MAX_SIZE);

    // create object representing the player
    // place the player inside the first room
    let player = Object::new(player_start.0, player_start.1, CHARA_CHAR, colors::WHITE);
    let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, CHARA_CHAR, colors::YELLOW);

    // the list of objects with those two
    let mut objects = [player, npc];

    let mut previous_player_position = (-1, -1);

    fov_check(&map,&mut fov_map);

    while !root.window_closed() {
        // render the screen
        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(&mut root, &mut con, &objects, &mut map, &mut fov_map, fov_recompute);

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
