use gdnative::prelude::*;
use gdrust::ecs::app::with_world;
use gdrust::ecs::engine_sync::events::spawn_game;
use gdrust::ecs::engine_sync::resources::GameOver;
use gdrust::macros::gdrust;

#[gdrust(extends = Spatial)]
pub struct Game;
#[methods]
impl Game {
    #[export]
    fn _ready(&mut self, owner: TRef<Spatial>) {
        with_world(|w| {
            spawn_game(w, owner.upcast::<Node>().claim());
        });
    }

    #[export]
    fn free(&mut self, owner: TRef<Spatial>) {
        with_world(|w| {
            let mut game_over = w.get_resource_mut::<Option<GameOver>>().unwrap();
            *game_over = Some(GameOver::Win);
        });
    }
}
