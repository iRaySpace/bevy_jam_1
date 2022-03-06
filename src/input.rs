use crate::animation::Animation;
use crate::player::Player;
use crate::AppState;
use bevy::prelude::*;

pub struct InputPlugin;

fn input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Animation, &Player)>) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        let (mut anim, _) = query.get_single_mut().unwrap();
        anim.current_value = 1;
        anim.current_frame = 0;
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        let (mut anim, _) = query.get_single_mut().unwrap();
        anim.current_value = 0;
        anim.current_frame = 0;
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        let (mut anim, _) = query.get_single_mut().unwrap();
        anim.current_value = 2;
        anim.current_frame = 0;
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        let (mut anim, _) = query.get_single_mut().unwrap();
        anim.current_value = 3;
        anim.current_frame = 0;
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(input));
    }
}
