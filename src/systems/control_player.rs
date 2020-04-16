use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Read<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            settings,
            input_manager,
            player_store,
            mut velocity_store,
        ): Self::SystemData,
    ) {
        let speed = settings.player.speed;

        for (_, velocity) in (&player_store, &mut velocity_store).join() {
            for axis in Axis::iter() {
                if let Some(val) = input_manager.axis_value(
                    (IngameAxis::PlayerX, IngameAxis::PlayerY).by_axis(&axis),
                ) {
                    velocity.set(&axis, speed.by_axis(&axis) * val);
                }
            }
        }
    }
}
