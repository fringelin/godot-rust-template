use bevy::prelude::*;
use gdnative::prelude::*;
use gdrust::macros::*;

#[derive(Component)]
pub struct Cube;

#[single_value(extends = Vector3)]
#[derive(Component)]
pub struct StartPosition {
    pub value: Vector3,
}

#[single_value(extends = f32)]
#[derive(Component)]
pub struct CountTime {
    pub value: f32,
}

#[single_value(extends = f32)]
#[derive(Component)]
pub struct RotateSpeed {
    pub value: f32,
}
