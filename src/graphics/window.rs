use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::graphics::colors::cell_color;
use crate::reader::{get_random_file_from_directory, load_puzzle};

const CELL_SIZE: f32 = 50.0;
const LINE_THICKNESS: f32 = 2.0;

#[derive(Component)]
struct Cell;
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
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexStart,
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
                        "Random Puzzle",
                        TextStyle {
                            font: Default::default(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
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
                let random_file = get_random_file_from_directory(&training_path).unwrap();
                let puzzle = load_puzzle(&random_file);
                for train in &puzzle.train {
                    // Input
                    for y in 0..train.input.height {
                        for x in 0..train.input.width {
                            let value = train.input.get(x, y);
                            let color = cell_color(value);

                            commands.spawn(SpriteBundle {
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
                    // Output
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "LOAD".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Random Puzzle".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}