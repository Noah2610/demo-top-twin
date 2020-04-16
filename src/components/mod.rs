pub mod prelude {
    pub use super::health_display::HealthDisplay;
    pub use super::player::Player;
    pub use deathframe::components::prelude::*;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}

mod health_display;
mod player;
