use bevy::{ecs::system::Resource, prelude::World};
use gdnative::{
    api::{InputEvent, Node},
    prelude::{Ref, TRef},
};

use super::resources::Delta;

pub const PRESSED_ACTIONS: [&str; 4] = ["ui_left", "ui_right", "ui_up", "ui_down"];

pub struct SpawnGame(pub Ref<Node>);

pub struct DespawnPlayingGame;

pub struct UserInput(pub Action);

pub enum Action {
    Pressed(&'static str),
    Released(&'static str),
}

pub fn update_delta_resource<T: Resource + Delta>(world: &mut World, delta: f32) {
    world
        .get_resource_mut::<T>()
        .expect("Umm, there should be a godot time here!")
        .set_delta(delta);
}

pub fn user_input(world: &mut World, event: TRef<InputEvent>) {
    let mut send_user_input = world
        .get_resource_mut::<bevy::app::Events<UserInput>>()
        .expect("should be a user input event");
    for action in PRESSED_ACTIONS.iter() {
        if event.is_action(action, false) {
            if event.is_pressed() && !event.is_echo() {
                send_user_input.send(UserInput(Action::Pressed(action)));
            } else if !event.is_pressed() && !event.is_echo() {
                send_user_input.send(UserInput(Action::Released(action)));
            }
        }
    }
}

pub fn spawn_game(world: &mut World, node: Ref<Node>) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnGame>>()
        .expect("No world spawn game event, did you forget to add Spawn Game into your events?")
        .send(SpawnGame(node));
}
