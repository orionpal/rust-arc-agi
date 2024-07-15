use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn start_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
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
}