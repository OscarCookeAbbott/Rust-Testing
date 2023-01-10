mod vector;

use bevy::prelude::*;
use vector::*;

#[derive(Component)]
struct Position(Vector<f64>);

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(hello_world)
        .add_startup_system(spawn_entities)
        .add_system(move_entities)
        .add_system(print_entity_positions)
        .run();
}

fn hello_world() {
    println!("hello world");
}

fn spawn_entities(mut commands: Commands) {
    commands.spawn((
        Name("Testicles".to_string()),
        Position(vector!(1.0, 2.0, 3.0)),
    ));
    commands.spawn((
        Name("Giblets".to_string()),
        Position(vector!(1.0, 2.0, 3.0)),
    ));
    commands.spawn((
        Name("Rockets".to_string()),
        Position(vector!(1.0, 2.0, 3.0)),
    ));
}

fn move_entities(mut query: Query<&mut Position>) {
    for mut position in &mut query {
        position.0 += vector!(1.0, 2.0, 3.0);
    }
}

fn print_entity_positions(query: Query<(&Name, &Position)>) {
    for (name, position) in query.iter() {
        println!(
            "{0}: {1}, {2}, {3}",
            name.0, position.0.x, position.0.y, position.0.z
        );
    }
}
