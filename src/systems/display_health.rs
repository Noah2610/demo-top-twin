use super::system_prelude::*;
use crate::resource;
use amethyst::assets::AssetStorage;
use amethyst::ui::{Anchor, FontAsset, FontHandle, UiText, UiTransform};
use deathframe::amethyst;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Default)]
pub struct DisplayHealthSystem {
    registered: HashMap<Entity, (Entity, u16)>,
}

impl<'a> System<'a> for DisplayHealthSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, HealthDisplay>,
        Read<'a, FontHandles<PathBuf>>,
        WriteStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (
            entities,
            health_store,
            health_display_store,
            font_handles,
            mut ui_transform_store,
            mut ui_text_store,
        ): Self::SystemData,
    ) {
        if let Some(font_handle) = font_handles
            .get(&resource("fonts/undefined-medium.ttf"))
            .cloned()
        {
            for (entity, health, health_display) in
                (&entities, &health_store, &health_display_store).join()
            {
                let (health_entity, prev_health) =
                    self.registered.entry(entity).or_insert_with(|| {
                        let new_entity = entities.create();
                        ui_transform_store
                            .insert(
                                new_entity,
                                UiTransform::new(
                                    String::new(),
                                    health_display.anchor,
                                    health_display.pivot,
                                    health_display.offset.0,
                                    health_display.offset.1,
                                    health_display.z,
                                    health_display.size.0,
                                    health_display.size.1,
                                ),
                            )
                            .unwrap();
                        ui_text_store
                            .insert(
                                new_entity,
                                UiText::new(
                                    font_handle.clone(),
                                    health.to_string(),
                                    health_display.color,
                                    health_display.font_size,
                                ),
                            )
                            .unwrap();
                        (new_entity, health.health)
                    });

                if health.health != *prev_health {
                    if let Some(ui_text) = ui_text_store.get_mut(*health_entity)
                    {
                        ui_text.text = health.to_string();
                    }
                }
            }
        }
    }
}
