use crate::entity::{Position, Velocity, Entity};

pub struct Torpedo {
    position: Position,
    velocity: Velocity,
    timer: usize
}

impl Entity for Torpedo {
    fn position(&self) -> Position { self.position }
    fn velocity(&self) -> Velocity { self.velocity }
}

impl Torpedo {
    pub fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity, timer: 500 }
    }

    pub fn position(&self) -> Position { self.position }

    pub fn tick(&mut self) {
        self.position = self.next_position();
        self.timer -= 1;
    }

    pub fn expired(&self) -> bool {
        self.timer == 0
    }
}
