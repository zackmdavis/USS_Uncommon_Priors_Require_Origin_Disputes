use entity::{Position, Velocity, Orientation, Entity};

pub struct Ship {
    position: Position,
    velocity: Velocity,
    orientation: Orientation,
    thrust_strength: f32
}

impl Entity for Ship {
    fn position(&self) -> Position { self.position }
    fn velocity(&self) -> Velocity { self.velocity }
}


impl Ship {
    pub fn new() -> Self {
        Ship {
            position: Position(100., 100.),
            velocity: Velocity(1., 1.),
            orientation: Orientation(0.),
            thrust_strength: 0.3
        }
    }

    pub fn orientation(&self) -> Orientation { self.orientation }

    pub fn thrust(&mut self) {
        let Velocity(mut dx, mut dy) = self.velocity;
        dx += self.thrust_strength * self.orientation.0.cos();
        dy += self.thrust_strength * self.orientation.0.sin();
        self.velocity = Velocity(dx, dy);
    }

    pub fn reorient_left(&mut self) {
        // TODO: implement `Add`; this is hideous
        let new_angle = self.orientation.0 - 0.1;
        self.orientation = Orientation(new_angle);
    }

    pub fn reorient_right(&mut self) {
        let new_angle = self.orientation.0 + 0.1;
        self.orientation = Orientation(new_angle);
    }

    pub fn tick(&mut self) {
        self.position = self.next_position();
    }
}
