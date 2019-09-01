use wasm_bindgen;

use wasm_bindgen::prelude::*;

mod agent;
mod entity;
mod ship;
mod torpedo;

#[allow(unused_imports)]
use crate::agent::{Agent, PatrolAI,  HunterAI, SensorSweep};
use crate::entity::{Entity, Orientation, Position, Velocity};
use crate::ship::Ship;
use crate::torpedo::Torpedo;

pub enum EntityType {
    OurHeroine,
    Ship,
    Torpedo(bool /* ready */),
}

#[wasm_bindgen]
pub struct Arena {
    our_heroine: Ship,
    agents: Vec<Agent>,
    torpedos: Vec<Torpedo>,
}

#[wasm_bindgen]
impl Arena {
    fn new() -> Self {
        Arena {
            our_heroine: Ship::new(
                "Uncommon Priors Require Origin Disputes".to_owned(),
                Position(100., 100.),
                Velocity(0., 0.),
                Orientation(0.),
                0.3,
                100.,
            ),
            agents: vec![
                Agent {
                    ship: Ship::new(
                        "Discovery".to_owned(),
                        Position(100., 150.),
                        Velocity(0., 0.),
                        Orientation(-3.),
                        0.2,
                        100.,
                    ),
                    ai: Box::new(PatrolAI::new(vec![
                        Position(100., 100.),
                        Position(100., 300.),
                        Position(500., 300.),
                        Position(500., 100.),
                    ])),
                },
                Agent {
                    ship: Ship::new(
                        "Valiant".to_owned(),
                        Position(40., 40.),
                        Velocity(0., 0.),
                        Orientation(-3.),
                        0.2,
                        100.,
                    ),
                    ai: Box::new(PatrolAI::new(vec![
                        Position(100., 100.),
                        Position(100., 300.),
                        Position(500., 300.),
                        Position(500., 100.),
                    ])),
                },
                Agent {
                    ship: Ship::new(
                        "Defiant".to_owned(),
                        Position(400., 240.),
                        Velocity(0., 0.),
                        Orientation(-3.),
                        0.2,
                        100.,
                    ),
                    ai: Box::new(PatrolAI::new(vec![
                        Position(500., 300.),
                        Position(500., 100.),
                        Position(100., 100.),
                        Position(100., 300.),
                    ])),
                },
                Agent {
                    ship: Ship::new(
                        "Titanic".to_owned(),
                        Position(350., 140.),
                        Velocity(0., 0.),
                        Orientation(-3.),
                        0.2,
                        100.,
                    ),
                    ai: Box::new(HunterAI {}),
                },
            ],
            torpedos: Vec::new(),
        }
    }

    fn add_torpedo(&mut self, torpedo: Torpedo) {
        self.torpedos.push(torpedo);
    }

    pub fn tick(&mut self) {
        self.our_heroine.tick();
        let sensors = SensorSweep {
            heroine_position: self.our_heroine.position()
        };
        for agent in &mut self.agents {
            agent.ai.tick(&mut agent.ship, &sensors);
            agent.ship.tick();
        }
        // separate pass to avoid double-borrow
        for i in (0..self.agents.len()).rev() {
            if self.agents[i].ship.shields < 0. {
                self.agents.swap_remove(i);
            }
        }
        for i in (0..self.torpedos.len()).rev() {
            let mut boom = false;
            self.torpedos[i].tick();
            if !self.torpedos[i].ready() {
                continue;
            }
            if self.torpedos[i]
                .position()
                .distance_to(self.our_heroine.position())
                < 10.
            {
                if self.our_heroine.shields > 1. {  // plot armor
                    self.our_heroine.shields -= 8.;
                }
                boom = true;
            }
            for agent in &mut self.agents {
                if self.torpedos[i]
                    .position()
                    .distance_to(agent.ship.position())
                    < 10.
                {
                    agent.ship.shields -= 9.1;
                    boom = true;
                }
            }
            if self.torpedos[i].expired() || boom {
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
        let torpedo = Torpedo::new(self.our_heroine.position(), velocity);
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
            (EntityType::Ship, &self.agents[(i - 1) as usize].ship)
        } else {
            (
                EntityType::Torpedo(self.torpedos[(i - 1 - ship_count) as usize].ready()),
                &self.torpedos[(i - 1 - ship_count) as usize],
            )
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
            self.agents[(i - 1) as usize].ship.orientation().0
        } else {
            0. // dummy value
        }
    }

    pub fn entity_render_instruction_r(&self, i: u16) -> f32 {
        let (entity_type, _) = self.entity(i);
        match entity_type {
            EntityType::OurHeroine | EntityType::Ship => 10.,
            EntityType::Torpedo(_) => 2.,
        }
    }

    pub fn entity_render_instruction_kind(&self, i: u16) -> u8 {
        let (entity_type, _) = self.entity(i);
        match entity_type {
            EntityType::OurHeroine => 1,
            EntityType::Ship => 2,
            EntityType::Torpedo(false) => 3,
            EntityType::Torpedo(true) => 4,
        }
    }

    pub fn entity_render_instruction_shields(&self, i: u16) -> f32 {
        // XXX: code duplication
        let ship_count = self.agents.len() as u16;
        if i == 0 {
            self.our_heroine.shields()
        } else if i >= 1 && i <= ship_count {
            self.agents[(i - 1) as usize].ship.shields()
        } else {
            0. // dummy value
        }
    }
}

#[wasm_bindgen]
extern "C" {
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
