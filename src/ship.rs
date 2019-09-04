use crate::entity::{Entity, Orientation, Position, Spin, Velocity};
use crate::torpedo::Torpedo;

#[derive(Debug)]
pub struct Ship {
    name: String,
    position: Position,
    velocity: Velocity,
    orientation: Orientation,
    thrust_strength: f32,
    top_speed: f32,
    pub shields: f32,
}

impl Entity for Ship {
    fn position(&self) -> Position {
        self.position
    }
    fn velocity(&self) -> Velocity {
        self.velocity
    }
}

impl Ship {
    pub fn new(
        name: String,
        position: Position,
        velocity: Velocity,
        orientation: Orientation,
        thrust_strength: f32,
        top_speed: f32,
        shields: f32,
    ) -> Self {
        Ship {
            name,
            position,
            velocity,
            orientation,
            thrust_strength,
            top_speed,
            shields,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn thrust_strength(&self) -> f32 {
        self.thrust_strength
    }

    pub fn shields(&self) -> f32 {
        self.shields
    }

    pub fn thrust(&mut self) {
        let proposed_velocity =
            self.velocity + self.orientation.unit_velocity() * self.thrust_strength;
        if proposed_velocity.abs() < self.top_speed {
            self.velocity = proposed_velocity;
        }
    }

    pub fn reorient_left(&mut self) {
        self.orientation = self.orientation + Spin(-0.1);
    }

    pub fn reorient_right(&mut self) {
        self.orientation = self.orientation + Spin(0.1);
    }

    pub fn tick(&mut self) {
        self.position = self.next_position();
    }

    pub fn summon_torpedo(&self) -> Torpedo {
        let mut velocity = self.velocity();
        velocity += self.orientation().unit_velocity() * 0.7;
        Torpedo::new(self.position(), velocity)
    }
}
