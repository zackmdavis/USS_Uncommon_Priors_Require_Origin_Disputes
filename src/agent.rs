use ship::Ship;

pub struct Agent {
    pub ship: Ship,
    pub ai: Box<AI>
}

pub trait AI {
    // TODO: provide observations?!
    fn tick(&self, ship: &mut Ship);
}


pub struct MarkI;

impl AI for MarkI {
    fn tick(&self, ship: &mut Ship) {
        ship.reorient_left();
        ship.thrust();
    }
}
