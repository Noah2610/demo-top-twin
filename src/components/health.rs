use super::component_prelude::*;
use std::fmt;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Health {
    pub health: u16,
    pub max:    u16,
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.health, self.max)
    }
}
