use bevy::prelude::*;

use crate::core::{resources, systems};

#[allow(dead_code)]
pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, systems::add_people);
        app.add_systems(Update, (systems::hello_world, (systems::update_people, systems::greet_people).chain()));
    }
}