use bevy::{prelude::*};

mod et_scene;
mod et_game;

use et_scene::et_main_menu::ETMainMenu;
use et_scene::r#trait::ETScene;


fn main() {
    let mut app = App::new();
    et_game::ETGame::new(&mut app)
        .run();
}

fn setup() {

}


