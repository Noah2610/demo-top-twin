use super::state_prelude::*;
use crate::level_loader::load_level;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();

        let player = init_player(&mut data.world);
        let _camera = init_camera(&mut data.world, player);

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

fn init_player(world: &mut World) -> Entity {
    use crate::components::prelude::*;
    use deathframe::core::geo::prelude::*;
    use std::path::PathBuf;

    let player_settings = world.read_resource::<Settings>().player.clone();

    let mut transform = Transform::default();
    transform.set_translation_z(1.0);

    let size = Size::from(player_settings.size);

    let sprite_render = {
        SpriteRender {
            sprite_sheet:  world
                .read_resource::<SpriteSheetHandles<PathBuf>>()
                .get(&resource("spritesheets/spritesheet.png"))
                .unwrap(),
            sprite_number: 0,
        }
    };

    world
        .create_entity()
        .with(Player::default())
        .with(transform)
        .with(Hitbox::from(vec![Rect::from(&size)]))
        .with(size)
        .with(Collidable::new(CollisionTag::Player))
        .with(Collider::new(CollisionTag::Player))
        .with(Solid::new(SolidTag::Player))
        .with(Velocity::default())
        .with(sprite_render)
        .with(ScaleOnce::default())
        .with(player_settings.base_friction)
        .build()
}

fn init_camera(world: &mut World, player: Entity) -> Entity {
    use crate::components::prelude::*;
    use deathframe::amethyst::renderer::Camera;
    use deathframe::amethyst::utils::ortho_camera::{
        CameraNormalizeMode,
        CameraOrtho,
        CameraOrthoWorldCoordinates,
    };

    let camera_settings = world.read_resource::<Settings>().camera.clone();

    let mut transform = Transform::default();
    transform.set_translation_z(camera_settings.z);

    let size = Size::from(camera_settings.size);

    let camera = Camera::standard_2d(size.w, size.h);
    let mut camera_ortho =
        CameraOrtho::normalized(CameraNormalizeMode::Contain);
    let half_size = size.half();
    camera_ortho.world_coordinates = CameraOrthoWorldCoordinates {
        top:    half_size.h,
        bottom: -half_size.h,
        left:   -half_size.w,
        right:  half_size.w,
    };

    world
        .create_entity()
        .with(transform)
        .with(size)
        .with(camera)
        .with(Follow::new(player))
        .build()
}
