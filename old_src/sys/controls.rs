use tcod::console::*;

use game::entity::Manager;
use game::Map;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
  TookTurn,
  DidntTakeTurn,
  Exit
}



pub fn handle_keys(root: &mut Root, map: &mut Map, ents: &mut Manager) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use self::PlayerAction::*;

    let key = root.wait_for_keypress(true);

    let result =
    match (key, ents.player.alive) {
       (Key { code: Enter, alt: true, .. }, _)=> {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }
       (Key { code: Escape, .. }, _) => Exit,  // exit game

        // movement keys
        (Key { code: Up, .. }, true) => {
          ents.player.move_by(0, -1, map, ents);
          TookTurn
        } ,
        (Key { code: Down, .. }, true) => {
          ents.player.move_by(0, 1, map, ents);
          TookTurn
        },
        (Key { code: Left, .. }, true) => {
          ents.player.move_by(-1, 0, map, ents);
          TookTurn
        },
        (Key { code: Right, .. }, true) => {
          ents.player.move_by(1, 0, map, ents);
          TookTurn
        },
        _ => DidntTakeTurn,
    };
    result
}
