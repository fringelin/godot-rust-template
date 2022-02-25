pub mod components;

use crate::engine_sync::components::{GodotObjRef, PlayingGame};
use crate::engine_sync::resources::PhysicsDelta;
use crate::engine_sync::stages::SyncStages;
use crate::spinning::components::{Cube, RotateSpeed, StartPosition, Time};
use bevy::prelude::*;
use gdnative::api::MeshInstance;
use gdnative::prelude::*;
use gdrust::unsafe_functions::RefExt;

pub struct SpinningPlugin;
impl Plugin for SpinningPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSpinningCube>()
            .add_system(spawn_spinning_cube)
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, spinning_cube_sync);
    }
}

pub struct SpawnSpinningCube(pub (Ref<MeshInstance>, f32));

fn spawn_spinning_cube(
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
            .insert(Time::new(0.0))
            .insert(PlayingGame);
    }
}

fn spinning_cube_sync(
    delta: Res<PhysicsDelta>,
    mut query: Query<
        (
            &GodotObjRef<MeshInstance>,
            &StartPosition,
            &RotateSpeed,
            &mut Time,
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
