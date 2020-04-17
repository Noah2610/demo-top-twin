mod init_camera;
mod init_player;

use super::state_prelude::{self, *};
use crate::level_loader::load_level;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();

        let player = init_player::init_player(&mut data.world);
        let _camera = init_camera::init_camera(&mut data.world, player);

        let level_name = {
            use std::env;
            env::args().skip(1).next().unwrap_or("level.map".into())
        };

        load_level(level_name, &mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        Trans::None
    }
}
