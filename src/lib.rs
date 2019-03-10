extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod entity;
mod ship;
mod torpedo;

use ship::Ship;
use torpedo::Torpedo;
use entity::{Position, Velocity, Orientation, Entity};


pub enum EntityType {
    OurHeroine,
    Ship,
    Torpedo
}

#[wasm_bindgen]
pub struct Arena {
    our_heroine: Ship,
    ships: Vec<Ship>,
    torpedos: Vec<Torpedo>
}

#[wasm_bindgen]
impl Arena {
    fn new() -> Self {
        Arena {
            our_heroine: Ship::new(),
            ships: Vec::new(),
            torpedos: Vec::new()
        }
    }

    fn add_ship(&mut self, ship: Ship) {
        self.ships.push(ship);
    }

    fn add_torpedo(&mut self, torpedo: Torpedo) {
        self.torpedos.push(torpedo);
    }

    pub fn tick(&mut self) {
        self.our_heroine.tick();
        for ship in &mut self.ships {
            ship.tick();
        }
        for torpedo in &mut self.torpedos {
            torpedo.tick();
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

    pub fn input_fire(&mut self) {
        let Velocity(mut dx, mut dy) = self.our_heroine.velocity();
        dx += 0.7 * self.our_heroine.orientation().0.cos();
        dy += 0.7 * self.our_heroine.orientation().0.sin();
        let torpedo = Torpedo::new(
            // TODO: slight displacement?!
            self.our_heroine.position(),
            Velocity(dx, dy)
        );
        self.add_torpedo(torpedo);
    }

    pub fn entity_count(&self) -> u16 {
        (1 + self.ships.len() + self.torpedos.len()) as u16
    }

    fn entity(&self, i: u16) -> (EntityType, &dyn Entity) {
        let ship_count = self.ships.len() as u16;
        if i == 0 {
            (EntityType::OurHeroine, &self.our_heroine)
        } else if i >= 1 && i <= ship_count {
            (EntityType::Ship, &self.ships[(i-1) as usize])
        } else {
            (EntityType::Torpedo,
             &self.torpedos[(i-1-ship_count) as usize])
        }
    }

    pub fn entity_render_instruction_x(&self, i: u16) -> f32 {
        let (_, entity) = self.entity(i);
        entity.position().0
    }

    pub fn entity_render_instruction_y(&self, i: u16) -> f32 {
        let (_, entity) = self.entity(i);
        entity.position().1
    }

    pub fn entity_render_instruction_o(&self, i: u16) -> f32 {
        match i {
            0 => self.our_heroine.orientation().0,
            // TODO: other ships
            _ => 0. // dummy value
        }
    }

    pub fn entity_render_instruction_r(&self, i: u16) -> f32 {
        let (entity_type, _) = self.entity(i);
        match entity_type {
            EntityType::OurHeroine | EntityType::Ship => 10.,
            EntityType::Torpedo => 2.
        }
    }

    pub fn entity_render_instruction_kind(&self, i: u16) -> u8 {
        let (entity_type, _) = self.entity(i);
        match entity_type {
            EntityType::OurHeroine => 1,
            EntityType::Ship => 2,
            EntityType::Torpedo => 3
        }
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
