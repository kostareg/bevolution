use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Blob;

impl Blob {
    fn step(&self, force: Vec3) -> Vec3 {
        let delta = rand::random::<f32>() - 0.5;
        Vec3 {
            x: (force.x + delta) % 0.01,
            y: (force.y + delta) % 0.01,
            z: (force.z + delta) % 0.01,
        }
    }
}

fn spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hx = 3.;
    let hy = 3.;
    let hz = 3.;

    let verts = vec![
        Vec3::new(-hx, -hy, -hz),
        Vec3::new( hx, -hy, -hz),
        Vec3::new( hx,  hy, -hz),
        Vec3::new(-hx,  hy, -hz),
        Vec3::new(-hx, -hy,  hz),
        Vec3::new( hx, -hy,  hz),
        Vec3::new( hx,  hy,  hz),
        Vec3::new(-hx,  hy,  hz),
    ];

    let indices = vec![
        [0,1,2], [0,2,3],
        [4,6,5], [4,7,6],
        [0,4,5], [0,5,1],
        [3,2,6], [3,6,7],
        [1,5,6], [1,6,2],
        [0,3,7], [0,7,4],
    ];

    commands.spawn((
        RigidBody::Fixed,
        Collider::trimesh(verts, indices).unwrap(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(3.)))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 144, 255))),
        Transform::from_xyz(0., -3., 0.),
    ));

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

fn spawn_blobs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let max_x = 25;
    let max_z = 25;

    for i in 0..(max_x * max_z * 5) {
        commands.spawn((
            Blob,
            Collider::cuboid(0.06, 0.06, 0.06),
            RigidBody::Dynamic,
            GravityScale(0.),
            ExternalForce { force: Vec3::ZERO, torque: Vec3::ZERO },
            Restitution::coefficient(0.7),
            Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz((i % max_x) as f32 / 5. - 2.5, ((i / (max_x * max_z)) as f32) / 5. - 2.5, (((i / max_x) as f32) % (max_z as f32)) / 5. - 2.5),
        ));
    }
}

fn step(
    mut query: Query<(&Blob, &mut ExternalForce)>
) {
    for (blob, mut ext_force) in query.iter_mut() {
        ext_force.force = blob.step(ext_force.force);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (spawn_environment, spawn_blobs))
        .add_systems(FixedUpdate, step)
        .run();
}
