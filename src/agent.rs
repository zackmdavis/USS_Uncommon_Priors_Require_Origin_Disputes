use crate::entity::{Entity, Position, Spin};
use crate::torpedo::Torpedo;
use crate::log;
use crate::ship::Ship;

const CRUISING_SPEED: f32 = 1.;

pub struct Agent {
    pub ship: Ship,
    pub ai: Box<dyn AI>,
}

pub struct SensorSweep {
    pub heroine_position: Position,
}

pub trait AI {
    fn tick(&mut self, ship: &mut Ship, sensors: &SensorSweep) -> Option<Torpedo>;
}

#[derive(Debug, PartialEq)]
enum Mode {
    Orient,
    Accel,
    Glide,
    Disorient,
    Deaccel,
}

#[derive(Debug)]
pub struct PatrolAI {
    waypoints: Vec<Position>,
    next: usize,
    mode: Mode,
}

impl PatrolAI {
    pub fn new(waypoints: Vec<Position>) -> Self {
        PatrolAI {
            waypoints,
            next: 0,
            mode: Mode::Orient,
        }
    }

    pub fn orient(&mut self, ship: &mut Ship) {
        let waypoint = self.waypoints[self.next];
        let heading = ship.position().orientation_to(waypoint);
        let diff: Spin = heading - ship.orientation();
        if diff.0.abs() < 0.1 {
            log(&format!(
                "{} switching to Accel mode on heading {:?} to waypoint {:?}",
                ship.name(),
                heading,
                waypoint
            ));
            self.mode = Mode::Accel;
            return;
        }
        if diff.0 > 0. {
            ship.reorient_right();
        } else {
            ship.reorient_left();
        }
    }

    pub fn accel(&mut self, ship: &mut Ship) {
        if ship.velocity().abs() < CRUISING_SPEED {
            ship.thrust();
        } else {
            log(&format!("{} switching to Glide mode", ship.name()));
            self.mode = Mode::Glide;
        }
    }

    fn slowdown_distance(&mut self, ship: &mut Ship) -> f32 {
        let heading = ship.velocity().countering_orientation();
        let orientation_diff = heading - ship.orientation();
        // XXX: reorientation magic number
        let turnaround_ticks = orientation_diff.0.abs() / 0.1;

        let speed = ship.velocity().abs();
        let deaccel_ticks = speed / ship.thrust_strength();
        let ticks = turnaround_ticks + deaccel_ticks;
        // XXX approximation (correct answer would need to integrate
        // over slowdown): ticks * (distance/tick) = distance
        ticks * speed
    }

    pub fn glide(&mut self, ship: &mut Ship) {
        let waypoint = self.waypoints[self.next];
        let slowdown_distance = self.slowdown_distance(ship);
        if ship.position().distance_to(waypoint) < slowdown_distance {
            log(&format!(
                "{} switching to Disorient mode with slowdown distance {} to waypoint {:?}",
                ship.name(),
                slowdown_distance,
                waypoint
            ));
            self.mode = Mode::Disorient;
        }
    }

    pub fn disorient(&mut self, ship: &mut Ship) {
        let heading = ship.velocity().countering_orientation();
        let diff: Spin = heading - ship.orientation();
        if diff.0.abs() <= 0.05 {
            log(&format!("{} switching to Deaccel mode", ship.name()));
            self.mode = Mode::Deaccel;
            return;
        }
        if diff.0 > 0. {
            ship.reorient_right();
        } else {
            ship.reorient_left();
        }
    }

    pub fn deaccel(&mut self, ship: &mut Ship) {
        if ship.velocity().abs() > 0.1 {
            ship.thrust();
        } else {
            log(&format!(
                "{} switching to Orient mode for next waypoint!",
                ship.name()
            ));
            self.next = (self.next + 1) % self.waypoints.len();
            self.mode = Mode::Orient;
        }
    }
}

impl AI for PatrolAI {
    fn tick(&mut self, ship: &mut Ship, _: &SensorSweep) -> Option<Torpedo> {
        match self.mode {
            Mode::Orient => {
                self.orient(ship);
            }
            Mode::Accel => {
                self.accel(ship);
            }
            Mode::Glide => {
                self.glide(ship);
            }
            Mode::Disorient => {
                self.disorient(ship);
            }
            Mode::Deaccel => {
                self.deaccel(ship);
            }
        }
        None
    }
}

pub struct TurretAI { pub cooldown: usize }

impl AI for TurretAI {
    fn tick(&mut self, ship: &mut Ship, sensors: &SensorSweep) -> Option<Torpedo> {
        let heading = ship.position().orientation_to(sensors.heroine_position);
        let diff = heading - ship.orientation();
        if diff.0 > 0. {
            ship.reorient_right();
        } else {
            ship.reorient_left();
        }
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
        if diff.0.abs() < 0.1 && self.cooldown == 0 {
            self.cooldown = 50;
            Some(ship.summon_torpedo())
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub struct HunterAI;

impl AI for HunterAI {
    fn tick(&mut self, ship: &mut Ship, sensors: &SensorSweep) -> Option<Torpedo> {
        let heading = ship.position().orientation_to(sensors.heroine_position);
        let diff = heading - ship.orientation();
        if diff.0.abs() < 0.1 {
            ship.thrust();
        }
        if diff.0 > 0. {
            ship.reorient_right();
        } else {
            ship.reorient_left();
        }
        None
    }
}
