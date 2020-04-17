use crate::components::prelude::HitPoints;

#[derive(Clone, Deserialize)]
pub struct EnemySettings {
    pub damage: HitPoints,
}
