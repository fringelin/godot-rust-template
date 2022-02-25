pub mod components;
pub mod events;

use crate::engine_sync::components::GodotObjRef;
use crate::engine_sync::resources::PhysicsDelta;
use crate::engine_sync::stages::SyncStages;
use crate::spinning::components::{CountTime, Cube, RotateSpeed, StartPosition};
use crate::spinning::events::{spawn_spinning_cube_listener, SpawnSpinningCube};
use bevy::prelude::*;
use gdnative::api::MeshInstance;
use gdnative::prelude::*;
use gdrust::unsafe_functions::RefExt;

pub struct SpinningPlugin;
impl Plugin for SpinningPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSpinningCube>()
            .add_system(spawn_spinning_cube_listener)
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, spinning_cube_sync);
    }
}

fn spinning_cube_sync(
    delta: Res<PhysicsDelta>,
    mut query: Query<
        (
            &GodotObjRef<MeshInstance>,
            &StartPosition,
            &RotateSpeed,
            &mut CountTime,
        ),
        With<Cube>,
    >,
) {
    if let Some((mesh, start, rotate_speed, mut time)) = query.iter_mut().next() {
        let mesh_instance = mesh.expect_safe();

        use gdnative::api::SpatialMaterial;

        time.value += delta.value;
        mesh_instance.rotate_y((rotate_speed.value * delta.value) as f64);

        let offset = Vector3::UP * time.cos() * 0.5;
        mesh_instance.set_translation(start.value + offset);

        if let Some(mat) = mesh_instance.get_surface_material(0) {
            let mat = mat.expect_safe();
            let mat = mat.cast::<SpatialMaterial>().expect("Incorrect material");
            mat.set_albedo(Color::from_rgba(time.cos().abs(), 0.0, 0.0, 1.0));
        }
    }
}
