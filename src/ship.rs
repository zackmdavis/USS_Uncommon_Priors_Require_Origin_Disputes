use crate::entity::{SPEED_LIMIT, Entity, Orientation, Position, Spin, Velocity};

#[derive(Debug)]
pub struct Ship {
    name: String,
    position: Position,
    velocity: Velocity,
    // XXX: arbitrarily settable orientation is a hack
    pub orientation: Orientation,
    thrust_strength: f32,
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
        shields: f32,
    ) -> Self {
        Ship {
            name,
            position,
            velocity,
            orientation,
            thrust_strength,
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
        let proposed_velocity = self.velocity + self.orientation.unit_velocity() * self.thrust_strength;
        if proposed_velocity.abs() < SPEED_LIMIT {
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
}
