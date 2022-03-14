use bevy::prelude::*;
use gdnative::export::Export;
use gdnative::prelude::*;

#[derive(Component, Default, Copy, Clone)]
pub struct Cube;

#[derive(Component, Default, Copy, Clone)]
pub struct StartPosition {
    pub value: Vector3,
}

#[derive(Component, Default, Copy, Clone, FromVariant, ToVariant)]
pub struct CountTime {
    pub time: f32,
}

impl Export for CountTime {
    type Hint = ();

    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(VariantType::Dictionary)
    }
}

#[derive(Component, Default, Copy, Clone, FromVariant, ToVariant)]
pub struct RotateSpeed {
    pub speed: f32,
}

impl Export for RotateSpeed {
    type Hint = ();

    fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
        ExportInfo::new(VariantType::Dictionary)
    }
}
