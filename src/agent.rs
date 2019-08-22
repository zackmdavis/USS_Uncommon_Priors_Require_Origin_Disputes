use crate::log;
use crate::entity::{Entity, Position};
use crate::ship::Ship;

pub struct Agent {
    pub ship: Ship,
    pub ai: Box<dyn AI>
}

pub trait AI {
    // TODO: provide observations?!
    fn tick(&mut self, ship: &mut Ship);
}

#[derive(Debug)]
enum Mode {
    Stopping,
    Seeking,
}

pub struct PatrolAI {
    waypoints: Vec<Position>,
    next: usize,
    mode: Mode,
}

impl PatrolAI {
    pub fn new(waypoints: Vec<Position>) -> Self {
        PatrolAI { waypoints, next: 0, mode: Mode::Stopping }
    }
}

impl AI for PatrolAI {

    fn tick(&mut self, ship: &mut Ship) {
        log(&format!("{:?}", self.mode));
        match self.mode {
            Mode::Stopping => {
                let desired_orientation = ship.velocity().countering_orientation();
                let orientation_diff = desired_orientation - ship.orientation();
                if orientation_diff.0 > 0. {
                    ship.reorient_left();
                } else {
                    ship.reorient_right();
                }

                if orientation_diff.0.abs() < 0.1 {
                    ship.thrust()
                }

                if ship.velocity().abs() < 0.3 {
                    self.next = (self.next + 1) % self.waypoints.len();
                    log(&format!("heading to next waypoint {:?}", self.waypoints[self.next]));
                    self.mode = Mode::Seeking;
                }
            }
            Mode::Seeking => {
                let waypoint = self.waypoints[self.next];
                if waypoint.distance_to(ship.position()) < 20. {
                    self.mode = Mode::Stopping;
                }

                let desired_orientation = ship.position().orientation_to(waypoint);
                let orientation_diff = ship.orientation() - desired_orientation;
                // log(&format!("{:?}", orientation_diff));
                if orientation_diff.0 > 0. {
                    ship.reorient_left();
                } else {
                    ship.reorient_right();
                }

                if orientation_diff.0.abs() < 0.2 && ship.velocity().abs() < 1. {
                    ship.thrust()
                }
            }
        }
    }

}
