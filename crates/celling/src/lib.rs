use bevy::prelude::*;

mod systems;
mod res;
mod components;
mod comm;

use res::*;

pub struct CellingPlugin {}
impl Default for CellingPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for CellingPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CellsMap::default())
        .insert_resource(res::settings::Settings::default())
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, (systems::rigids::rigidize,))
        .add_systems(Update, (
            systems::cells::handle,
            systems::rigids::handle,
            systems::load::spawn_image_sprite_handle,
        ))
        .add_systems(PostUpdate, (
            systems::cells::handle_update_map,
            systems::cells::handle_debug,
        ));
    }
}

fn setup(
) {
    
}

pub mod prelude {
    pub use crate::systems::*;
    pub use crate::res::*;
    pub use crate::components::*;
    pub use crate::comm::*;
    pub use crate::CellingPlugin;
}