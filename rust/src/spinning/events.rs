use crate::engine_sync::components::{GodotObjRef, PlayingGame};
use crate::spinning::components::*;
use bevy::prelude::{Commands, EventReader, World};
use gdnative::api::MeshInstance;
use gdnative::prelude::*;
use gdrust::unsafe_functions::RefExt;

pub struct SpawnSpinningCube(pub (Ref<MeshInstance>, f32));

pub fn spawn_spinning_cube_listener(
    mut commands: Commands,
    mut on_spawn_spinning_cube: EventReader<SpawnSpinningCube>,
) {
    if let Some(SpawnSpinningCube((mesh, speed))) = on_spawn_spinning_cube.iter().next() {
        let mesh_instance = mesh.expect_safe();
        mesh_instance.set_physics_process(true);

        commands
            .spawn()
            .insert(GodotObjRef::new(mesh.clone()))
            .insert(Cube)
            .insert(RotateSpeed::new(*speed))
            .insert(StartPosition::new(Vector3::ZERO))
            .insert(CountTime::new(0.0))
            .insert(PlayingGame);
    }
}

pub fn spawn_spinning_cube(world: &mut World, node: Ref<MeshInstance>, speed: f32) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnSpinningCube>>()
        .expect(
            "No Events<E> resource found. Did you forget to call `.init_resource` or `.add_event`?",
        )
        .send(SpawnSpinningCube((node.clone(), speed)));
}
