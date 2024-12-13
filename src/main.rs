#![allow(clippy::unnecessary_cast)]

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use engine::game_runner::GameRunnerPlugin;

mod engine;



fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins,
             GameRunnerPlugin
        ))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .run();
}