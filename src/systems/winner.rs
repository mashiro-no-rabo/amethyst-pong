use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, System, WriteStorage},
};

use crate::pong::{Ball, ARENA_WIDTH};

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (WriteStorage<'s, Ball>, WriteStorage<'s, Transform>);

    fn run(&mut self, (mut balls, mut locals): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // Right player scored :)
                println!("Player 2 scored!");
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // Left player scored :(
                println!("Player 1 scored..");
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0]; // Reverse direction horizontally
                transform.set_x(ARENA_WIDTH / 2.0); // Reset position
            }
        }
    }
}
