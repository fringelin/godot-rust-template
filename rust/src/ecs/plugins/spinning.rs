use crate::ecs::plugins::engine_sync::components::{GodotObjRef, PlayingGame};
use crate::ecs::plugins::engine_sync::resources::PhysicsDelta;
use crate::ecs::plugins::engine_sync::stages::SyncStages;
use bevy::prelude::*;
use gdnative::api::MeshInstance;
use gdnative::prelude::*;

pub struct SpinningPlugin;
impl Plugin for SpinningPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSpinningCube>()
            .add_system(spawn_spinning_cube)
            .add_system_to_stage(SyncStages::UpdateBevyPhysics, spinning_cube_sync);
    }
}

#[derive(Component)]
pub struct Cube;

#[derive(Component)]
pub struct StartPosition(pub Vector3);

#[derive(Component)]
pub struct Time(f32);

#[derive(Component)]
pub struct RotateSpeed(f32);

pub struct SpawnSpinningCube(pub (Ref<MeshInstance>, f32));

fn spawn_spinning_cube(
    mut commands: Commands,
    mut on_spawn_spinning_cube: EventReader<SpawnSpinningCube>,
) {
    if let Some(SpawnSpinningCube((mesh, speed))) = on_spawn_spinning_cube.iter().next() {
        let mesh_instance = unsafe { mesh.assume_safe() };
        mesh_instance.set_physics_process(true);

        commands
            .spawn()
            .insert(GodotObjRef(mesh.clone()))
            .insert(Cube)
            .insert(RotateSpeed(*speed))
            .insert(StartPosition(Vector3::ZERO))
            .insert(Time(0.0))
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
        let mesh_instance = unsafe { mesh.0.assume_safe() };

        use gdnative::api::SpatialMaterial;

        time.0 += delta.0 as f32;
        mesh_instance.rotate_y((rotate_speed.0 * delta.0) as f64);

        let offset = Vector3::UP * time.0.cos() * 0.5;
        mesh_instance.set_translation(start.0 + offset);

        if let Some(mat) = mesh_instance.get_surface_material(0) {
            let mat = unsafe { mat.assume_safe() };
            let mat = mat.cast::<SpatialMaterial>().expect("Incorrect material");
            mat.set_albedo(Color::from_rgba(time.0.cos().abs(), 0.0, 0.0, 1.0));
        }
    }
}
