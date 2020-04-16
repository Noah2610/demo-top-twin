use crate::resource;
use crate::resources::prelude::*;
use crate::settings::Settings;
use crate::states::aliases::{CustomData, GameData, GameDataBuilder};
use amethyst::core::frame_limiter::FrameRateLimitConfig;
use amethyst::utils::app_root_dir::application_root_dir;
use deathframe::amethyst;

pub fn run() -> amethyst::Result<()> {
    use crate::states::prelude::Startup;
    use amethyst::ApplicationBuilder;

    start_logger();

    let settings = Settings::load()?;
    let game_data = build_game_data(&settings)?;

    let mut game: amethyst::CoreApplication<GameData> =
        ApplicationBuilder::new(application_root_dir()?, Startup::default())?
            .with_frame_limit_config(frame_rate_limit_config()?)
            .with_resource(settings)
            .build(game_data)?;

    game.run();

    Ok(())
}

fn start_logger() {
    use amethyst::{LogLevelFilter, LoggerConfig};
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });
}

fn frame_rate_limit_config() -> amethyst::Result<FrameRateLimitConfig> {
    use std::fs::File;
    Ok(ron::de::from_reader(File::open(resource(
        "config/frame_limiter.ron",
    ))?)?)
}

fn build_game_data<'a, 'b>(
    _settings: &Settings,
) -> amethyst::Result<GameDataBuilder<'a, 'b>> {
    use crate::input::prelude::*;
    use crate::systems::prelude::*;
    use amethyst::core::transform::TransformBundle;
    use amethyst::renderer::types::DefaultBackend;
    use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
    use amethyst::ui::{RenderUi, UiBundle};
    use amethyst::utils::fps_counter::FpsCounterBundle;
    use amethyst::utils::ortho_camera::CameraOrthoSystem;
    use deathframe::bundles::*;

    let transform_bundle = TransformBundle::new();
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(resource("config/display.ron"))?
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderUi::default())
        .with_plugin(RenderFlat2D::default());
    let audio_bundle = AudioBundle::<SoundKey, SongKey>::default();
    let menu_input_bundle = MenuBindings::bundle()?;
    let ingame_input_bundle = IngameBindings::bundle()?;
    let physics_bundle =
        PhysicsBundle::<CollisionTag, SolidTag>::new().with_deps(&[]);
    let animation_bundle = AnimationBundle::<AnimationKey>::new();

    let custom_game_data = GameDataBuilder::default()
        .custom(CustomData::default())
        .dispatcher(DispatcherId::MainMenu)?
        .dispatcher(DispatcherId::Ingame)?
        .dispatcher(DispatcherId::Paused)?
        .with_core_bundle(FpsCounterBundle)?
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(rendering_bundle)?
        .with_core_bundle(audio_bundle)?
        .with_core_bundle(menu_input_bundle)?
        .with_core_bundle(UiBundle::<MenuBindings>::new())?
        .with_core(PrintFpsSystem::default(), "print_fps_system", &[])?
        .with_core(CameraOrthoSystem::default(), "camera_ortho_system", &[])?
        .with_core(ScaleSpritesSystem::default(), "scale_sprites_system", &[])?
        .with_bundle(DispatcherId::Ingame, ingame_input_bundle)?
        .with_bundle(DispatcherId::Ingame, physics_bundle)?
        .with_bundle(DispatcherId::Ingame, animation_bundle)?
        .with(
            DispatcherId::MainMenu,
            InputManagerSystem::<MenuBindings>::default(),
            "main_menu_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            InputManagerSystem::<IngameBindings>::default(),
            "ingame_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Paused,
            InputManagerSystem::<MenuBindings>::default(),
            "paused_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            FollowSystem::default(),
            "follow_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            ConfineEntitiesSystem::default(),
            "confine_entities_system",
            &["move_entities_system"],
        )?
        .with(
            DispatcherId::Ingame,
            EntityLoaderSystem::default(),
            "entity_loader_system",
            &[
                "move_entities_system",
                "follow_system",
                "confine_entities_system",
            ],
        )?
        .with(
            DispatcherId::Ingame,
            ControlPlayerSystem::default(),
            "control_player_system",
            &["ingame_input_manager_system"],
        )?;

    Ok(custom_game_data)
}
