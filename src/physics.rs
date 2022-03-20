use bevy::prelude::*;
use heron::prelude::*;

#[derive(PhysicsLayer)]
pub enum Layer {
    Player,
    Consumable,
    Animal,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(heron::prelude::PhysicsPlugin::default())
            .insert_resource(Gravity::from(Vec3::new(0., 0., 0.)));
    }
}
