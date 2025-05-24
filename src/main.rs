mod engine;
mod fixed_aspect_ratio;

use bevy::{
    color::palettes::css::{BLACK, GREEN, GREY, RED, WHITE},
    ecs::spawn::SpawnIter,
    prelude::*,
};
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
        .add_event::<MyButtonEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (generate_button_events, tile_interaction).chain())
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
        MyButton::default(),
        BackgroundColor(WHITE.into()),
        // BorderColor(Color::BLACK),
    )
}

#[derive(Component, Default)]
#[require(Button)]
struct MyButton {
    last_interaction: Interaction,
}

/// Button events, representing transitions between `Interaction`s
#[derive(Event)]
struct MyButtonEvent {
    entity: Entity,
    action: MyButtonEventAction,
}

#[derive(Debug, Clone, Copy)]
enum MyButtonEventAction {
    /// Mouse started hovering this frame
    /// None -> Hovered
    Hovered,
    /// Mouse stopped hovering this frame
    /// Hovered -> None
    Unhovered,
    /// Mouse button pressed over the node this frame
    /// Hovered -> Pressed, None -> Pressed
    Pressed,
    /// Mouse button released this frame (not over the node)
    /// Pressed -> None
    Released,
    /// Node clicked this frame (mouse button was originally pressed
    /// over this node, and then was released over this node)
    /// Pressed -> Hovered
    Clicked,
}

fn generate_button_events(
    mut events: EventWriter<MyButtonEvent>,
    query: Query<(Entity, &Interaction, &mut MyButton), Changed<Interaction>>,
) {
    for (entity, interaction, mut button) in query {
        use Interaction::*;
        match (button.last_interaction, *interaction) {
            (None, Hovered) => Some(MyButtonEventAction::Hovered),
            (Hovered, None) => Some(MyButtonEventAction::Unhovered),
            (Hovered, Pressed) | (None, Pressed) => Some(MyButtonEventAction::Pressed),
            (Pressed, None) => Some(MyButtonEventAction::Released),
            (Pressed, Hovered) => Some(MyButtonEventAction::Clicked),
            (Pressed, Pressed) | (Hovered, Hovered) | (None, None) => Option::None,
        }
        .map(|action| events.write(MyButtonEvent { entity, action }));
        button.last_interaction = *interaction;
    }
}

fn tile_interaction(
    mut events: EventReader<MyButtonEvent>,
    mut query: Query<&mut BackgroundColor, With<Tile>>,
) {
    for event in events.read() {
        if let Ok(mut background_color) = query.get_mut(event.entity) {
            use MyButtonEventAction::*;
            match event.action {
                Hovered => {
                    background_color.0 = GREY.into();
                }
                Unhovered => {
                    background_color.0 = WHITE.into();
                }
                Pressed => {
                    background_color.0 = BLACK.into();
                }
                Released => {
                    background_color.0 = RED.into();
                }
                Clicked => {
                    background_color.0 = GREEN.into();
                }
            }
        }
    }
}
