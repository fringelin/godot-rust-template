mod components;
mod game;
mod spinning;

use crate::game::Game;
use crate::spinning::{SpinningBundle, SpinningPlugin};
use gdnative::prelude::{godot_init, InitHandle};
use gdrust::ecs::app::init_ecs;
use gdrust::ecs::engine_controller::ECSController;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<ECSController>();
    handle.add_class::<Game>();
    handle.add_class::<SpinningBundle>();

    init_ecs(SpinningPlugin);
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
