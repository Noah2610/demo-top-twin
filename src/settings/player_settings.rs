use crate::components::prelude::*;

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:           (f32, f32),
    pub speed:          (f32, f32),
    pub max_speed:      (f32, f32),
    pub base_friction:  BaseFriction,
    pub health:         Health,
    pub health_display: HealthDisplay,
}
