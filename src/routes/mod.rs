mod docker;
mod misc;

use rocket::{Rocket, Route};

pub trait RouteSet {
    fn base(&self) -> String;
    fn routes(&self) -> Vec<Route>;
    fn apply_to(&self, rocket: Rocket) -> Rocket {
        rocket.mount(&self.base(), self.routes())
    }
}

pub fn apply_all_sets(r: Rocket) -> Rocket {
    let sets: Vec<Box<dyn RouteSet>> =
        vec![Box::new(docker::DockerRoutes), Box::new(misc::MiscRoutes)];
    let mut rocket = r;
    for set in sets {
        rocket = set.apply_to(rocket);
    }

    rocket
}
