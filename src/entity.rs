use std::ops::{AddAssign, Sub, SubAssign, Mul};

const ARENA_WIDTH: f32 = 600.;
const ARENA_HEIGHT: f32 = 400.;

/// why did Rust choose remainder instead of modulus?!
fn modulo(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
}

#[derive(Copy, Clone, Debug)]
pub struct Position(pub f32, pub f32);

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Position) -> Position {
        Position(self.0 - other.0, self.1 - other.1)
    }
}

impl Position {
    pub fn distance_to(&self, other: Position) -> f32 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }

    pub fn orientation_to(&self, other: Position) -> Orientation {
        let displacement = other - *self;
        Orientation((displacement.1/displacement.0).atan())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity(pub f32, pub f32);

impl AddAssign for Velocity {
    fn add_assign(&mut self, other: Velocity) {
        *self = Velocity(self.0 + other.0, self.1 + other.1);
    }
}

impl SubAssign for Velocity {
    fn sub_assign(&mut self, other: Velocity) {
        *self = Velocity(self.0 - other.0, self.0 - other.1);
    }
}

impl Mul<f32> for Velocity {
    type Output = Velocity;

    fn mul(self, scalar: f32) -> Velocity {
        Velocity(self.0 * scalar, self.1 * scalar)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Orientation(pub f32);

impl AddAssign for Orientation {
    fn add_assign(&mut self, other: Orientation) {
        *self = Orientation(self.0 + other.0);
    }
}

impl Sub for Orientation {
    type Output = Self;

    fn sub(self, other: Orientation) -> Orientation {
        Orientation(self.0 - other.0)
    }
}

impl SubAssign for Orientation {
    fn sub_assign(&mut self, other: Orientation) {
        *self = Orientation(self.0 - other.0);
    }
}

impl Orientation {
    pub fn unit_velocity(&self) -> Velocity {
        let (dx, dy) = self.0.sin_cos();
        Velocity(dx, dy)
    }
}

pub trait Entity {
    fn position(&self) -> Position;

    fn velocity(&self) -> Velocity;

    fn next_position(&mut self) -> Position {
        let Position(x, y) = self.position();
        let Velocity(dx, dy) = self.velocity();
        Position(modulo(x + dx, ARENA_WIDTH), modulo(y + dy, ARENA_HEIGHT))
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use super::{Orientation, Position};

    #[test]
    fn concerning_adding_orientations() {
        let mut o = Orientation(0.);
        o += Orientation(0.1);
        assert_eq!(o, Orientation(0.1));
    }

    #[test]
    fn concerning_orientation_to() {
        let origin = Position(0., 0.);
        let x = Position(1., 1.);
        assert_eq!(origin.orientation_to(x), Orientation(PI/4.));
    }
}
