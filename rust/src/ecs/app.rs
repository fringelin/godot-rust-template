use crate::ecs::plugins::spinning::SpinningPlugin;
use bevy::{diagnostic::DiagnosticsPlugin, log::LogPlugin, prelude::*};

use super::plugins::engine_sync::EngineSyncPlugin;

pub fn get_ecs() -> App {
    let mut ecs = App::new();
    ecs.add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(EngineSyncPlugin)
        .add_plugin(SpinningPlugin); // order matters here

    ecs
}
