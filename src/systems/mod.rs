pub mod prelude {
    pub use deathframe::systems::prelude::*;
}

mod system_prelude {
    pub use crate::components::prelude::*;
    pub use crate::resources::prelude::*;
    pub use deathframe::systems::system_prelude::*;
}
