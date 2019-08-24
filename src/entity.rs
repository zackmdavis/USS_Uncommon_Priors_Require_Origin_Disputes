use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg};

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

impl Sub for Velocity {
    type Output = Self;

    fn sub(self, other: Velocity) -> Velocity {
        Velocity(self.0 - other.0, self.1 - other.1)
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

impl Velocity {
    pub fn abs(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }

    pub fn countering_orientation(&self) -> Orientation {
        -Orientation(self.1.atan2(self.0))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Orientation(pub f32);
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Spin(pub f32);


impl Add<Spin> for Orientation {
    type Output = Self;

    fn add(self, other: Spin) -> Self {
        Orientation(modulo(self.0 + other.0, 2.*PI))
    }
}

impl Sub for Orientation {
    type Output = Spin;

    fn sub(self, other: Orientation) -> Spin {
        let raw = self.0 - other.0;
        if raw.abs() > PI {
            // TODO: simplify?! (formula discovered "empirically")
            Spin(-1. * ((2.*PI).copysign(raw) - raw))
        } else {
            Spin(raw)
        }
    }
}

impl Neg for Orientation {
    type Output = Self;

    fn neg(self) -> Orientation {
        Orientation(modulo(self.0 + PI, 2.*PI))
    }
}

impl Orientation {
    pub fn unit_velocity(&self) -> Velocity {
        let (dy, dx) = self.0.sin_cos();
        Velocity(dx, dy)
    }
}

impl Sub for Spin {
    type Output = Self;

    fn sub(self, other: Spin) -> Spin {
        Spin(self.0 - other.0)
    }
}

impl Spin {
    fn abs(&self) -> f32 {
        self.0.abs()
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
    use super::{Orientation, Position, Velocity, Spin};

    // XXX TODO FIXME: what am I failing to understand about macro imports?
    macro_rules! assert_eq_within_eps {
        // crude edit of the canonical `assert_eq!`
        ($left:expr, $right:expr, $eps:expr) => ({
            match (&($left), &($right)) {
                (left_val, right_val) => {
                    if (*left_val - *right_val).abs() > $eps {
                        panic!("assertion failed: left and right not within ε={} \
                                (left: `{:?}`, right: `{:?}`)", $eps, left_val, right_val)
                    }
                }
            }
        })
    }

    #[test]
    fn concerning_orientation_to() {
        let origin = Position(0., 0.);
        let x = Position(1., 1.);
        assert_eq!(origin.orientation_to(x), Orientation(PI/4.));
    }

    #[test]
    fn concerning_unit_velocity() {
        let cases = vec![
            (Orientation(0.).unit_velocity(), Velocity(1., 0.)),
            (Orientation(PI).unit_velocity(), Velocity(-1., 0.)),
        ];
        for (actual, expected) in cases {
            assert_eq_within_eps!(expected.0, actual.0, 0.0001);
            assert_eq_within_eps!(expected.1, actual.1, 0.0001);
        }
    }

    #[test]
    fn concerning_countering_orientation() {
        let w = Velocity(-1., -1.).countering_orientation();
        assert_eq_within_eps!(w.0, PI/4., 0.0001);
    }

    #[test]
    fn concerning_orientation_differences_wtf() {
        // Need to consistently report minimal spin (no branch
        // discontinuity at zero)
        assert_eq_within_eps!(
            Orientation(0.4) - Orientation(0.2),
            Spin(0.2),
            0.001
        );
        assert_eq_within_eps!(
            Orientation(0.2) - Orientation(0.4),
            Spin(-0.2),
            0.001
        );
        assert_eq_within_eps!(
             Orientation(2.*PI - 0.1) - Orientation(0.1),
            Spin(-0.2),
            0.001
        );
        assert_eq_within_eps!(
            Orientation(0.1) - Orientation(2.*PI - 0.1),
            Spin(0.2),
            0.001
        );
    }

}
