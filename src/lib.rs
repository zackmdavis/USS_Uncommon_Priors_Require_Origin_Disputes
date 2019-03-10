extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

const ARENA_WIDTH: f32 = 600.;
const ARENA_HEIGHT: f32 = 400.;

#[derive(Copy, Clone, Debug)]
pub struct Position(f32, f32);

#[derive(Copy, Clone, Debug)]
pub struct Velocity(f32, f32);

#[derive(Copy, Clone, Debug)]
pub struct Orientation(f32);

/// why did Rust choose remainder instead of modulus?!
fn modulo(a: f32, b: f32) -> f32 {
    ((a % b) + b) % b
}

trait Entity {
    fn position(&self) -> Position;

    fn velocity(&self) -> Velocity;

    fn next_position(&mut self) -> Position {
        let Position(x, y) = self.position();
        let Velocity(dx, dy) = self.velocity();
        Position(modulo(x + dx, ARENA_WIDTH), modulo(y + dy, ARENA_HEIGHT))
    }
}


struct Ship {
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
    fn new() -> Self {
        Ship {
            position: Position(100., 100.),
            velocity: Velocity(1., 1.),
            orientation: Orientation(0.),
            thrust_strength: 0.3
        }
    }

    fn orientation(&self) -> Orientation { self.orientation }

    fn thrust(&mut self) {
        let Velocity(mut dx, mut dy) = self.velocity;
        dx += self.thrust_strength * self.orientation.0.cos();
        dy += self.thrust_strength * self.orientation.0.sin();
        self.velocity = Velocity(dx, dy);
    }

    fn reorient_left(&mut self) {
        // TODO: implement `Add`; this is hideous
        let new_angle = self.orientation.0 - 0.1;
        self.orientation = Orientation(new_angle);
    }

    fn reorient_right(&mut self) {
        let new_angle = self.orientation.0 + 0.1;
        self.orientation = Orientation(new_angle);
    }

    fn tick(&mut self) {
        self.position = self.next_position();
    }
}

#[wasm_bindgen]
pub struct Arena {
    our_heroine: Ship,
    ships: Vec<Ship>
}

#[wasm_bindgen]
impl Arena {
    fn new() -> Self {
        Arena {
            our_heroine: Ship::new(),
            ships: Vec::new()
        }
    }

    fn add_ship(&mut self, ship: Ship) {
        self.ships.push(ship);
    }

    pub fn tick(&mut self) {
        self.our_heroine.tick();
        for ship in &mut self.ships {
            ship.tick();
        }
    }

    pub fn input_left(&mut self) {
        self.our_heroine.reorient_left();
    }

    pub fn input_right(&mut self) {
        self.our_heroine.reorient_right();
    }

    pub fn input_thrust(&mut self) {
        self.our_heroine.thrust();
    }

    pub fn entity_count(&self) -> u16 {
        (1 + self.ships.len()) as u16
    }

    pub fn entity_render_instruction_x(&self, i: u16) -> f32 {
        let entity = match i {
            0 => &self.our_heroine,
            _ => &self.ships[i as usize]
        };
        entity.position.0
    }

    pub fn entity_render_instruction_y(&self, i: u16) -> f32 {
        let entity = match i {
            0 => &self.our_heroine,
            _ => &self.ships[i as usize]
        };
        entity.position.1
    }

    pub fn entity_render_instruction_o(&self, i: u16) -> f32 {
        let entity = match i {
            0 => &self.our_heroine,
            _ => &self.ships[i as usize]
        };
        entity.orientation.0
    }

    pub fn entity_render_instruction_r(&self, i: u16) -> f32 {
        12.
    }

    pub fn entity_render_instruction_kind(&self, i: u16) -> u8 {
        1
    }

}

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn uncommon_priors_require_origin_disputes() -> Arena {
    log("Hello WASM world in console!");
    let mut arena = Arena::new();
    arena
}
