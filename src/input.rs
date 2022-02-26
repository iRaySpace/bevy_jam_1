use bevy::prelude::*;

pub struct InputPlugin;

fn input(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        info!("up");
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        info!("down");
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        info!("left");
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        info!("right");
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input);
    }
}
