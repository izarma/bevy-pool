#![allow(clippy::unnecessary_cast)]

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use engine::game_runner::GameRunnerPlugin;

mod engine;

const DARK_GREEN: Color = Color::srgb(0.1, 0.5, 0.1);
const BROWN: Color = Color::srgb(0.7, 0.4, 0.1);
const YELLOW: Color = Color::srgb(1.0, 0.92, 0.0);


fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins,
            GameRunnerPlugin,
            PhysicsPlugins::default().with_length_unit(20.0)
        ))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

#[derive(Component)]
struct CueBall;

#[derive(Component)]
struct RegularBall;
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2d);

    // Set up table dimensions
    // We'll define a table area approximately 2400x1200 units in world coordinates.
    let table_width = 2400.0;
    let table_height = 1200.0;
    let cushion_thickness = 50.0; // thickness of the rails

    let square_sprite = Sprite {
        color: BROWN,
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // Ceiling
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(0.0, 50.0 * 6.0, 0.0).with_scale(Vec3::new(20.0, 1.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
    // Floor
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(0.0, -50.0 * 6.0, 0.0).with_scale(Vec3::new(20.0, 1.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
    // Left wall
    commands.spawn((
        square_sprite.clone(),
        Transform::from_xyz(-50.0 * 9.5, 0.0, 0.0).with_scale(Vec3::new(1.0, 11.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
    // Right wall
    commands.spawn((
        square_sprite,
        Transform::from_xyz(50.0 * 9.5, 0.0, 0.0).with_scale(Vec3::new(1.0, 11.0, 1.0)),
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
        // Ball setup
        let ball_radius = 10.0;
        let ball_mesh = meshes.add(Circle::new(ball_radius));
        let cue_ball_material = materials.add(Color::WHITE);
        let balls_material = materials.add(YELLOW);
    
        // Cue ball placement: Place it near the left side of the table (the "head" side)
        let cue_ball_x = -250.0; // Move it to the left quarter of the table
        let cue_ball_y = 0.0;
        commands.spawn((
            Mesh2d(ball_mesh.clone()),
            MeshMaterial2d(cue_ball_material.clone()),
            Transform::from_xyz(cue_ball_x, cue_ball_y, 0.2),
            RigidBody::Dynamic,
            Collider::circle(ball_radius),
            CueBall,
        ));
    
        // Rack the balls at the opposite end (the "foot" side):
        // We'll place the apex ball at (table_width/4.0, 0.0), and form a triangle behind it.
        let rack_x = 250.0;
        let rack_y = 0.0;
        let spacing = 2.05 * ball_radius; // slightly more than diameter to avoid initial overlap
    
        // 15 balls in a triangular rack:
        // Row 1: 1 ball
        // Row 2: 2 balls
        // Row 3: 3 balls
        // Row 4: 4 balls
        // Row 5: 5 balls
        // The apex ball (row 1) is at rack_x, rack_y.
        // Subsequent rows move diagonally down-left and down-right.
    
        let mut ball_positions = Vec::new();
        let start_y = rack_y;
        let start_x = rack_x;
        let mut count = 0;
    
        for row in 1..=5 {
            let row_count = row;
            // Center the row vertically around rack_y:
            let row_start_x = start_x + (row as f32 - 1.0) * spacing;
            let row_start_y = start_y - ((row_count as f32 - 1.0) * spacing / 2.0);
            for i in 0..row_count {
                let x_pos = row_start_x;
                let y_pos = row_start_y + i as f32 * spacing; // same vertical line, actually we align horizontally
                ball_positions.push((x_pos, y_pos));
                count += 1;
                println!("Ball {} at ({}, {})", count, x_pos, y_pos);
            }
        }

    // Place the balls. The 8-ball should be at the center of the triangle: that's row 3, middle ball.
    // The indexing of the balls in ball_positions will be:
    // Row 1: index 0 (1 ball)
    // Row 2: indices 1..3 (2 balls)
    // Row 3: indices 3..6 (3 balls)
    // Row 4: indices 6..10 (4 balls)
    // Row 5: indices 10..15 (5 balls)
    // Middle of row 3 is index 4 (0-based: row 3 starts at 3, middle ball is 3 + 1 = 4)
    let eight_ball_index = 4;

    for (i, (x_pos, y_pos)) in ball_positions.iter().enumerate() {
        let material = if i == eight_ball_index {
            materials.add(Color::BLACK)
        } else {
            balls_material.clone()
        };
        commands.spawn((
            Mesh2d(ball_mesh.clone()),
            MeshMaterial2d(material),
            Transform::from_xyz(*x_pos, *y_pos, 0.2),
            RigidBody::Dynamic,
            Collider::circle(ball_radius),
            RegularBall,
        ));
    }
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut marbles: Query<&mut LinearVelocity, With<CueBall>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for mut linear_velocity in &mut marbles {
        if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            // Use a higher acceleration for upwards movement to overcome gravity
            linear_velocity.y += 2500.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            linear_velocity.y -= 500.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            linear_velocity.x -= 500.0 * delta_time;
        }
        if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            linear_velocity.x += 500.0 * delta_time;
        }
    }
}