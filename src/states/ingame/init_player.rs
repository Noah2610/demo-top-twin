use super::state_prelude::*;

pub(super) fn init_player(world: &mut World) -> Entity {
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
        .with(player_settings.health)
        .with(player_settings.health_display)
        .with(HealthActionQueue::default())
        .with(TakesDamage::default())
        .with(Lifecycle::default())
        .build()
}
