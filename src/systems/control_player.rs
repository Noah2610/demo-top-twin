use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Read<'a, InputManager<IngameBindings>>,
        Read<'a, Time>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            settings,
            input_manager,
            time,
            player_store,
            mut velocity_store,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;
        let speed = settings.player.speed;
        let max_speed = settings.player.max_speed;

        for (_, velocity) in (&player_store, &mut velocity_store).join() {
            for axis in Axis::iter() {
                if let Some(val) = input_manager.axis_value(
                    (IngameAxis::PlayerX, IngameAxis::PlayerY).by_axis(&axis),
                ) {
                    velocity.increase_with_max(
                        &axis,
                        speed.by_axis(&axis) * dt * val,
                        max_speed.by_axis(&axis),
                    );
                }
            }
        }
    }
}
