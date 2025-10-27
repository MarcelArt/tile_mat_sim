use bevy::prelude::*;

use crate::core::{components::{Name, Person}, resources};

#[allow(dead_code)]
pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[allow(dead_code)]
pub fn hello_world() {
    println!("Hello from systems!");
}

#[allow(dead_code)]
pub fn greet_people(query: Query<&Name, With<Person>>, time: Res<Time>, mut timer: ResMut<resources::GreetTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

#[allow(dead_code)]
pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}