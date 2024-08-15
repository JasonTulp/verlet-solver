use ggez::{event, glam::*, graphics::{self, Color}, Context, GameResult, ContextBuilder};

mod types;
use types::*;

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("verlet_solver", "Jason")
        .window_setup(ggez::conf::WindowSetup::default().title("Verlet Solver"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        let dt: f32 = ctx.time.delta().as_secs_f32();
        self.total_time += dt;

        if self.total_time > SPAWN_TIME {
            self.total_time = 0.0;
            self.spawn_particle(ctx)
        }


        for _ in 0..SUB_STEPS {
            self.apply_gravity();
            self.apply_collisions();
            self.apply_constraints();
            self.update_positions(dt);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.05, 0.05, 0.1, 1.0]));

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0,0.0),
            ROOM_RADIUS,
            0.1,
            graphics::Color::from([0.0, 0.0, 0.01, 1.0]),
        ).unwrap();

        canvas.draw(&circle, Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0));
        self.draw_particles(&mut canvas, ctx);

        canvas.finish(ctx)?;

        Ok(())
    }
}
