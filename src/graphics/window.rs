use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::reader::load_puzzle;


const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 8;
const CELL_SIZE: f32 = 50.0;
const LINE_THICKNESS: f32 = 2.0;

#[derive(Component)]
struct Cell;

#[derive(Component)]
struct GridLine;
pub fn start_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}
#[derive(Component)]
struct MyCameraMarker;

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Camera2dBundle::default(),
        MyCameraMarker,
    ));
    // Create Button
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),

                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: Default::default(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
    // Create Grid
    setup_grid(commands);
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::rgb(255.0,0.0,0.0);
                let training_path = "training/";
                let evaluation_path = "evaluation/";
                let grid = load_puzzle(get_random_file_from_directory(&training_path));
                for (y, row) in grid.values.iter().enumerate() {
                    for (x, &value) in row.iter().enumerate() {
                        let color = match value {
                            0 => Color::rgb(0.0, 0.0, 0.0), // Black
                            1 => Color::rgb(0.0, 1.0, 0.0), // Green
                            2 => Color::rgb(0.0, 0.0, 1.0), // Blue
                            3 => Color::rgb(1.0, 1.0, 0.0), // Yellow
                            4 => Color::rgb(1.0, 0.0, 1.0), // Magenta
                            5 => Color::rgb(0.0, 1.0, 1.0), // Cyan
                            6 => Color::rgb(0.3, 0.3, 0.3), // Gray
                            7 => Color::rgb(1.0, 0.0, 0.0), // Red
                            8 => Color::rgb(0.7, 0.7, 0.7), // Light Gray
                            9 => Color::rgb(1.0, 1.0, 1.0), // White
                            _ => Color::rgb(0.0, 0.0, 0.0), // Default Black
                        };

                        commands.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color,
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: Vec3::new(x as f32 * 20.0, y as f32 * 20.0, 0.0),
                                scale: Vec3::splat(20.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    }
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup_grid(mut commands: Commands){
    // Spawn cells
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let position = Vec3::new(
                (x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE,
                (y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE,
                0.0,
            );

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(
                            (x as f32) / (GRID_WIDTH as f32),
                            (y as f32) / (GRID_HEIGHT as f32),
                            0.5,
                        ),
                        custom_size: Some(Vec2::splat(CELL_SIZE - LINE_THICKNESS)),
                        ..default()
                    },
                    transform: Transform::from_translation(position),
                    ..default()
                },
                Cell,
            ));
        }
    }

}