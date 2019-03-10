const ARENA_WIDTH: f32 = 600.;
const ARENA_HEIGHT: f32 = 400.;

/// why did Rust choose remainder instead of modulus?!
fn modulo(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
}

#[derive(Copy, Clone, Debug)]
pub struct Position(pub f32, pub f32);

#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub f32, pub f32);

#[derive(Copy, Clone, Debug)]
pub struct Orientation(pub f32);

pub trait Entity {
    fn position(&self) -> Position;

    fn velocity(&self) -> Velocity;

    fn next_position(&mut self) -> Position {
        let Position(x, y) = self.position();
        let Velocity(dx, dy) = self.velocity();
        Position(modulo(x + dx, ARENA_WIDTH), modulo(y + dy, ARENA_HEIGHT))
    }
}
