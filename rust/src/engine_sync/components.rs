use bevy::prelude::*;
use gdnative::prelude::*;
use std::ops::Deref;

#[derive(Component)]
pub struct GodotObjRef<T: GodotObject> {
    value: Ref<T>,
}

impl<T: GodotObject> GodotObjRef<T> {
    pub fn new(value: Ref<T>) -> Self {
        Self { value }
    }
}

impl<T: GodotObject> Deref for GodotObjRef<T> {
    type Target = Ref<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Component)]
pub struct GameNode;
#[derive(Component)]
pub struct PlayingGame;
