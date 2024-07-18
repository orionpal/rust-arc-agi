use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};


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
    let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0)));
    let color = Color::rgb(0.0,55.0,0.0);
    commands.spawn(MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(color),
        transform: Transform::from_xyz(
            50.0,
            0.0,
            0.0,
        ),
        ..default()
    });
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
    setup_grid(commands);
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
fn button_system(
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

    // Spawn horizontal grid lines
    for y in 0..=GRID_HEIGHT {
        let position = Vec3::new(
            0.0,
            (y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE,
            1.0,
        );

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(GRID_WIDTH as f32 * CELL_SIZE, LINE_THICKNESS)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            GridLine,
        ));
    }

    // Spawn vertical grid lines
    for x in 0..=GRID_WIDTH {
        let position = Vec3::new(
            (x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE,
            0.0,
            1.0,
        );

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(LINE_THICKNESS, GRID_HEIGHT as f32 * CELL_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            GridLine,
        ));
    }
}