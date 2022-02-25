mod app;
mod ecs_controller;
pub mod engine_sync;
pub mod spinning;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<ecs_controller::ECSController>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
