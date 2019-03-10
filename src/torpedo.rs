use entity::{Position, Velocity, Entity};

pub struct Torpedo {
    position: Position,
    velocity: Velocity
}

impl Entity for Torpedo {
    fn position(&self) -> Position { self.position }
    fn velocity(&self) -> Velocity { self.velocity }
}

impl Torpedo {
    pub fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }

    pub fn tick(&mut self) {
        self.position = self.next_position();
    }
}
