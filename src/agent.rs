use crate::entity::{Entity, Position};
use crate::ship::Ship;

pub struct Agent {
    pub ship: Ship,
    pub ai: Box<AI>
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
        use super::log;
        let waypoint = self.waypoints[self.next];
        // log(&format!("{:?}", waypoint));
        if waypoint.distance_to(ship.position()) < 100. {
            self.next = (self.next + 1) % self.waypoints.len();
            return;
        }
        let _distance_to_waypoint = ship.position().distance_to(waypoint);
        let orientation_to_waypoint = ship.position().orientation_to(waypoint);
        // log(&format!("orientation to waypoint {:?}", orientation_to_waypoint));
        let orientation_diff = orientation_to_waypoint - ship.orientation();
        if orientation_diff.0 < 0. {
            ship.reorient_left();
        } else {
            ship.reorient_right();
        }
        // if we're pointed in the right direction, thrust
        // if orientation_diff.0 < 0.01 {
        //     ship.thrust();
        // }
    }
}
