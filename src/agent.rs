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


pub struct PatrolAI {
    waypoints: Vec<Position>,
    next: usize,
}

impl PatrolAI {
    pub fn new(waypoints: Vec<Position>) -> Self {
        PatrolAI { waypoints, next: 0 }
    }
}

impl AI for PatrolAI {
    fn tick(&mut self, ship: &mut Ship) {
        let waypoint = self.waypoints[self.next];
        if waypoint.distance_to(ship.position()) < 100. {
            self.next = (self.next + 1) % self.waypoints.len();
            return;
        }
        let _distance_to_waypoint = ship.position().distance_to(waypoint);
        let orientation_to_waypoint = ship.position().orientation_to(waypoint);

        if ship.velocity().abs() < 1. {
            ship.orientation = orientation_to_waypoint;
            ship.thrust();
        } else {
            // slow down
            ship.orientation = ship.velocity().countering_orientation();
            ship.thrust();
        }

    }
}
