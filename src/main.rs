use bevy::prelude::*;

#[derive(Component)]
struct Blob;

fn spawn_blobs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..100 {
        commands.spawn((
            Blob,
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(i as f32 * 1.1, 0., 0.),
        ));
    }

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn step(query: Query<&mut Blob>) {
}

fn render() {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_blobs)
        .add_systems(FixedUpdate, step)
        .add_systems(RunFixedMainLoop, (render.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop)))
        .run();
}
