use tcod::console::*;

use game::Map;



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
  TookTurn,
  DidntTakeTurn,
  Exit
}



pub fn handle_keys(root: &mut Root, map: &mut Map) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use PlayerAction::*;

    let mut player = map.objects.remove(0);
    let key = root.wait_for_keypress(true);

    let result =
    match (key,map.objects[0].alive) {
       (Key { code: Enter, alt: true, .. }, _)=> {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }
       (Key { code: Escape, .. }, _) => Exit,  // exit game

        // movement keys
        (Key { code: Up, .. }, true) => {
          player.move_by(0, -1, map);
          TookTurn
        } ,
        (Key { code: Down, .. }, true) => {
          player.move_by(0, 1, map);
          TookTurn
        },
        (Key { code: Left, .. }, true) => {
          player.move_by(-1, 0, map);
          TookTurn
        },
        (Key { code: Right, .. }, true) => {
          player.move_by(1, 0, map);
          TookTurn
        },
        _ => DidntTakeTurn,
    };
    map.objects.insert(0,player);
    result
}
