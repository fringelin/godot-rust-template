use crate::components::*;
use bevy::prelude::{App, Bundle, Component, Plugin, Query, Res, With};
use gdnative::api::MeshInstance;
use gdnative::prelude::*;
use gdrust::ecs::app::with_world;
use gdrust::ecs::engine_sync::resources::PhysicsDelta;
use gdrust::ecs::engine_sync::stages::SyncStages;
use gdrust::macros::gdrust;
use gdrust::unsafe_functions::RefExt;

#[derive(Component, Clone)]
pub struct Spinning {
    pub owner: Ref<MeshInstance>,
}

#[gdrust(extends = MeshInstance)]
#[derive(Bundle, Clone)]
pub struct SpinningBundle {
    #[default(Spinning{ owner: _owner.claim() })]
    pub spin: Spinning,
    pub start_pos: StartPosition,
    #[export]
    #[default(RotateSpeed{ speed: 1.})]
    pub rotate_speed: RotateSpeed,
    pub count_time: CountTime,
}
#[methods]
impl SpinningBundle {
    #[export]
    fn _ready(&mut self, owner: TRef<MeshInstance>) {
        owner.set_physics_process(true);

        with_world(|w| {
            w.spawn().insert_bundle(self.clone()).insert(Cube);
        });
    }
}

pub struct SpinningPlugin;
impl Plugin for SpinningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(SyncStages::UpdateBevyPhysics, spinning_cube_sync);
    }
}

fn spinning_cube_sync(
    delta: Res<PhysicsDelta>,
    mut query: Query<(&Spinning, &StartPosition, &RotateSpeed, &mut CountTime), With<Cube>>,
) {
    if let Some((mesh, start, rotate_speed, mut time)) = query.iter_mut().next() {
        let mesh_instance = mesh.owner.expect_safe();

        use gdnative::api::SpatialMaterial;

        time.time += delta.value;
        mesh_instance.rotate_y((rotate_speed.speed * delta.value) as f64);

        let offset = Vector3::UP * time.time.cos() * 0.5;
        mesh_instance.set_translation(start.value + offset);

        if let Some(mat) = mesh_instance.get_surface_material(0) {
            let mat = mat.expect_safe();
            let mat = mat.cast::<SpatialMaterial>().expect("Incorrect material");
            mat.set_albedo(Color::from_rgba(time.time.cos().abs(), 0.0, 0.0, 1.0));
        }
    }
}
