use bevy::prelude::*;

#[derive(Component)]
struct Blob;

impl Blob {
    fn step(&self, pos: Vec3) -> Vec3 {
        let delta = (rand::random::<f32>() - 0.5) / 100.;
        Vec3 {
            x: (pos.x + delta).clamp(-2., 2.),
            y: (pos.y + delta).clamp(-2., 2.),
            z: (pos.y + delta).clamp(-2., 2.),
        }
    }
}

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
        Transform::from_xyz(-9., 9., 9.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn step(
    mut query: Query<(&Blob, &mut Transform)>,
) {
    for (blob, mut transform) in &mut query {
        transform.translation = blob.step(transform.translation);
    }
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
