use bevy::prelude::*;

pub struct AnimationPlugin;

#[derive(Debug)]
pub struct AnimationValue {
    pub frames: Vec<u8>,
}

#[derive(Component, Debug)]
pub struct Animation {
    pub values: Vec<AnimationValue>,
    pub current_value: u8,
    pub current_frame: u8,
    pub duration: Timer,
}

fn animate_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Animation,
        &mut TextureAtlasSprite
    )>,
) {
    for (mut anim, mut sprite) in query.iter_mut() {
        anim.duration.tick(time.delta());
        if anim.duration.just_finished() {
            let anim_value = anim.values.get(anim.current_value as usize).unwrap();
            let current_frame = (anim.current_frame + 1) % (anim_value.frames.len() as u8);
            let anim_frame = anim_value.frames.get(current_frame as usize);
            match anim_frame {
                None => {}
                Some(val) => {
                    sprite.index = *val as usize;
                    anim.current_frame = current_frame;
                }
            }
        }
    }
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_system);
    }
}
