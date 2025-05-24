mod engine;
mod fixed_aspect_ratio;

use bevy::{ecs::spawn::SpawnIter, prelude::*};
use fixed_aspect_ratio::{FixedAspectRatio, FixedAspectRatioPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FixedAspectRatioPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
            //     viewport_height: 10.,
            // },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        children![board(6)],
    ));
    // commands.spawn(board(6));
}

fn board(size: u16) -> impl Bundle {
    (
        Name::new("Board"),
        Node {
            display: Display::Grid,
            padding: UiRect::all(Val::Px(5.)),
            aspect_ratio: Some(1.),
            grid_template_columns: vec![RepeatedGridTrack::flex(size, 1.)],
            grid_template_rows: vec![RepeatedGridTrack::flex(size, 1.)],
            row_gap: Val::Px(5.),
            column_gap: Val::Px(5.),
            ..default()
        },
        FixedAspectRatio,
        Children::spawn(SpawnIter((0..size * size).map(|i| tile()))),
    )
}

#[derive(Component)]
struct Tile;

fn tile() -> impl Bundle {
    (
        Name::new("Tile"),
        Tile,
        Node {
            // border: UiRect::all(Val::Px(5.)),
            ..default()
        },
        Button,
        BackgroundColor(Color::WHITE),
        // BorderColor(Color::BLACK),
    )
}
