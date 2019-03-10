extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod ship;
mod entity;

use ship::Ship;
use entity::{Position, Velocity, Orientation, Entity};


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
        entity.position().0
    }

    pub fn entity_render_instruction_y(&self, i: u16) -> f32 {
        let entity = match i {
            0 => &self.our_heroine,
            _ => &self.ships[i as usize]
        };
        entity.position().1
    }

    pub fn entity_render_instruction_o(&self, i: u16) -> f32 {
        let entity = match i {
            0 => &self.our_heroine,
            _ => &self.ships[i as usize]
        };
        entity.orientation().0
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
