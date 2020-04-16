use super::component_prelude::*;
use deathframe::amethyst::ui::Anchor;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct HealthDisplay {
    pub offset:    (f32, f32),
    pub z:         f32,
    pub size:      (f32, f32),
    pub color:     [f32; 4],
    pub font_size: f32,
    pub anchor:    Anchor,
    pub pivot:     Anchor,
}
