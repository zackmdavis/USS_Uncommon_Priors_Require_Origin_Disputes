use crate::entity::{Entity, Position, Spin};
use crate::log;
use crate::ship::Ship;

const CRUISING_SPEED: f32 = 1.;

pub struct Agent {
    pub ship: Ship,
    pub ai: Box<dyn AI>,
}

pub struct SensorSweep {
    pub heroine_position: Position
}

pub trait AI {
    fn tick(&mut self, ship: &mut Ship, sensors: &SensorSweep);
}

#[derive(Debug)]
enum Mode {
    Orient,
    Accel,
    Glide,
    Disorient,
    Deaccel,
}

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
        let heading = ship.position().orientation_to(self.waypoints[self.next]);
        let diff: Spin = heading - ship.orientation();
        log(&format!("heading is {:?}", heading));
        log(&format!("orientation is {:?}", ship.orientation()));
        log(&format!("diff is {:?}", diff));
        if diff.0.abs() < 0.1 {
            log("switching to Accel mode");
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
            log("switching to Glide mode");
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
        let slowdown_distance = self.slowdown_distance(ship);
        log(&format!("slowdown distance is {:?}", slowdown_distance));
        if ship.position().distance_to(self.waypoints[self.next]) < slowdown_distance {
            log("switching to Disorient mode");
            self.mode = Mode::Disorient;
        }
    }

    pub fn disorient(&mut self, ship: &mut Ship) {
        let heading = ship.velocity().countering_orientation();
        let diff: Spin = heading - ship.orientation();
        if diff.0.abs() <= 0.05 {
            log("switching to Deaccel mode");
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
            log("switching to Orient mode for next waypoint!");
            self.next = (self.next + 1) % self.waypoints.len();
            self.mode = Mode::Orient;
        }
    }
}

impl AI for PatrolAI {
    fn tick(&mut self, ship: &mut Ship, _: &SensorSweep) {
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
    }
}

pub struct HunterAI;


impl AI for HunterAI {
    fn tick(&mut self, ship: &mut Ship, sensors: &SensorSweep) {
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
    }
}
