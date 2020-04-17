mod block;

use crate::components::prelude::*;
use crate::resource;
use crate::resources::prelude::*;
use crate::settings::prelude::*;
use amethyst::ecs::{World, WorldExt};
use amethyst::prelude::*;
use block::prelude::*;
use deathframe::amethyst;
use deathframe::core::geo::prelude::*;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const ENEMY_DAMAGE: HitPoints = 1;

pub fn load_level<S>(file: S, world: &mut World)
where
    S: fmt::Display,
{
    let mut level_file =
        File::open(resource(format!("levels/{}", file))).unwrap();
    let mut level_raw = String::new();
    level_file.read_to_string(&mut level_raw).unwrap();

    let level_settings = world.read_resource::<Settings>().level.clone();

    let mut blocks = Vec::new();

    let block_size = Size::from(level_settings.tile_size);

    for (y, line) in level_raw.lines().rev().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            blocks.push(Block {
                block_type: BlockType::from(chr),
                pos:        (x as f32 * block_size.w, y as f32 * block_size.h),
            });
        }
    }

    let hitbox = Hitbox::from(vec![Rect::from(&block_size)]);
    let (sprite_render_wall, sprite_render_enemy) = {
        let sprite_sheet = world
            .read_resource::<SpriteSheetHandles<PathBuf>>()
            .get(&resource("spritesheets/spritesheet.png"))
            .unwrap();

        (
            SpriteRender {
                sprite_sheet:  sprite_sheet.clone(),
                sprite_number: 1,
            },
            SpriteRender {
                sprite_sheet,
                sprite_number: 2,
            },
        )
    };

    for block in blocks {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            block.pos.0,
            block.pos.1,
            block.block_type.z(),
        );

        let mut entity_builder = world
            .create_entity()
            .with(transform)
            .with(block_size.clone())
            .with(ScaleOnce::default());

        entity_builder = match &block.block_type {
            BlockType::Empty => entity_builder,

            BlockType::Wall => entity_builder
                .with(Collidable::new(CollisionTag::Solid))
                .with(Solid::new(SolidTag::Solid))
                .with(hitbox.clone())
                .with(sprite_render_wall.clone()),

            BlockType::Enemy => entity_builder
                .with(Collidable::new(CollisionTag::Enemy))
                .with(hitbox.clone())
                .with(sprite_render_enemy.clone())
                .with(Velocity::default())
                .with(DealsDamage::new(ENEMY_DAMAGE)),
        };

        let _entity = entity_builder.build();
    }
}
