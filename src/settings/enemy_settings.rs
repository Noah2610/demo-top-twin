//! resources/settings/enemy.ron

use crate::components::prelude::*;

#[derive(Clone, Deserialize)]
pub struct EnemySettings {
    pub damage: HitPoints,
    pub health: Health,
}
