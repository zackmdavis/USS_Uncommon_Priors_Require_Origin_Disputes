use wasm_bindgen;

use wasm_bindgen::prelude::*;

mod agent;
mod entity;
mod ship;
mod torpedo;

use crate::agent::{Agent, PatrolAI};
use crate::ship::Ship;
use crate::torpedo::Torpedo;
use crate::entity::{Position, Velocity, Orientation, Entity};


pub enum EntityType {
    OurHeroine,
    Ship,
    Torpedo
}

#[wasm_bindgen]
pub struct Arena {
    our_heroine: Ship,
    agents: Vec<Agent>,
    torpedos: Vec<Torpedo>
}

#[wasm_bindgen]
impl Arena {
    fn new() -> Self {
        Arena {
            our_heroine: Ship::new(
                Position(100., 100.),
                Velocity(0., 0.),
                Orientation(0.),
                0.3
            ),
            agents: vec![
                Agent {
                    ship: Ship::new(
                        Position(300., 250.),
                        Velocity(0.1, -0.2),
                        Orientation(0.),
                        0.2
                    ),
                    ai: Box::new(PatrolAI::new(vec![Position(100., 100.),
                                                    Position(500., 350.)]))
                }
            ],
            torpedos: Vec::new()
        }
    }

    fn add_torpedo(&mut self, torpedo: Torpedo) {
        self.torpedos.push(torpedo);
    }

    pub fn tick(&mut self) {
        // log(&format!("heroine orientation: {:?}", self.our_heroine.orientation()));
        self.our_heroine.tick();
        for agent in &mut self.agents {
            // log(&format!("agent orientation: {:?}", agent.ship.orientation()));
            agent.ai.tick(&mut agent.ship);
            agent.ship.tick();
        }
        for i in (0..self.torpedos.len()).rev() {
            self.torpedos[i].tick();
            if self.torpedos[i].boom() {
                self.torpedos.swap_remove(i);
            }
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
        let mut velocity = self.our_heroine.velocity();
        velocity += self.our_heroine.orientation().unit_velocity() * 0.7;
        let torpedo = Torpedo::new(
            // TODO: slight displacement?!
            self.our_heroine.position(),
            velocity
        );
        self.add_torpedo(torpedo);
    }

    pub fn entity_count(&self) -> u16 {
        (1 + self.agents.len() + self.torpedos.len()) as u16
    }

    fn entity(&self, i: u16) -> (EntityType, &dyn Entity) {
        let ship_count = self.agents.len() as u16;
        if i == 0 {
            (EntityType::OurHeroine, &self.our_heroine)
        } else if i >= 1 && i <= ship_count {
            (EntityType::Ship, &self.agents[(i-1) as usize].ship)
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
        // XXX: code duplication
        let ship_count = self.agents.len() as u16;
        if i == 0 {
            self.our_heroine.orientation().0
        } else if i >= 1 && i <= ship_count {
            self.agents[(i-1) as usize].ship.orientation().0
        } else {
            0. // dummy value
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
    let arena = Arena::new();
    arena
}
