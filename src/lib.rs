extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


#[derive(Copy, Clone, Debug)]
pub struct Position(f32, f32);

#[derive(Copy, Clone, Debug)]
pub struct Velocity(f32, f32);

#[derive(Copy, Clone, Debug)]
pub struct Orientation(f32);

trait Entity {
    fn position(&self) -> Position;

    fn velocity(&self) -> Velocity;

    fn tick(&mut self) -> Position {
        let Position(x, y) = self.position();
        let Velocity(dx, dy) = self.velocity();
        Position(x + dx, y + dy)
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
    fn orientation(&self) -> Orientation { self.orientation }
}

struct Arena {
    ships: Vec<Ship>
}

impl Arena {
    fn tick(&mut self) {
        for ship in &mut self.ships {
            ship.tick();
        }
    }
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
    // doesn't work </3
    //
    // fn renderCircle(x: f32, y: f32, r: f32);
}

#[wasm_bindgen]
pub fn rah() {
    alert("hello");
    // renderCircle(100., 100., 10.);
}
