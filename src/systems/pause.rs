use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System, Write},
    input::InputHandler,
};

pub struct PauseSystem;

impl<'a> System<'a> for PauseSystem {
    type SystemData = (Write<'a, Time>, Read<'a, InputHandler<String, String>>);

    fn run(&mut self, (mut time, input): Self::SystemData) {
        if let Some(true) = input.action_is_down("pause") {
            let old_ts = time.time_scale();
            &time.set_time_scale(1.0 - old_ts);
        }
    }
}
