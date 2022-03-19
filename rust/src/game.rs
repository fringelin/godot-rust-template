use bevy::prelude::{DespawnRecursiveExt, Entity};
use gdnative::prelude::*;
use gdrust::ecs::app::with_world;
use gdrust::ecs::engine_sync::components::{GameNode, GodotObjRef};
use gdrust::ecs::engine_sync::events::spawn_game;
use gdrust::macros::gdrust;

#[gdrust(extends = Spatial)]
pub struct Game {
    pub entity: Option<Entity>,
}

#[methods]
impl Game {
    #[export]
    fn _ready(&mut self, owner: TRef<Spatial>) {
        with_world(|w| {
            let entity = w
                .spawn()
                .insert(GodotObjRef::new(owner.upcast::<Node>().claim()))
                .insert(GameNode)
                .id();
            spawn_game(w, entity);
            self.entity = Some(entity);
        });
    }

    #[export]
    fn free(&mut self, owner: TRef<Spatial>) {
        with_world(|w| {
            w.entity_mut(self.entity.unwrap()).despawn_recursive();
        });
    }
}
