use bevy::prelude::*;

pub struct CueBallMovementPlugin;

impl Plugin for CueBallMovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, movement);
    }