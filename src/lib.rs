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

trait Entity {
    fn position(&self) -> Position;

    fn velocity(&self) -> Velocity;

    fn next_position(&mut self) -> Position {
        let Position(x, y) = self.position();
        let Velocity(dx, dy) = self.velocity();
        Position((x + dx) % ARENA_WIDTH, (y + dy) % ARENA_HEIGHT)
    }
}


struct Ship {
    position: Position,
    velocity: Velocity,
    orientation: Orientation,
}

impl Entity for Ship {
    fn position(&self) -> Position { self.position }
    fn velocity(&self) -> Velocity { self.velocity }
}


impl Ship {
    fn new() -> Self {
        Ship {
            position: Position(100., 100.),
            velocity: Velocity(10., 10.),
            orientation: Orientation(0.),
        }
    }

    fn orientation(&self) -> Orientation { self.orientation }

    fn tick(&mut self) {
        self.position = self.next_position();
    }
}

#[wasm_bindgen]
pub struct Arena {
    ships: Vec<Ship>
}

#[wasm_bindgen]
impl Arena {
    fn new() -> Self {
        Arena { ships: Vec::new() }
    }

    fn add_ship(&mut self, ship: Ship) {
        self.ships.push(ship);
    }

    pub fn tick(&mut self) {
        for ship in &mut self.ships {
            ship.tick();
        }
    }

    pub fn entity_count(&self) -> u16 {
        self.ships.len() as u16
    }

    pub fn entity_render_instruction_x(&self, i: u16) -> f32 {
        let entity = &self.ships[i as usize];
        entity.position.0
    }

    pub fn entity_render_instruction_y(&self, i: u16) -> f32 {
        let entity = &self.ships[i as usize];
        entity.position.1
    }

    pub fn entity_render_instruction_r(&self, i: u16) -> f32 {
        10.
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
    let our_heroine = Ship::new();
    arena.add_ship(our_heroine);
    arena
}
