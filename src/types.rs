use ggez::{event, glam::*, graphics::{self, Color}, Context, GameResult, ContextBuilder};
use oorandom::Rand32;

const INITIAL_PARTICLES: u64 = 1;
pub const WINDOW_WIDTH: u32 = 2000;
pub const WINDOW_HEIGHT: u32 = 2000;
pub const GRAVITY: f32 = 100.0;
pub const TARGET_FPS: u32 = 60;
pub const ROOM_RADIUS: f32 = 900.0;
pub const SUB_STEPS: u8 = 2;
pub const SPAWN_TIME: f32 = 0.1;

// Write a function that gets the distance between two points
fn distance(a: &Vec2, b: &Vec2) -> f32 {
    (a - b).length()
}

#[derive(Clone)]
pub(crate) struct Particle {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub acc: Vec2,
    pub radius: f32,
    pub circle: graphics::Mesh,
}

impl Particle {
    pub fn new(ctx: &mut Context, pos: Vec2, radius: f32, color: Color) -> Self {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            radius,
            0.2,
            color,
        ).unwrap();
        Self {
            pos,
            prev_pos: pos,
            acc: Vec2::new(0.0, 0.0),
            radius,
            circle,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let vel: Vec2 = self.pos - self.prev_pos;
        self.prev_pos = self.pos;
        self.pos = self.pos + vel + self.acc * dt * dt;
        self.acc = Vec2::ZERO;
        // check_window_bounds(self);
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acc += acc;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        canvas.draw(&self.circle, self.pos);
        Ok(())
    }
}

fn random_vec(rng: &mut Rand32, lower_bound: (f32, f32), upper_bound: (f32, f32)) -> Vec2 {
    let x = rng.rand_range(0..((upper_bound.0 - lower_bound.0) as u32)) as f32 + lower_bound.0;
    let y = rng.rand_range(0..((upper_bound.1 - lower_bound.1) as u32)) as f32 + lower_bound.1;
    Vec2::new(x, y)
}

fn random_colour(rng: &mut Rand32) -> Color {
    let r = rng.rand_range(0..255) as f32 / 255.0;
    let g = rng.rand_range(0..255) as f32 / 255.0;
    let b = rng.rand_range(0..255) as f32 / 255.0;
    let a: f32 = 1.0;
    Color::from([r, g, b, a])
}

pub(crate) struct MainState {
    pub particles: Vec<Particle>,
    pub rng: Rand32,
    pub total_time: f32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));


        let mut main_state = Self {
            particles: Vec::new(),
            rng,
            total_time: 0.0,
        };
        for _ in 0..INITIAL_PARTICLES {
            Self::spawn_particle(&mut main_state, ctx);
        }

        Ok(main_state)
    }

    pub fn spawn_particle(&mut self, ctx: &mut Context) {
        let offset = random_vec(&mut self.rng, (-1.0, -1.0), (1.0, 1.0));
        let pos= Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0) + offset;
        let radius = self.rng.rand_range(5..50) as f32;
        self.particles.push(Particle::new(
            ctx,
            pos,
            radius,
            random_colour(&mut self.rng),
        ));
    }

    pub fn update_positions(&mut self, dt: f32) {
        self.particles.iter_mut().for_each(|p| {
            p.update_position(dt);
        });
    }

    pub fn apply_gravity(&mut self) {
        self.particles.iter_mut().for_each(|p| {
            p.accelerate(Vec2::new(0.0, GRAVITY));
        });
    }

    pub fn draw_particles(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) {
        self.particles.iter().for_each(|p| {
            p.draw(ctx, canvas).unwrap();
        });
    }

    pub fn apply_collisions(&mut self) {
        let response_coef: f32 = 0.75;
        let particle_count = self.particles.len();

        // iterate between all pairs of particles and change position
        for i in 0..particle_count {

            for j in i + 1..particle_count {
                let p1 = &mut self.particles[i].clone();
                let p2 = &mut self.particles[j].clone();
                let v = p1.pos - p2.pos;
                let dist2 = v.x * v.x + v.y * v.y;
                let min_dist = p1.radius + p2.radius;

                // Check overlapping
                if dist2 < min_dist * min_dist {
                    let dist = f32::sqrt(dist2);
                    let n = v / dist;
                    let mass_ratio_1 = p1.radius / (p1.radius + p2.radius);
                    let mass_ratio_2 = p2.radius / (p1.radius + p2.radius);
                    let delta = (dist - min_dist) * response_coef * 0.5;
                    // Update positions
                    p1.pos -= n * delta * mass_ratio_2;
                    p2.pos += n * delta * mass_ratio_1;
                    self.particles[i] = p1.clone();
                    self.particles[j] = p2.clone();
                }
            }
        }
    }

    pub fn apply_constraints(&mut self) {
        let position: Vec2 = Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);
        self.particles.iter_mut().for_each(|p| {
            let to_obj: Vec2 = position - p.pos;
            let distance: f32 = f32::sqrt(to_obj.x * to_obj.x + to_obj.y * to_obj.y); //p.pos.distance(position);

            if distance > ROOM_RADIUS - p.radius {
                let n: Vec2 = to_obj / distance;
                p.pos = position - n * (ROOM_RADIUS - p.radius);
            }
        });
    }

}