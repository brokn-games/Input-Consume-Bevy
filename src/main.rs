use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(keyboard_input_system.system())
        .add_system(command_reader.system())
        .run();
}

struct Megaman;

#[derive(Debug)]
struct State(MegamanState);

#[derive(Debug)]
enum MegamanState {
    Walking,
    Idle,
    Jumping,
}

fn keyboard_input_system(commands: &mut Commands, keyboard_input: Res<Input<KeyCode>>) {
    commands.spawn((Megaman, State(MegamanState::Idle)));

    if keyboard_input.pressed(KeyCode::Right) {
        commands.spawn((Megaman, State(MegamanState::Walking)));
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        commands.spawn((Megaman, State(MegamanState::Jumping)));
    }

    if (keyboard_input.pressed(KeyCode::RWin) || keyboard_input.pressed(KeyCode::LWin))
        && keyboard_input.just_pressed(KeyCode::Q)
    {
        std::process::exit(0);
    }
}

fn command_reader(query: Query<&State, With<Megaman>>) {
    for state in query.iter() {
        println!("Megaman states is: {:#?}", state);
    }
}
